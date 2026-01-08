# Database Schema Documentation

## Overview

OpenHuman uses **SQLite** as its local-first database with **SQLx** ORM for type-safe queries and compile-time verification. The schema is designed to support:

- Multi-account email management
- Email threading using JWZ algorithm (RFC 5256)
- Full-text search via FTS5
- Gmail-style labels
- SMTP outbox queue with retry logic
- IMAP sync state tracking with multi-folder support

## Schema Version

**Current Version:** 3 migrations  
**Migration System:** SQLx built-in migrations with `migrate!` macro

## Tables

### `accounts`

Stores email account configurations for IMAP/SMTP connections.

```sql
CREATE TABLE accounts (
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
```

**Fields:**

- `id` - Unique account identifier
- `email` - Email address (unique constraint)
- `provider` - Provider name (e.g., "Gmail", "Outlook", "Custom")
- `imap_host`, `imap_port` - IMAP server configuration (default port: 993)
- `smtp_host`, `smtp_port` - SMTP server configuration
- `sync_enabled` - Boolean flag (1=enabled, 0=disabled)
- `created_at`, `updated_at` - Timestamps

**Credentials:** Stored separately in OS keychain (not in database)

**Key:** `openhuman:<email_address>`

---

### `messages`

Stores parsed email message data with IMAP-specific fields.

```sql
CREATE TABLE messages (
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
```

**Fields:**

- `id` - Internal row ID
- `account_id` - Foreign key to accounts table
- `message_id` - Globally unique Message-ID from email headers
- `thread_id` - Foreign key to threads table (JWZ algorithm)
- `folder` - Folder name (INBOX, Sent, Archive, Trash, etc.)
- `subject` - Email subject line
- `from_addr`, `to_addr`, `cc_addr`, `bcc_addr` - Email addresses
- `date` - Send date from email headers (ISO 8601 format)
- `is_read`, `is_starred` - Boolean flags (0/1)
- `body_plain`, `body_html` - Message bodies
- `references_header`, `in_reply_to` - Threading headers
- `imap_uid` - IMAP UID for this message (unique per folder)
- `imap_flags` - JSON string of IMAP flags (\Seen, \Flagged, \Deleted, etc.)

**Unique Constraint:** `(account_id, message_id)` prevents duplicate messages

**Indexes:**

- `idx_messages_account_date` - Fast inbox queries sorted by date
- `idx_messages_thread` - Thread grouping lookups
- `idx_messages_message_id` - Threading by Message-ID
- `idx_messages_folder` - Folder filtering
- `idx_messages_read`, `idx_messages_starred` - Status filtering
- `idx_messages_imap_uid` - Fast IMAP UID lookups for incremental sync

---

### `threads`

Stores conversation thread metadata using JWZ algorithm.

```sql
CREATE TABLE threads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    thread_subject TEXT,
    participant_count INTEGER NOT NULL DEFAULT 0,
    latest_message_date TEXT,
    unread_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

**Fields:**

- `id` - Thread identifier
- `thread_subject` - Base subject (stripped of Re:/Fwd: prefixes)
- `participant_count` - Number of unique participants in thread
- `latest_message_date` - Date of most recent message in thread
- `unread_count` - Count of unread messages in thread

**Threading Algorithm:** Jamie Zawinski REFERENCES (RFC 5256)

- Uses `Message-ID`, `References`, `In-Reply-To` headers
- Supports dummy placeholders for missing parent messages
- Subject-based fallback for messages without References headers

**Index:**

- `idx_threads_latest_date` - Sort threads by latest activity

---

### `attachments`

Stores email attachment metadata.

```sql
CREATE TABLE attachments (
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
```

**Fields:**

- `id` - Attachment identifier
- `message_id` - Foreign key to messages table
- `filename` - Original filename from email
- `size` - File size in bytes
- `mime_type` - MIME type (e.g., "image/png", "application/pdf")
- `file_path` - Path to stored file on disk
- `is_inline` - Boolean flag for inline images (0/1)
- `content_id` - Content-ID for inline attachments

**Cascade Delete:** Deleting a message deletes all its attachments

**Index:**

- `idx_attachments_message` - Fast message attachment lookups

---

### `labels`

Stores Gmail-style labels for tag-based organization.

```sql
CREATE TABLE labels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    color TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
    UNIQUE(account_id, name)
);
```

**Fields:**

- `id` - Label identifier
- `account_id` - Foreign key to accounts table
- `name` - Label name (unique per account)
- `color` - Optional hex color code for UI display

**Unique Constraint:** `(account_id, name)` prevents duplicate label names

**Index:**

- `idx_labels_account` - Fast account label lookups

---

### `message_labels`

Junction table for many-to-many message-label relationships.

```sql
CREATE TABLE message_labels (
    message_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (message_id, label_id),
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE,
    FOREIGN KEY (label_id) REFERENCES labels(id) ON DELETE CASCADE
);
```

**Fields:**

- `message_id` - Foreign key to messages table
- `label_id` - Foreign key to labels table

**Composite Primary Key:** `(message_id, label_id)` prevents duplicates

**Cascade Delete:**

- Deleting a message removes all its label associations
- Deleting a label removes all message associations

**Indexes:**

- `idx_message_labels_message` - Fast message-to-labels lookups
- `idx_message_labels_label` - Fast label-to-messages lookups

---

### `outbox`

Queues outgoing emails for SMTP sending with retry logic.

```sql
CREATE TABLE outbox (
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
```

**Fields:**

- `id` - Outbox entry identifier
- `account_id` - Foreign key to accounts table
- `recipients` - JSON string with to/cc/bcc arrays
- `subject`, `body_plain`, `body_html` - Email content
- `send_status` - Status: `pending`, `sending`, `sent`, `failed`
- `retry_count` - Number of send attempts (max 5)
- `next_retry_at` - ISO 8601 timestamp for next retry
- `error_message` - Last error message on failure

**Retry Strategy:** Exponential backoff (1s, 2s, 4s, 8s, 16s)

**Index:**

- `idx_outbox_status_retry` - Queue processing optimization

---

### `sync_state`

Tracks IMAP sync progress and UID mappings per account.

```sql
CREATE TABLE sync_state (
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
```

**Fields:**

- `id` - Sync state identifier
- `account_id` - Foreign key to accounts table (unique)
- `last_sync_at` - ISO 8601 timestamp of last successful sync
- `uid_mappings` - JSON string mapping UIDs to message IDs
- `uid_validity` - IMAP UIDVALIDITY value for detecting mailbox rebuilds
- `uid_next` - IMAP UIDNEXT value for tracking next expected UID
- `messages_fetched` - Count of messages fetched in last sync
- `sync_status` - Status: `idle`, `syncing`, `completed`, `failed`
- `error_message` - Last sync error message

**Unique Constraint:** One sync state per account

**UIDVALIDITY:** IMAP mechanism to detect when a mailbox has been rebuilt (triggers full re-sync)

**Index:**

- `idx_sync_state_account` - Fast account sync state lookups

---

### `folders`

Stores folder/label definitions per account with IMAP-specific metadata.

```sql
CREATE TABLE folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    folder_type TEXT,
    selectable INTEGER DEFAULT 1,
    flags TEXT,
    uidvalidity INTEGER,
    uidnext INTEGER,
    message_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
    UNIQUE(account_id, name)
);
```

**Fields:**

- `id` - Folder identifier
- `account_id` - Foreign key to accounts table
- `name` - Folder name (INBOX, Sent, Archive, Trash, etc.)
- `folder_type` - Auto-detected folder type (INBOX, Sent, Drafts, Trash, Archive, Junk)
- `selectable` - Boolean flag (1=selectable, 0=hierarchy only)
- `flags` - JSON string of IMAP folder flags
- `uidvalidity` - IMAP UIDVALIDITY for this folder
- `uidnext` - IMAP UIDNEXT for this folder
- `message_count` - Cached count of messages in folder

**Unique Constraint:** `(account_id, name)` prevents duplicate folder names

**Default Folders:** Auto-discovered via IMAP LIST command on account setup

**Folder Type Detection:** Automatically detects special folders (INBOX, Sent, Drafts, Trash, Archive) based on name patterns and IMAP attributes

---

### `message_fts`

FTS5 virtual table for full-text search.

```sql
CREATE VIRTUAL TABLE message_fts USING fts5(
    message_id UNINDEXED,
    subject,
    body_plain,
    body_html,
    content='messages',
    content_rowid='id',
    tokenize='porter unicode61'
);
```

**Fields:**

- `message_id` - Unindexed copy for result retrieval
- `subject`, `body_plain`, `body_html` - Indexed for search

**Configuration:**

- `content='messages'` - Links to messages table
- `content_rowid='id'` - Row ID mapping
- `tokenize='porter unicode61'` - Porter stemming + Unicode support

**Triggers:** Auto-sync with messages table (insert/update/delete)

**Search Features:**

- Full-text search across subject and body
- Phrase queries: `"exact phrase"`
- Boolean operators: `AND`, `OR`, `NOT`
- Snippet generation with `<mark>` tags for highlighting

---

## Migrations

Migrations are stored in `src-tauri/migrations/` and applied automatically on app startup.

### Migration Files

1. **20260105000001_initial_schema.sql** - Core tables (accounts, messages, threads, attachments, outbox, sync_state, folders)
2. **20260105000002_fts5_search.sql** - FTS5 virtual table and triggers
3. **20260106000001_labels_schema.sql** - Labels and message_labels tables

### Running Migrations

Migrations run automatically via SQLx `migrate!` macro embedded at compile time:

```rust
// In src-tauri/src/db.rs
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

### Creating New Migrations

```bash
# Create new migration file
touch src-tauri/migrations/YYYYMMDDHHMMSS_description.sql

# Write SQL in the file
# Rebuild to embed at compile time
cargo build
```

---

## Query Performance Guidelines

### Fast Queries (<50ms target)

**✅ Good:**

```sql
-- Uses idx_messages_account_date index
SELECT * FROM messages
WHERE account_id = ?
ORDER BY date DESC
LIMIT 100;

-- Uses idx_messages_thread index
SELECT * FROM messages
WHERE thread_id = ?;

-- Uses FTS5 index
SELECT * FROM message_fts
WHERE message_fts MATCH 'query';
```

**❌ Avoid:**

```sql
-- Full table scan (no index on subject)
SELECT * FROM messages
WHERE subject LIKE '%search%';

-- Full table scan (no index on body_plain)
SELECT * FROM messages
WHERE body_plain LIKE '%text%';
```

**💡 Use FTS5 instead:**

```sql
-- Fast full-text search
SELECT * FROM message_fts
WHERE message_fts MATCH 'subject:search OR body_plain:text';
```

---

## Database Initialization

The database is initialized on app startup before any other operations:

```rust
// In src-tauri/src/main.rs
#[tokio::main]
async fn main() {
    // Initialize database pool
    crate::db::init_db().await
        .expect("Failed to initialize database");

    // Run Tauri app
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Failed to run app");
}
```

### Database Location

**Development:** `~/.openhuman/dev.db`  
**Production:** `~/.openhuman/openhuman.db`

**Encryption:** Relies on filesystem encryption (FileVault, BitLocker, etc.)  
**Credentials:** Stored in OS keychain, never in database

---

## Backup & Recovery

### Manual Backup

```bash
# Copy database file
cp ~/.openhuman/openhuman.db ~/.openhuman/backup-$(date +%Y%m%d).db
```

### Restore

```bash
# Stop app, replace database
cp ~/.openhuman/backup-20260106.db ~/.openhuman/openhuman.db
```

### Data Export

Future feature: Export to JSON/CSV via Tauri command

---

## Related Documentation

- **IPC Pattern:** `docs/IPC_PATTERN.md` - Type-safe command definitions
- **Backend Commands:** `docs/BACKEND_COMMANDS.md` - All Tauri commands
- **Migrations Standards:** `agent-os/standards/backend/migrations.md` - Best practices
