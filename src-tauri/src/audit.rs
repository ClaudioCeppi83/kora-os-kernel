use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_EMAIL: Regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    static ref RE_IP: Regex = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap();
    static ref RE_PHONE: Regex = Regex::new(r"\+?[\d\s-]{10,15}").unwrap();
}

/// Represents an immutable audit log entry in the security chain.
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct AuditLog {
    /// Unique identifier for the log entry.
    pub id: String,
    /// SHA-256 hash of the previous log entry in the chain.
    pub prev_hash: String,
    /// SHA-256 hash representing the current state of this entry and the chain.
    pub curr_hash: String,
    /// The system or user action performed.
    pub action: String,
    /// The security ring or user level associated with the action.
    pub user: String,
    /// Redacted metadata or event details.
    pub metadata: String,
    /// ISO-8601 timestamp of the event.
    pub timestamp: String,
    /// The agency context in which the event occurred.
    pub agency_id: String,
}

/// Scrub PII data from a string
#[allow(dead_code)] // Added #[allow(dead_code)] to scrub_pii as it's the only method that fits the context of the snippet's placement.
pub fn scrub_pii(text: &str) -> String {
    let mut scrubbed = text.to_string();
    scrubbed = RE_EMAIL.replace_all(&scrubbed, "[REDACTED_PII]").to_string();
    scrubbed = RE_IP.replace_all(&scrubbed, "[REDACTED_PII]").to_string();
    scrubbed = RE_PHONE.replace_all(&scrubbed, "[REDACTED_PII]").to_string();
    scrubbed
}

/// Records a security event in the immutable audit log chain.
///
/// Automatically scrubs PII from metadata and computes the next SHA-256 hash link.
pub async fn log_event(
    pool: &Pool<Sqlite>,
    action: &str,
    user: &str,
    metadata: &str,
    agency_id: &str,
) -> Result<String, String> {
    // 1. Scrub PII
    let clean_metadata = scrub_pii(metadata);
    let original_metadata = metadata.to_string();

    // 2. Fetch Last Hash
    let last_log: Option<(String,)> = sqlx::query_as(
        "SELECT curr_hash FROM audit_logs ORDER BY timestamp DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    let prev_hash = last_log.map(|l| l.0).unwrap_or_else(|| "0".to_string());
    let timestamp = Utc::now().to_rfc3339();

    // 3. Compute New Hash (SHA-256 Chain)
    // Hash(N) = HASH(EventData + Hash(N-1))
    let payload = format!("{}{}{}{}{}{}", timestamp, action, user, clean_metadata, agency_id, prev_hash);
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    let curr_hash = format!("{:x}", hasher.finalize());

    // 4. Insert Record (Redacted)
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO audit_logs (id, prev_hash, curr_hash, action, user, metadata, timestamp, agency_id) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&prev_hash)
    .bind(&curr_hash)
    .bind(action)
    .bind(user)
    .bind(&clean_metadata)
    .bind(&timestamp)
    .bind(agency_id)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    // 5. Store Shadow Metadata (Encrypted/Hidden) if PII was redacted
    if clean_metadata != original_metadata {
        // In a real scenario, encrypt original_metadata here
        let _ = sqlx::query(
            "INSERT INTO shadow_metadata (log_id, encrypted_data) VALUES (?, ?)"
        )
        .bind(&id)
        .bind(&original_metadata) // Placeholder for actual encryption
        .execute(pool)
        .await;
    }

    Ok(curr_hash)
}

/// Validates the integrity of the entire audit log chain.
///
/// Iteratively recomputes hashes and verifies that each entry correctly links to its predecessor.
/// Returns the latest block hash if valid, or None if a breach is detected.
pub async fn validate_chain(pool: &Pool<Sqlite>) -> Result<Option<String>, String> {
    let logs: Vec<AuditLog> = sqlx::query_as::<_, AuditLog>(
        "SELECT id, prev_hash, curr_hash, action, user, metadata, timestamp, agency_id FROM audit_logs ORDER BY timestamp ASC, id ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    if logs.is_empty() {
        return Ok(Some("0".to_string()));
    }

    let mut expected_prev = "0".to_string();
    let mut last_hash = String::new();

    for log in logs.iter() {
        if log.prev_hash != expected_prev {
            return Ok(None);
        }

        let payload = format!("{}{}{}{}{}{}", log.timestamp, log.action, log.user, log.metadata, log.agency_id, log.prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let calculated = format!("{:x}", hasher.finalize());

        if calculated != log.curr_hash {
            return Ok(None);
        }

        expected_prev = log.curr_hash.clone();
        last_hash = log.curr_hash.clone();
    }

    Ok(Some(last_hash))
}

pub async fn get_logs(pool: &Pool<Sqlite>, limit: i64) -> Result<Vec<AuditLog>, String> {
    sqlx::query_as::<_, AuditLog>(
        "SELECT id, prev_hash, curr_hash, action, user, metadata, timestamp, agency_id FROM audit_logs ORDER BY timestamp DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e: sqlx::Error| e.to_string())
}
