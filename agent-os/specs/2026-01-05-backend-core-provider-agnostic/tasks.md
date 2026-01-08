# Task Breakdown: Backend Core (Provider-Agnostic)

## Overview

Total Tasks: 8 Task Groups

## Task List

### Foundation & Dependencies

#### Task Group 1: Project Dependencies and Error Types

**Dependencies:** None

- [x] 1.0 Complete dependency setup and error infrastructure
  - [x] 1.1 Add required dependencies to `src-tauri/Cargo.toml`
    - SQLx with SQLite, tokio runtime, and migrations features
    - lettre for SMTP client
    - async-pop3 or custom POP3 implementation dependencies
    - mailparse or mail-parser for MIME parsing
    - keyring for OS keychain integration
    - thiserror for custom error types
    - tokio with full features for async runtime
  - [x] 1.2 Define custom error types using thiserror
    - Create `src-tauri/src/error.rs` with enums: DatabaseError, SyncError, SmtpError, Pop3Error, MimeParseError, KeychainError
    - Implement Display and Error traits via thiserror derive macros
    - Add user-friendly error messages following `agent-os/standards/global/error-handling.md`
    - Define Result type aliases: `type DbResult<T> = Result<T, DatabaseError>`
  - [x] 1.3 Create retry utilities module
    - Implement exponential backoff function: `async fn retry_with_backoff<F, T>(operation: F, max_attempts: u32) -> Result<T>`
    - Start with 1s delay, double on each retry (1s, 2s, 4s, 8s, 16s)
    - Max 5 attempts as specified in requirements
  - [x] 1.4 Run cargo build to verify dependencies compile
    - Ensure no dependency conflicts
    - Verify error types compile correctly
    - Check that retry utilities are properly typed

**Acceptance Criteria:**

- All dependencies added and compile without conflicts
- Custom error types defined with user-friendly messages
- Exponential backoff retry utility implemented with correct timing
- Cargo build succeeds

### Database Layer

#### Task Group 2: SQLite Schema and Migrations

**Dependencies:** Task Group 1

- [x] 2.0 Complete database schema and migration system
  - [x] 2.1 Write 2-6 focused tests for database models
    - Test account creation with valid credentials
    - Test message insertion with foreign key constraints
    - Test thread creation and parent-child relationships
    - Test sync_state UIDL mapping storage and retrieval
    - Skip exhaustive edge case testing
  - [x] 2.2 Create initial migration `001_initial_schema.sql`
    - Create `accounts` table: id, email, provider, pop3_host, pop3_port, smtp_host, smtp_port, sync_enabled, created_at, updated_at
    - Create `messages` table: id, account_id, message_id, thread_id, folder, subject, from_addr, to_addr, cc_addr, bcc_addr, date, is_read, is_starred, body_plain, body_html, references, in_reply_to, created_at
    - Create `threads` table: id, thread_subject, participant_count, latest_message_date, unread_count, created_at
    - Create `attachments` table: id, message_id, filename, size, mime_type, file_path, is_inline, content_id, created_at
    - Create `outbox` table: id, account_id, recipients, subject, body_plain, body_html, send_status, retry_count, next_retry_at, created_at
    - Create `sync_state` table: id, account_id, last_sync_at, uidl_mappings (JSON), messages_fetched, sync_status, error_message
    - Create `folders` table: id, account_id, name, message_count, created_at
  - [x] 2.3 Add indexes to initial migration
    - Index on messages(account_id, date DESC) for inbox queries
    - Index on messages(thread_id) for conversation grouping
    - Index on messages(message_id) for threading lookups
    - Index on sync_state(account_id) for sync status checks
    - Index on outbox(send_status, next_retry_at) for queue processing
  - [x] 2.4 Add foreign key constraints with ON DELETE CASCADE
    - messages.account_id → accounts.id
    - messages.thread_id → threads.id
    - attachments.message_id → messages.id
    - outbox.account_id → accounts.id
    - sync_state.account_id → accounts.id
    - folders.account_id → accounts.id
  - [x] 2.5 Create FTS5 migration `002_fts5_search.sql`
    - Create virtual FTS5 table for messages: message_fts(subject, body_plain, body_html)
    - Add triggers to keep FTS5 table in sync with messages table
    - Create index for fast full-text search
  - [x] 2.6 Initialize database connection in `src-tauri/src/db.rs`
    - Create connection pool with SQLx
    - Run migrations on app startup using `sqlx::migrate!` macro
    - Export connection pool for use in commands
  - [x] 2.7 Ensure database tests pass
    - Run ONLY the 2-6 tests written in 2.1
    - Verify migrations run successfully with `cargo sqlx migrate run`
    - Test foreign key constraints work correctly

**Acceptance Criteria:**

- The 2-6 tests written in 2.1 pass
- All tables created with correct schema
- Indexes and foreign keys properly configured
- FTS5 search index created and synchronized
- Database initializes on app startup
- Migrations run successfully

### Secrets Management

#### Task Group 3: OS Keychain Integration

**Dependencies:** Task Group 2

- [x] 3.0 Complete keychain integration for credential storage
  - [x] 3.1 Write 2-4 focused tests for keychain operations
    - Test storing credentials with key format `openhuman:<email>`
    - Test retrieving stored credentials
    - Test handling missing/deleted credentials gracefully
    - Skip platform-specific edge cases
  - [x] 3.2 Create keychain module `src-tauri/src/keychain.rs`
    - Implement `store_credentials(email: &str, password: &str) -> Result<()>`
    - Implement `get_credentials(email: &str) -> Result<String>`
    - Implement `delete_credentials(email: &str) -> Result<()>`
    - Use key format: `openhuman:<email_address>`
  - [x] 3.3 Handle keychain errors gracefully
    - Wrap keychain errors in custom KeychainError type
    - Never log or expose credentials in error messages
    - Return user-friendly error messages for auth failures
  - [x] 3.4 Ensure keychain tests pass
    - Run ONLY the 2-4 tests written in 3.1
    - Verify credentials stored and retrieved correctly
    - Test error handling for missing credentials

**Acceptance Criteria:**

- The 2-4 tests written in 3.1 pass
- Credentials stored securely in OS keychain
- No credentials logged or exposed in errors
- Graceful error handling for keychain failures

### Email Protocol Layer

#### Task Group 4: POP3 Fetch Engine

**Dependencies:** Task Groups 2, 3

- [x] 4.0 Complete POP3 fetch engine with incremental sync
  - [x] 4.1 Write 3-6 focused tests for POP3 operations
    - Test connection and authentication to POP3 server
    - Test UIDL command for fetching new message IDs
    - Test incremental sync (only new messages)
    - Test retry logic with exponential backoff (mock network failures)
    - Test authentication failure handling (no retry)
    - Skip exhaustive protocol edge cases
  - [x] 4.2 Create POP3 client module `src-tauri/src/email/pop3.rs`
    - Implement `Pop3Client` struct with async methods
    - Implement `connect(host: &str, port: u16, email: &str, password: &str) -> Result<Pop3Client>`
    - Implement `fetch_uidl_list() -> Result<Vec<String>>` for UIDL command
    - Implement `fetch_message(uidl: &str) -> Result<Vec<u8>>` to retrieve raw message
  - [x] 4.3 Implement incremental sync logic
    - Load UIDL mappings from `sync_state` table
    - Fetch new UIDLs not in existing mappings
    - Download only new messages (last N days on first sync, configurable default 30)
    - Update `sync_state` with new UIDL mappings after successful fetch
  - [x] 4.4 Add retry logic with exponential backoff
    - Use retry utility from Task 1.3 for network errors
    - Max 5 attempts with backoff: 1s, 2s, 4s, 8s, 16s
    - After 5 failures, mark account as "sync_failed" in sync_state
    - Handle auth failures separately: fail immediately without retry
  - [x] 4.5 Ensure POP3 tests pass
    - Run ONLY the 3-6 tests written in 4.1
    - Verify incremental sync fetches only new messages
    - Test retry logic and failure handling

**Acceptance Criteria:**

- The 3-6 tests written in 4.1 pass
- POP3 connection and authentication working
- UIDL-based incremental sync implemented
- Exponential backoff retry logic working correctly
- Auth failures handled without retry

#### Task Group 5: SMTP Send Engine

**Dependencies:** Task Groups 2, 3

- [x] 5.0 Complete SMTP send engine with outbox queue
  - [x] 5.1 Write 3-5 focused tests for SMTP operations
    - Test email sending with single recipient
    - Test batch sending (multiple To/Cc/Bcc recipients)
    - Test outbox queue processing
    - Test retry logic for failed sends
    - Skip exhaustive MIME formatting tests
  - [x] 5.2 Create SMTP client module `src-tauri/src/email/smtp.rs`
    - Implement `SmtpClient` struct using lettre crate
    - Implement `send_email(to: Vec<String>, subject: &str, body_plain: &str, body_html: Option<&str>) -> Result<()>`
    - Support single and batch recipients (To, Cc, Bcc)
    - Format emails with proper MIME structure (plain text + HTML)
  - [x] 5.3 Implement outbox queue processing
    - Query `outbox` table for pending emails (send_status = 'pending' or 'retry')
    - Process each email with retry logic
    - Update outbox record with send_status: 'sent', 'failed', 'retry'
    - Increment retry_count and set next_retry_at on failures
  - [x] 5.4 Add retry logic with exponential backoff
    - Use retry utility from Task 1.3 for network errors
    - Max 5 attempts with backoff: 1s, 2s, 4s, 8s, 16s
    - After 5 failures, mark email as "send_failed" in outbox
    - Allow manual retry trigger via Tauri command
  - [x] 5.5 Emit Tauri events for send progress
    - Emit 'email_send_started' event with email ID
    - Emit 'email_send_progress' with retry count
    - Emit 'email_send_completed' or 'email_send_failed' with final status
  - [x] 5.6 Ensure SMTP tests pass
    - Run ONLY the 3-5 tests written in 5.1
    - Verify single and batch sending works
    - Test outbox queue processing and retry logic

**Acceptance Criteria:**

- The 3-5 tests written in 5.1 pass
- SMTP client sends emails with proper MIME formatting
- Outbox queue processes pending emails
- Retry logic with exponential backoff working
- Tauri events emitted for send progress

### Message Processing Layer

#### Task Group 6: MIME Parsing and Threading

**Dependencies:** Task Group 4

- [x] 6.0 Complete MIME parsing and JWZ threading algorithm
  - [x] 6.1 Write 4-7 focused tests for MIME and threading
    - Test parsing plain text email
    - Test parsing HTML email with inline images
    - Test parsing email with attachments
    - Test parsing iCal calendar invites
    - Test JWZ threading with References header
    - Test subject-based fallback threading
    - Skip exhaustive malformed email tests
  - [x] 6.2 Create MIME parser module `src-tauri/src/email/mime_parser.rs`
    - Implement `parse_message(raw: &[u8]) -> Result<ParsedMessage>` using mailparse or mail-parser
    - Extract plain text body, HTML body, subject, headers (From, To, Cc, Bcc, Date)
    - Extract Message-ID, References, In-Reply-To headers for threading
    - Parse attachments: filename, size, MIME type, content
    - Handle inline images with is_inline flag and content-id
    - Parse iCal/ICS calendar invites as special attachment type
  - [x] 6.3 Implement HTML sanitization
    - Strip script tags and dangerous attributes to prevent XSS
    - Preserve safe HTML formatting for rendering
    - Log sanitization warnings for suspicious content
  - [x] 6.4 Handle malformed emails gracefully
    - Log parsing errors with message details
    - Store partial data if possible (e.g., headers but not body)
    - Continue sync without halting on single bad message
  - [x] 6.5 Create threading module `src-tauri/src/email/threading.rs`
    - Implement JWZ/REFERENCES algorithm (RFC 5256)
    - Build parent-child relationships using Message-ID and References headers
    - Fall back to In-Reply-To if References missing
    - Create dummy placeholder containers for missing parent messages
    - Implement subject-based fallback: extract base subject (remove "Re:", "Fwd:"), case-insensitive merge
  - [x] 6.6 Prevent threading infinite loops
    - Track visited Message-IDs during traversal
    - Detect and break circular references
    - Limit max thread depth to prevent stack overflow
  - [x] 6.7 Update thread metadata after grouping
    - Calculate participant_count from unique email addresses
    - Set latest_message_date from most recent message
    - Count unread_count from is_read = false messages
  - [x] 6.8 Ensure MIME and threading tests pass
    - Run ONLY the 4-7 tests written in 6.1
    - Verify MIME parsing extracts all required fields
    - Test threading algorithm groups messages correctly

**Acceptance Criteria:**

- The 4-7 tests written in 6.1 pass
- MIME parser extracts text, HTML, headers, attachments, inline images, iCal
- HTML sanitized to prevent XSS
- Malformed emails handled gracefully
- JWZ threading algorithm implemented correctly
- Thread metadata updated after grouping

### Sync Orchestration

#### Task Group 7: Background Sync Scheduler

**Dependencies:** Task Groups 4, 6

- [x] 7.0 Complete background sync orchestration
  - [x] 7.1 Write 3-5 focused tests for sync orchestration
    - Test initial sync on app launch
    - Test periodic sync with tokio interval
    - Test parallel multi-account sync
    - Test sync cancellation
    - Skip exhaustive concurrency edge cases
  - [x] 7.2 Create sync orchestrator module `src-tauri/src/email/sync_orchestrator.rs`
    - Implement `start_sync_scheduler(accounts: Vec<Account>, interval_minutes: u64)`
    - Use `tokio::time::interval` for periodic sync (default 5 minutes)
    - Trigger initial sync on app launch for all enabled accounts
  - [x] 7.3 Implement parallel multi-account sync
  - [x] 7.4 Track sync progress per account
  - [x] 7.5 Emit real-time sync events
  - [x] 7.6 Handle sync cancellation
  - [x] 7.7 Respect user preferences
  - [x] 7.8 Ensure sync orchestration tests pass
    - Run ONLY the 3-5 tests written in 7.1
    - Verify initial sync triggers on app launch
    - Test parallel sync across multiple accounts

**Acceptance Criteria:**

- The 3-5 tests written in 7.1 pass
- Initial sync triggers automatically on app launch
- Periodic sync runs at configured intervals
- Parallel multi-account sync implemented
- Sync events emitted to frontend
- Sync cancellation working correctly

### Tauri IPC Integration

#### Task Group 8: Tauri Commands and TypeScript Integration

**Dependencies:** Task Groups 2-7

- [x] 8.0 Complete Tauri IPC commands and frontend integration
  - [x] 8.1 Write 4-8 focused tests for Tauri commands
    - Test add_account command
    - Test sync_emails command
    - Test send_email command
    - Test search_messages command (FTS5)
    - Test mark_read/mark_unread commands
    - Test get_sync_status command
    - Skip exhaustive command parameter validation
  - [x] 8.2 Create account management commands `src-tauri/src/commands/account_management.rs`
  - [x] 8.3 Create email sync commands `src-tauri/src/commands/email_sync.rs`
  - [x] 8.4 Create email operations commands `src-tauri/src/commands/email_operations.rs`
  - [x] 8.5 Create search commands `src-tauri/src/commands/search.rs`
  - [x] 8.6 Register commands in `src-tauri/src/lib.rs`
  - [x] 8.7 Create TypeScript types in `src/types/commands.ts`
    - Added complete TypeScript interfaces for all Tauri commands
    - Added Tauri event types and event mapping
    - Created type-safe wrapper functions for all commands
    - Located in `src/types/commands.ts` (expanded from 108 to 400+ lines)
  - [x] 8.8 Setup Tauri event listeners in TypeScript
    - Created `src/lib/tauri-events.ts` with type-safe event listeners
    - Functions for each event: onSyncStarted, onSyncCompleted, onSyncFailed, etc.
    - Convenience function `setupTauriEventListeners` for bulk setup
    - Proper cleanup/unlisten support
  - [x] 8.9 Ensure Tauri command tests pass
    - Run ONLY the 4-8 tests written in 8.1
    - Verify commands execute successfully
    - Test error handling returns proper error messages

**Acceptance Criteria:**

- The 4-8 tests written in 8.1 pass
- All Tauri commands implemented and registered
- TypeScript types match Rust signatures exactly
- Type-safe invoke wrappers provided
- Tauri events emitted and documented
- Commands follow IPC pattern from `docs/IPC_PATTERN.md`

### Testing & Quality

#### Task Group 9: Integration Testing and Documentation

**Dependencies:** Task Groups 1-8

- [x] 9.0 Complete integration testing and documentation
  - [x] 9.1 Review existing tests from Task Groups 1-8
    - Review database tests (2-6 tests from Task 2.1)
    - Review keychain tests (2-4 tests from Task 3.1)
    - Review POP3 tests (3-6 tests from Task 4.1)
    - Review SMTP tests (3-5 tests from Task 5.1)
    - Review MIME/threading tests (4-7 tests from Task 6.1)
    - Review sync orchestration tests (3-5 tests from Task 7.1)
    - Review Tauri command tests (4-8 tests from Task 8.1)
    - Total existing tests: approximately 21-41 tests
  - [x] 9.2 Analyze test coverage gaps for backend core only
  - [x] 9.3 Write up to 10 additional integration tests maximum
  - [x] 9.4 Run all backend feature tests
  - [x] 9.5 Run clippy and rustfmt
  - [x] 9.6 Update technical documentation
    - Created `docs/DATABASE_SCHEMA.md` - Comprehensive database schema documentation with:
      - All 10 tables documented with field descriptions
      - Migration history and instructions
      - Query performance guidelines
      - Backup/recovery procedures
      - FTS5 search configuration
    - Created `docs/BACKEND_COMMANDS.md` - Complete Tauri command reference with:
      - All 30+ commands documented with Rust signatures
      - TypeScript types and usage examples for each command
      - Error handling patterns
      - Event emission documentation
      - Performance notes and best practices

**Acceptance Criteria:**

- All feature tests pass (approximately 31-51 tests total)
- No more than 10 additional integration tests added
- Code passes clippy and rustfmt checks
- Database schema documented
- Tauri commands documented with examples

## Execution Order

Recommended implementation sequence:

1. **Foundation & Dependencies** (Task Group 1) - Setup project dependencies and error infrastructure
2. **Database Layer** (Task Group 2) - Create SQLite schema and migrations
3. **Secrets Management** (Task Group 3) - Integrate OS keychain for credentials
4. **Email Protocol Layer** (Task Groups 4-5) - Implement POP3 and SMTP in parallel
5. **Message Processing** (Task Group 6) - Add MIME parsing and threading
6. **Sync Orchestration** (Task Group 7) - Build background sync scheduler
7. **Tauri IPC Integration** (Task Group 8) - Create commands and TypeScript bindings
8. **Testing & Quality** (Task Group 9) - Integration tests and documentation
