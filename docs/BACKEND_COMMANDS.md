# Backend Commands Documentation

## Overview

This document provides comprehensive documentation for all Tauri commands in the OpenHuman backend. Each command includes:

- Rust signature
- TypeScript types
- Usage examples
- Error handling
- Related events

All commands follow the IPC pattern documented in `docs/IPC_PATTERN.md`.

---

## Table of Contents

1. [Account Management](#account-management)
2. [Email Sync](#email-sync)
3. [Email Operations](#email-operations)
4. [Email List & Filtering](#email-list--filtering)
5. [Search](#search)
6. [Label Operations](#label-operations)
7. [Folder Operations](#folder-operations)

---

## Account Management

### `add_account`

Add a new email account to the application.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn add_account(
    email: String,
    provider: String,
    imap_host: String,
    imap_port: u16,
    smtp_host: String,
    smtp_port: u16,
    password: String,
) -> Result<Account, String>
```

**TypeScript Types:**

```typescript
interface AddAccountRequest {
  email: string;
  provider: string;
  imap_host: string;
  imap_port: number;
  smtp_host: string;
  smtp_port: number;
  password: string;
}

type AddAccountResponse = Account;
```

**Usage Example (TypeScript):**

```typescript
import { invokeAddAccount } from '@/types/commands';

try {
  const account = await invokeAddAccount({
    email: 'user@example.com',
    provider: 'Gmail',
    imap_host: 'imap.gmail.com',
    imap_port: 993,
    smtp_host: 'smtp.gmail.com',
    smtp_port: 465,
    password: 'app-specific-password',
  });

  console.log('Account added:', account.id);
} catch (error) {
  console.error('Failed to add account:', error);
}
```

**Side Effects:**

- Stores credentials in OS keychain with key `openhuman:<email>`
- Creates account record in `accounts` table
- Initializes `sync_state` entry for the account
- Discovers folders via IMAP LIST command (INBOX, Sent, Archive, Trash, etc.)

**Error Cases:**

- Duplicate email address: `"Account with this email already exists"`
- Keychain access denied: `"Failed to store credentials: <reason>"`
- Database error: `"Database error: <reason>"`

---

### `list_accounts`

List all email accounts.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn list_accounts() -> Result<Vec<Account>, String>
```

**TypeScript Types:**

```typescript
type ListAccountsResponse = Account[];
```

**Usage Example (TypeScript):**

```typescript
import { invokeListAccounts } from '@/types/commands';

const accounts = await invokeListAccounts();
console.log(`Found ${accounts.length} accounts`);
```

---

### `remove_account`

Remove an email account and all associated data.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn remove_account(account_id: i64) -> Result<(), String>
```

**TypeScript Types:**

```typescript
interface RemoveAccountRequest {
  accountId: number;
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeRemoveAccount } from '@/types/commands';

await invokeRemoveAccount(accountId);
console.log('Account removed successfully');
```

**Side Effects:**

- Deletes credentials from OS keychain
- Deletes account from `accounts` table
- CASCADE DELETE: All messages, threads, labels, folders, sync state

**Warning:** This operation is irreversible. All local email data will be lost.

---

## Email Sync

### `sync_emails`

Sync emails from IMAP server for one or all accounts.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn sync_emails(account_id: Option<i64>) -> Result<SyncStats, String>
```

**TypeScript Types:**

```typescript
interface SyncEmailsRequest {
  accountId?: number; // undefined = sync all accounts
}

interface SyncStats {
  messages_synced: number;
  new_messages: number;
  errors: string[];
}

type SyncEmailsResponse = SyncStats;
```

**Usage Example (TypeScript):**

```typescript
import { invokeSyncEmails } from '@/types/commands';

// Sync specific account
const stats = await invokeSyncEmails(accountId);
console.log(`Synced ${stats.new_messages} new messages`);

// Sync all accounts
const allStats = await invokeSyncEmails();
console.log(`Total synced: ${allStats.messages_synced}`);
```

**Side Effects:**

- Fetches new messages via IMAP UID-based incremental sync
- Parses MIME messages and extracts headers/bodies/attachments
- Stores messages in `messages` table with IMAP UIDs and flags
- Updates `sync_state` with last_sync_at timestamp and UIDVALIDITY
- Runs JWZ threading algorithm to group messages
- Updates thread metadata (participant count, latest date, unread count)

**Events Emitted:**

- `sync_started` - When sync begins
- `sync_progress` - Periodic progress updates during sync
- `sync_completed` - When sync finishes successfully
- `sync_failed` - When sync fails after max retries

**Retry Logic:**

- Max 5 attempts with exponential backoff (1s, 2s, 4s, 8s, 16s)
- Authentication failures: fail immediately, emit `auth_required` event
- Network errors: retry with backoff

**Multi-Folder Sync:**

- INBOX syncs every 5 minutes
- Other folders sync every 30 minutes
- Parallel folder sync using tokio::spawn
- UIDVALIDITY tracking per folder to detect mailbox rebuilds

**Initial Sync:** Fetches last 30 days of emails per folder, then background historical sync (30 days at a time going backward)

---

### `get_sync_status`

Get sync status for an account.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn get_sync_status(account_id: i64) -> Result<SyncStatusInfo, String>
```

**TypeScript Types:**

```typescript
interface SyncStatusInfo {
  account_id: number;
  sync_status: string; // 'idle' | 'syncing' | 'completed' | 'failed'
  last_sync_at: string | null;
  messages_fetched: number;
  error_message: string | null;
}

type GetSyncStatusResponse = SyncStatusInfo;
```

**Usage Example (TypeScript):**

```typescript
import { invokeGetSyncStatus } from '@/types/commands';

const status = await invokeGetSyncStatus(accountId);
if (status.sync_status === 'failed') {
  console.error('Sync failed:', status.error_message);
}
```

---

### `cancel_sync`

Cancel ongoing sync for an account.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn cancel_sync(account_id: i64) -> Result<(), String>
```

**TypeScript Types:**

```typescript
interface CancelSyncRequest {
  accountId: number;
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeCancelSync } from '@/types/commands';

await invokeCancelSync(accountId);
console.log('Sync cancelled');
```

---

## Email Operations

### `send_email`

Queue an email for sending via SMTP.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn send_email(params: SendEmailParams) -> Result<i64, String>

pub struct SendEmailParams {
    pub account_id: i64,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body_plain: String,
    pub body_html: Option<String>,
}
```

**TypeScript Types:**

```typescript
interface SendEmailParams {
  account_id: number;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string;
  body_plain: string;
  body_html?: string;
}

type SendEmailResponse = number; // Returns outbox ID
```

**Usage Example (TypeScript):**

```typescript
import { invokeSendEmail } from '@/types/commands';

const outboxId = await invokeSendEmail({
  account_id: 1,
  to: ['recipient@example.com'],
  cc: [],
  bcc: [],
  subject: 'Hello',
  body_plain: 'Plain text version',
  body_html: '<p>HTML version</p>',
});

console.log('Queued in outbox:', outboxId);
```

**Side Effects:**

- Queues email in `outbox` table with status `pending`
- Processes outbox queue immediately (async)
- Sends via SMTP with exponential backoff retry

**Events Emitted:**

- `send_status` - Updates as email moves through sending process

**MIME Formatting:**

- Creates multipart/alternative message with plain and HTML parts
- Handles recipient lists (To, Cc, Bcc)

**Note:** Attachment support is planned for future implementation.

---

### `mark_read` / `mark_unread`

Mark messages as read or unread.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn mark_read(message_ids: Vec<i64>) -> Result<(), String>

#[tauri::command]
pub async fn mark_unread(message_ids: Vec<i64>) -> Result<(), String>
```

**TypeScript Types:**

```typescript
interface MarkReadRequest {
  messageIds: number[];
}

interface MarkUnreadRequest {
  messageIds: number[];
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeMarkRead, invokeMarkUnread } from '@/types/commands';

await invokeMarkRead([1, 2, 3]);
await invokeMarkUnread([4, 5]);
```

---

### `star_message` / `unstar_message`

Star or unstar a message.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn star_message(message_id: i64) -> Result<(), String>

#[tauri::command]
pub async fn unstar_message(message_id: i64) -> Result<(), String>
```

**TypeScript Types:**

```typescript
interface StarMessageRequest {
  messageId: number;
}

interface UnstarMessageRequest {
  messageId: number;
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeStarMessage, invokeUnstarMessage } from '@/types/commands';

await invokeStarMessage(messageId);
await invokeUnstarMessage(messageId);
```

---

### `delete_message`

Permanently delete a message.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn delete_message(message_id: i64) -> Result<(), String>
```

**TypeScript Types:**

```typescript
interface DeleteMessageRequest {
  messageId: number;
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeDeleteMessage } from '@/types/commands';

await invokeDeleteMessage(messageId);
```

**Side Effects:**

- CASCADE DELETE: All attachments, labels, FTS5 entries

**Warning:** This operation is irreversible.

---

### `bulk_mark_read`

Batch update read status for multiple messages.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn bulk_mark_read(message_ids: Vec<i64>, is_read: bool) -> Result<(), String>
```

**TypeScript Types:**

```typescript
interface BulkMarkReadRequest {
  messageIds: number[];
  isRead: boolean;
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeBulkMarkRead } from '@/types/commands';

await invokeBulkMarkRead([1, 2, 3, 4, 5], true); // Mark as read
await invokeBulkMarkRead([6, 7, 8], false); // Mark as unread
```

---

### `bulk_archive_messages`

Move multiple messages to Archive folder.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn bulk_archive_messages(message_ids: Vec<i64>) -> Result<(), String>
```

**TypeScript Types:**

```typescript
interface BulkArchiveMessagesRequest {
  messageIds: number[];
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeBulkArchiveMessages } from '@/types/commands';

await invokeBulkArchiveMessages([1, 2, 3, 4, 5]);
```

**Side Effects:**

- Updates `folder` field to `'Archive'` for all specified messages

---

## Email List & Filtering

### `list_messages`

List messages with optional filters.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn list_messages(
    account_id: Option<i64>,
    folder: Option<String>,
    label_id: Option<i64>,
    is_read: Option<bool>,
    is_starred: Option<bool>,
    limit: Option<i64>,
) -> Result<Vec<MessageListItem>, String>
```

**TypeScript Types:**

```typescript
interface MessageListItem {
  id: number;
  subject: string | null;
  from_addr: string;
  preview: string; // First 100 chars of body_plain
  date: string | null;
  is_read: boolean;
  is_starred: boolean;
  has_attachments: boolean;
}

interface ListMessagesRequest {
  accountId?: number;
  folder?: string;
  labelId?: number;
  isRead?: boolean;
  isStarred?: boolean;
  limit?: number; // Default: 10000
}

type ListMessagesResponse = MessageListItem[];
```

**Usage Example (TypeScript):**

```typescript
import { invokeListMessages } from '@/types/commands';

// Get all unread messages in INBOX
const unread = await invokeListMessages({
  folder: 'INBOX',
  isRead: false,
});

// Get starred messages for account
const starred = await invokeListMessages({
  accountId: 1,
  isStarred: true,
});

// Get messages with specific label
const labeled = await invokeListMessages({
  labelId: 5,
  limit: 100,
});
```

**Performance:**

- Uses indexed queries (target <50ms)
- Default limit: 10,000 messages
- Sorted by date DESC

---

## Search

### `search_messages`

Full-text search across all messages using FTS5.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn search_messages(
    query: String,
    account_id: Option<i64>,
) -> Result<Vec<MessageSearchResult>, String>
```

**TypeScript Types:**

```typescript
interface MessageSearchResult {
  id: number;
  account_id: number;
  message_id: string;
  subject: string | null;
  from_addr: string;
  to_addr: string | null;
  date: string | null;
  is_read: boolean;
  is_starred: boolean;
  snippet: string; // Contains <mark> tags for highlighting
}

interface SearchMessagesRequest {
  query: string;
  accountId?: number;
}

type SearchMessagesResponse = MessageSearchResult[];
```

**Usage Example (TypeScript):**

```typescript
import { invokeSearchMessages } from '@/types/commands';

// Search all accounts
const results = await invokeSearchMessages('meeting agenda');

// Search specific account
const accountResults = await invokeSearchMessages('urgent', accountId);

// Phrase search
const phraseResults = await invokeSearchMessages('"quarterly report"');

// Boolean search
const boolResults = await invokeSearchMessages('project AND deadline');
```

**Search Features:**

- Searches subject, body_plain, body_html
- Snippet generation with `<mark>` tags
- Phrase queries: `"exact phrase"`
- Boolean operators: `AND`, `OR`, `NOT`
- Porter stemming for better matches
- Unicode support (UTF-8)

**Performance:**

- FTS5 index provides instant results
- Target: <100ms for typical queries
- Limit: 100 results

**Snippet Format:**

```html
"...found <mark>meeting</mark> agenda in the..."
```

---

## Label Operations

### `create_label`

Create a new label for an account.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn create_label(
    account_id: i64,
    name: String,
    color: Option<String>,
) -> Result<Label, String>
```

**TypeScript Types:**

```typescript
interface CreateLabelRequest {
  accountId: number;
  name: string;
  color?: string; // Hex color code
}

interface Label {
  id: number;
  account_id: number;
  name: string;
  color?: string;
  created_at: string;
  message_count?: number;
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeCreateLabel } from '@/types/commands';

const label = await invokeCreateLabel(accountId, 'Work', '#3b82f6');
console.log('Created label:', label.id);
```

---

### `list_labels`

List all labels for an account.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn list_labels(
    account_id: i64,
    include_counts: bool,
) -> Result<Vec<Label>, String>
```

**TypeScript Types:**

```typescript
interface ListLabelsRequest {
  accountId: number;
  includeCounts: boolean; // Include message_count field
}
```

**Usage Example (TypeScript):**

```typescript
import { invokeListLabels } from '@/types/commands';

const labels = await invokeListLabels(accountId, true);
labels.forEach(label => {
  console.log(`${label.name}: ${label.message_count} messages`);
});
```

---

### `delete_label`

Delete a label and remove all message associations.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn delete_label(label_id: i64) -> Result<(), String>
```

**Usage Example (TypeScript):**

```typescript
import { invokeDeleteLabel } from '@/types/commands';

await invokeDeleteLabel(labelId);
```

**Side Effects:**

- CASCADE DELETE: All message_labels entries

---

### `apply_label` / `remove_label`

Apply or remove a label from a message.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn apply_label(message_id: i64, label_id: i64) -> Result<(), String>

#[tauri::command]
pub async fn remove_label(message_id: i64, label_id: i64) -> Result<(), String>
```

**Usage Example (TypeScript):**

```typescript
import { invokeApplyLabel, invokeRemoveLabel } from '@/types/commands';

await invokeApplyLabel(messageId, labelId);
await invokeRemoveLabel(messageId, labelId);
```

---

### `get_message_labels`

Get all labels applied to a message.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn get_message_labels(message_id: i64) -> Result<Vec<Label>, String>
```

**Usage Example (TypeScript):**

```typescript
import { invokeGetMessageLabels } from '@/types/commands';

const labels = await invokeGetMessageLabels(messageId);
console.log(
  'Message has labels:',
  labels.map(l => l.name)
);
```

---

### `archive_messages`

Move messages to Archive folder.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn archive_messages(message_ids: Vec<i64>) -> Result<(), String>
```

**Usage Example (TypeScript):**

```typescript
import { invokeArchiveMessages } from '@/types/commands';

await invokeArchiveMessages([1, 2, 3]);
```

---

## Folder Operations

### `list_folders`

List all folders for an account.

**Rust Signature:**

```rust
#[tauri::command]
pub async fn list_folders(account_id: Option<i64>) -> Result<Vec<Folder>, String>
```

**TypeScript Types:**

```typescript
interface Folder {
  id: number;
  account_id: number;
  name: string;
  message_count: number;
}

interface ListFoldersRequest {
  accountId?: number; // undefined = all accounts
}

type ListFoldersResponse = Folder[];
```

**Usage Example (TypeScript):**

```typescript
import { invokeListFolders } from '@/types/commands';

const folders = await invokeListFolders(accountId);
folders.forEach(folder => {
  console.log(`${folder.name}: ${folder.message_count} messages`);
});
```

---

## Tauri Events

### Event Listening

See `src/lib/tauri-events.ts` for type-safe event listeners.

**Example:**

```typescript
import { onSyncCompleted, onSyncFailed } from '@/lib/tauri-events';

const unsubscribe = await onSyncCompleted(event => {
  console.log('Sync completed:', event.payload.stats);
});

// Clean up when done
unsubscribe();
```

### Available Events

- `sync_started` - Sync begins for account
- `sync_progress` - Periodic progress updates
- `sync_completed` - Sync finishes successfully
- `sync_failed` - Sync fails after retries
- `send_status` - Email send status changes
- `auth_required` - Authentication failure, user action needed

---

## Error Handling

All commands return `Result<T, String>` with user-friendly error messages.

**Common Error Patterns:**

```typescript
try {
  await invokeSendEmail(params);
} catch (error) {
  if (error instanceof Error) {
    // Type-safe error handling
    if (error.message.includes('authentication')) {
      // Handle auth error
    } else if (error.message.includes('network')) {
      // Handle network error
    } else {
      // Generic error
      console.error('Failed to send email:', error.message);
    }
  }
}
```

---

## Related Documentation

- **IPC Pattern:** `docs/IPC_PATTERN.md` - Type-safe command pattern
- **Database Schema:** `docs/DATABASE_SCHEMA.md` - Table definitions
- **TypeScript Types:** `src/types/commands.ts` - All command types
- **Event Listeners:** `src/lib/tauri-events.ts` - Event handling utilities
