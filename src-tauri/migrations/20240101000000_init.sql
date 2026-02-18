-- Create documents table for RAG
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    hash TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create audit_logs table for Immutable Logging
CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY NOT NULL,
    prev_hash TEXT NOT NULL,
    curr_hash TEXT NOT NULL,
    action TEXT NOT NULL,
    user TEXT NOT NULL,
    metadata TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster path lookups
CREATE INDEX IF NOT EXISTS idx_documents_path ON documents(path);
