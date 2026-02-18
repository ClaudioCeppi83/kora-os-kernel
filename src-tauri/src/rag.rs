use sha2::{Digest, Sha256};
use sqlx::{Pool, Sqlite};
use std::fs::File;
use std::path::Path;
use uuid::Uuid;
use memmap2::Mmap;

const CHUNK_SIZE: usize = 50 * 1024; // 50KB
const OVERLAP_SIZE: usize = 5 * 1024; // 10% overlap (5KB)

pub async fn index_file(pool: &Pool<Sqlite>, file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err("File not found".to_string());
    }

    // 1. Open File and Mmap (Zero-Copy)
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mmap = unsafe { Mmap::map(&file).map_err(|e| e.to_string())? };
    let content_len = mmap.len();

    // 2. Compute Hash (Streaming/Slice based)
    let mut hasher = Sha256::new();
    hasher.update(&mmap);
    let hash = format!("{:x}", hasher.finalize());

    // 3. Check for Changes
    let existing: Option<(String,)> = sqlx::query_as("SELECT hash FROM documents WHERE path = ? LIMIT 1")
        .bind(file_path)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    if let Some((existing_hash,)) = existing {
        if existing_hash == hash {
            return Ok("File already indexed and unchanged".to_string());
        }
    }

    // 4. Atomic Update
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    
    sqlx::query("DELETE FROM documents WHERE path = ?")
        .bind(file_path)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // 5. Chunking Metadata (Zero-Copy)
    let mut start = 0;
    while start < content_len {
        let end = std::cmp::min(start + CHUNK_SIZE, content_len);
        
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO documents (id, path, hash, content, offset_start, offset_end) VALUES (?, ?, ?, '', ?, ?)"
        )
        .bind(id)
        .bind(file_path)
        .bind(&hash)
        .bind(start as i64)
        .bind(end as i64)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        start += CHUNK_SIZE - OVERLAP_SIZE;
        if start >= content_len {
            break;
        }
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(format!("Indexed {} bytes via Mmap. Hash: {}", content_len, hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_boundaries() {
        let content_len = 200 * 1024; // 200KB
        let mut start = 0;
        let mut count = 0;
        
        while start < content_len {
            let end = std::cmp::min(start + CHUNK_SIZE, content_len);
            assert!(end <= content_len);
            assert!(end > start);
            count += 1;
            start += CHUNK_SIZE - OVERLAP_SIZE;
            if start >= content_len { break; }
        }
        
        // (200 / 45) ~ 4.4 -> 5 chunks approx (actually 50KB size, 5KB overlap -> 45KB step)
        // 0-50, 45-95, 90-140, 135-185, 180-200
        assert_eq!(count, 5);
    }
}
