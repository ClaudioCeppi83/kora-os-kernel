-- Add shadow_metadata table for encrypted PII storage
CREATE TABLE IF NOT EXISTS shadow_metadata (
    log_id TEXT PRIMARY KEY,
    encrypted_data TEXT NOT NULL,
    FOREIGN KEY (log_id) REFERENCES audit_logs(id)
);
