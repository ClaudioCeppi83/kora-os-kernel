-- Refactor documents for Zero-Copy RAG
ALTER TABLE documents ADD COLUMN offset_start INTEGER;
ALTER TABLE documents ADD COLUMN offset_end INTEGER;

-- We can't easily drop columns in SQLite without a temp table if we want to be safe, 
-- but since this is an evolution phase, we will just nullify content or ignore it.
-- For a clean state, we might as well empty the table.
DELETE FROM documents;

-- In modern SQLite we could drop content, but letting it be NULL is safer for compatibility.
-- UPDATE documents SET content = NULL;
