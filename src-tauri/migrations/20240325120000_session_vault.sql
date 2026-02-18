CREATE TABLE IF NOT EXISTS kora_session_vault (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    prompt TEXT NOT NULL,
    response_hash TEXT NOT NULL,
    context_snapshot TEXT NOT NULL
);
