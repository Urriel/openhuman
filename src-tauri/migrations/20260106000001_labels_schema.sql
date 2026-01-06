-- Labels and Message Labels Schema
-- Adds support for Gmail-style labels with many-to-many relationships

-- Labels table for tag-based organization
CREATE TABLE IF NOT EXISTS labels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    color TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
    UNIQUE(account_id, name)
);

-- Junction table for many-to-many message-label relationship
CREATE TABLE IF NOT EXISTS message_labels (
    message_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (message_id, label_id),
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE,
    FOREIGN KEY (label_id) REFERENCES labels(id) ON DELETE CASCADE
);

-- Indexes for performance

-- Labels: account lookups
CREATE INDEX IF NOT EXISTS idx_labels_account ON labels(account_id);

-- Message Labels: fast message-to-label lookups
CREATE INDEX IF NOT EXISTS idx_message_labels_message ON message_labels(message_id);

-- Message Labels: fast label-to-message lookups
CREATE INDEX IF NOT EXISTS idx_message_labels_label ON message_labels(label_id);
