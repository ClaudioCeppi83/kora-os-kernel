-- Create agencies table
CREATE TABLE IF NOT EXISTS agencies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    is_active BOOLEAN DEFAULT 0
);

-- Seed default SYSTEM agency
INSERT OR IGNORE INTO agencies (id, name, created_at, is_active)
VALUES ('SYSTEM', 'System Default', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), 1);

-- Alter documents (formerly referenced as knowledge_index in error)
ALTER TABLE documents ADD COLUMN agency_id TEXT DEFAULT 'SYSTEM' REFERENCES agencies(id);

-- Alter kora_session_vault (formerly referenced as sessions in error)
ALTER TABLE kora_session_vault ADD COLUMN agency_id TEXT DEFAULT 'SYSTEM' REFERENCES agencies(id);

-- Alter audit_logs (NOT NULL DEFAULT 'SYSTEM' as requested)
ALTER TABLE audit_logs ADD COLUMN agency_id TEXT NOT NULL DEFAULT 'SYSTEM';
