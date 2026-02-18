use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};
use std::fs;
use tauri::AppHandle;
use tauri::Manager;

pub async fn init_db(app: &AppHandle) -> Result<Pool<Sqlite>, sqlx::Error> {
    // 1. Resolve path relative to app data dir
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
    
    let db_path = app_data_dir.join("kora_kernel.db");
    println!(" [KORA] Database Path: {:?}", db_path);
    let _db_url = format!("sqlite://{}", db_path.to_string_lossy());

    // 2. Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::new()
                .filename(&db_path)
                .create_if_missing(true)
                .foreign_keys(true)
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        )
        .await?;

    // 3. Run Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct SessionSnapshot {
    pub id: i64,
    pub timestamp: String,
    pub prompt: String,
    pub response_hash: String,
    pub context_snapshot: String,
    pub agency_id: String,
}

pub async fn save_session_snapshot(pool: &Pool<Sqlite>, agency_id: &str, prompt: &str, response_hash: &str, context: &str) -> Result<i64, sqlx::Error> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let id = sqlx::query(
        "INSERT INTO kora_session_vault (timestamp, prompt, response_hash, context_snapshot, agency_id) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(timestamp)
    .bind(prompt)
    .bind(response_hash)
    .bind(context)
    .bind(agency_id)
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(id)
}

#[allow(dead_code)]
pub async fn restore_last_session(pool: &Pool<Sqlite>, agency_id: &str) -> Result<Option<SessionSnapshot>, sqlx::Error> {
    sqlx::query_as::<_, SessionSnapshot>(
        "SELECT * FROM kora_session_vault WHERE agency_id = ? ORDER BY id DESC LIMIT 1"
    )
    .bind(agency_id)
    .fetch_optional(pool)
    .await
}
