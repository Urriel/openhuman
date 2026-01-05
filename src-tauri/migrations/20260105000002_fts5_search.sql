-- Full-text search setup using SQLite FTS5
-- Creates virtual table for fast message content search

-- Create FTS5 virtual table for message content search
CREATE VIRTUAL TABLE IF NOT EXISTS message_fts USING fts5(
    message_id UNINDEXED,
    subject,
    body_plain,
    body_html,
    content='messages',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Trigger to keep FTS5 table in sync when inserting messages
CREATE TRIGGER IF NOT EXISTS message_fts_insert AFTER INSERT ON messages BEGIN
    INSERT INTO message_fts(rowid, message_id, subject, body_plain, body_html)
    VALUES (NEW.id, NEW.message_id, NEW.subject, NEW.body_plain, NEW.body_html);
END;

-- Trigger to keep FTS5 table in sync when updating messages
CREATE TRIGGER IF NOT EXISTS message_fts_update AFTER UPDATE ON messages BEGIN
    UPDATE message_fts
    SET message_id = NEW.message_id,
        subject = NEW.subject,
        body_plain = NEW.body_plain,
        body_html = NEW.body_html
    WHERE rowid = NEW.id;
END;

-- Trigger to keep FTS5 table in sync when deleting messages
CREATE TRIGGER IF NOT EXISTS message_fts_delete AFTER DELETE ON messages BEGIN
    DELETE FROM message_fts WHERE rowid = OLD.id;
END;
