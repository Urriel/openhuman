-- Initial schema for OpenHuman email client
-- Creates all core tables for account management, email storage, threading, and sync state

-- Email accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    provider TEXT NOT NULL,
    imap_host TEXT NOT NULL,
    imap_port INTEGER NOT NULL DEFAULT 993,
    smtp_host TEXT NOT NULL,
    smtp_port INTEGER NOT NULL DEFAULT 465,
    sync_enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Email threads table (conversation grouping)
CREATE TABLE IF NOT EXISTS threads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    thread_subject TEXT,
    participant_count INTEGER NOT NULL DEFAULT 0,
    latest_message_date TEXT,
    unread_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Email messages table
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    message_id TEXT NOT NULL,
    thread_id INTEGER,
    folder TEXT NOT NULL DEFAULT 'INBOX',
    subject TEXT,
    from_addr TEXT NOT NULL,
    to_addr TEXT,
    cc_addr TEXT,
    bcc_addr TEXT,
    date TEXT,
    is_read INTEGER NOT NULL DEFAULT 0,
    is_starred INTEGER NOT NULL DEFAULT 0,
    body_plain TEXT,
    body_html TEXT,
    references_header TEXT,
    in_reply_to TEXT,
    imap_uid INTEGER,
    imap_flags TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
    FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE SET NULL,
    UNIQUE(account_id, message_id)
);

-- Attachments table
CREATE TABLE IF NOT EXISTS attachments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message_id INTEGER NOT NULL,
    filename TEXT NOT NULL,
    size INTEGER NOT NULL DEFAULT 0,
    mime_type TEXT,
    file_path TEXT,
    is_inline INTEGER NOT NULL DEFAULT 0,
    content_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
);

-- Outbox table for queued outgoing emails
CREATE TABLE IF NOT EXISTS outbox (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    recipients TEXT NOT NULL,
    subject TEXT,
    body_plain TEXT,
    body_html TEXT,
    send_status TEXT NOT NULL DEFAULT 'pending',
    retry_count INTEGER NOT NULL DEFAULT 0,
    next_retry_at TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

-- Sync state table (tracks IMAP sync progress per account)
CREATE TABLE IF NOT EXISTS sync_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL UNIQUE,
    last_sync_at TEXT,
    uid_mappings TEXT,
    uid_validity INTEGER,
    uid_next INTEGER,
    messages_fetched INTEGER NOT NULL DEFAULT 0,
    sync_status TEXT NOT NULL DEFAULT 'idle',
    error_message TEXT,
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

-- Folders table for label/folder management
CREATE TABLE IF NOT EXISTS folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    message_count INTEGER NOT NULL DEFAULT 0,
    folder_type TEXT,
    selectable INTEGER DEFAULT 1,
    flags TEXT,
    uidvalidity INTEGER,
    uidnext INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
    UNIQUE(account_id, name)
);

-- Indexes for performance

-- Messages: inbox queries sorted by date
CREATE INDEX IF NOT EXISTS idx_messages_account_date ON messages(account_id, date DESC);

-- Messages: thread grouping
CREATE INDEX IF NOT EXISTS idx_messages_thread ON messages(thread_id);

-- Messages: threading lookups by message_id
CREATE INDEX IF NOT EXISTS idx_messages_message_id ON messages(message_id);

-- Messages: folder filtering
CREATE INDEX IF NOT EXISTS idx_messages_folder ON messages(folder);

-- Messages: read/starred filtering
CREATE INDEX IF NOT EXISTS idx_messages_read ON messages(is_read);
CREATE INDEX IF NOT EXISTS idx_messages_starred ON messages(is_starred);

-- Messages: IMAP UID lookups
CREATE INDEX IF NOT EXISTS idx_messages_imap_uid ON messages(account_id, folder, imap_uid);

-- Sync state: account lookups
CREATE INDEX IF NOT EXISTS idx_sync_state_account ON sync_state(account_id);

-- Outbox: queue processing by status and retry time
CREATE INDEX IF NOT EXISTS idx_outbox_status_retry ON outbox(send_status, next_retry_at);

-- Attachments: message lookups
CREATE INDEX IF NOT EXISTS idx_attachments_message ON attachments(message_id);

-- Folders: account lookups
CREATE INDEX IF NOT EXISTS idx_folders_account ON folders(account_id);

-- Threads: latest message sorting
CREATE INDEX IF NOT EXISTS idx_threads_latest_date ON threads(latest_message_date DESC);
