# Specification: Backend Core (Provider-Agnostic)

## Goal
Build a local-first, provider-agnostic email backend in Rust that handles account management, POP3/SMTP protocols, MIME parsing, message threading, and sync orchestration to support a keyboard-first email client with offline capabilities.

## User Stories
- As a power user, I want to connect multiple email accounts from different providers (Gmail, Outlook, custom domains) so that I can manage all my email in one unified interface
- As a privacy-conscious user, I want all my emails stored locally in SQLite with credentials in OS keychain so that I maintain complete control over my data without cloud dependencies

## Specific Requirements

**SQLite Database Schema with SQLx ORM**
- Use SQLx as ORM with built-in migration support and `migrate!` macro for compile-time embedded migrations
- Create migrations in `src-tauri/migrations/` directory with numbered files (`001_initial_schema.sql`, etc.)
- Implement core tables: `accounts` (email account config), `messages` (parsed email data with read/unread tracking), `threads` (JWZ algorithm grouping), `attachments` (metadata and file references), `outbox` (queued outgoing emails with retry tracking), `sync_state` (per-account UIDL mappings and sync status), `folders` (label management)
- Setup FTS5 full-text search index on message content (subject, body_plain, body_html) for instant local search
- All database operations must use parameterized queries to prevent SQL injection
- Database queries must complete in <50ms for inbox list operations to meet performance targets
- Follow migration best practices from `agent-os/standards/backend/migrations.md` (reversible, small focused changes)
- Initialize database on app startup before any other backend operations

**OS Keychain Secrets Storage**
- Use keyring-rs or tauri-plugin-keychain to store email account credentials in OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- Store per-account credentials with key format: `openhuman:<email_address>`
- Support username/password authentication for POP3/SMTP (defer OAuth to later phases)
- Rely on filesystem encryption for database content (no additional encryption layer on SQLite file)
- Handle keychain access failures gracefully and prompt user for re-authentication
- Never log or expose credentials in error messages or debug output

**POP3 Fetch Engine with Incremental Sync**
- Implement POP3 client using async-pop3 crate or custom implementation with tokio
- Use UIDL command to fetch only new messages (incremental sync) and store UIDL mappings in `sync_state` table
- On initial account sync, fetch emails from last N days (configurable via global app preference, default 30 days max)
- Track last sync timestamp per account and sync only messages received since last sync
- Support parallel sync across all accounts concurrently using tokio::spawn for each account
- Implement exponential backoff retry logic: start with 1s delay, double on each retry (1s, 2s, 4s, 8s, 16s), max 5 attempts
- After 5 failed sync attempts, mark account as "sync_failed" in `sync_state` and require user intervention
- Handle authentication failures separately: fail immediately without retry and flag for user re-authentication

**SMTP Send Engine with Outbox Queue**
- Implement SMTP client using lettre crate for sending emails
- Queue outgoing emails in `outbox` table with fields: recipient(s), subject, body, send_status, retry_count, next_retry_at
- Support single recipient and batch sending (multiple recipients in To/Cc/Bcc)
- Format emails with proper MIME structure supporting both plain text and HTML bodies
- Implement exponential backoff retry logic (same as POP3: max 5 attempts)
- After 5 failed send attempts, flag email as "send_failed" in outbox and allow manual retry trigger
- Process outbox queue on app launch and after each successful email compose action
- Emit progress events to frontend via Tauri events for real-time send status updates

**MIME Message Parsing**
- Use mailparse or mail-parser crate to parse raw email messages into structured data
- Extract and store in `messages` table: plain text body, HTML body, subject, from/to/cc/bcc addresses, date, Message-ID, References, In-Reply-To headers
- Parse and store attachment metadata in `attachments` table: filename, size, MIME type, file path reference
- Handle inline images by storing as attachments with `is_inline` flag and content-id reference
- Parse iCal/ICS calendar invites and store as special attachment type with parsed event data
- Handle malformed emails gracefully: log parsing errors, store partial data, continue sync (do not halt on single bad message)
- Extract all headers required for JWZ threading algorithm (Message-ID, References, In-Reply-To, Subject)
- Sanitize HTML content to prevent XSS when rendering in frontend (strip script tags, dangerous attributes)

**JWZ Threading Algorithm (RFC 5256)**
- Implement full Jamie Zawinski REFERENCES threading algorithm as used by Gmail/Superhuman
- Build parent-child relationships using Message-ID: parse References header (full ancestry chain), fall back to In-Reply-To if missing
- Create dummy placeholder containers in `threads` table for missing parent messages (when reply exists but original is missing)
- Implement subject-based fallback: extract base subject (remove "Re:", "Fwd:", "[prefix]"), merge threads with identical base subjects (case-insensitive)
- Create hierarchical conversation trees with unlimited nesting depth stored in `threads` table
- Prevent infinite loops by tracking visited Message-IDs during threading traversal
- Run threading algorithm after each sync batch to group new messages into conversations
- Update thread metadata: participant count, latest message date, unread count per thread

**Background Sync Orchestration**
- Use tokio timers to schedule periodic sync jobs (default every 5 minutes, configurable in app settings)
- Trigger initial sync automatically on app launch for all enabled accounts
- After app launch sync, start periodic background sync loop using `tokio::time::interval`
- Process all accounts in parallel: spawn separate tokio task per account, collect results with `join_all`
- Track sync progress per account: update `sync_state` table with last_sync_at, messages_fetched, sync_status
- Emit real-time sync events to frontend via Tauri events: sync_started, sync_progress, sync_completed, sync_failed
- Handle sync cancellation: allow user to cancel ongoing sync via frontend command
- Respect user preferences: skip sync for disabled accounts, honor custom sync intervals per account

**Error Handling and Retry Strategy**
- Network errors (connection timeout, DNS failure): exponential backoff with max 5 retries, then flag as failed
- Authentication failures (invalid credentials, expired tokens): fail immediately, no auto-retry, emit auth_required event to frontend
- MIME parsing errors: log error with message details, skip message, continue sync (do not halt entire sync)
- Database errors (constraint violations, disk full): log error, halt operation, emit critical error event to frontend
- After 5 retry attempts on any operation: store error details in sync_state or outbox, emit failed event, allow manual retry
- Use Result<T, E> pattern for all fallible operations with custom error types (e.g., SyncError, SmtpError, DatabaseError)
- Follow error handling standards from `agent-os/standards/global/error-handling.md` (user-friendly messages, fail fast, graceful degradation)

**Tauri IPC Commands**
- Follow established IPC pattern from `docs/IPC_PATTERN.md` with type-safe Rust commands and TypeScript wrappers
- Create command modules: `commands/account_management.rs` (add/edit/remove accounts), `commands/email_sync.rs` (trigger sync, get status), `commands/email_operations.rs` (send, mark read/unread), `commands/search.rs` (FTS5 queries)
- Register all commands in `lib.rs` using `tauri::generate_handler![]`
- Provide TypeScript types in `src/types/commands.ts` for all request/response payloads
- Emit Tauri events for background operations: sync progress, send status, error notifications
- Include comprehensive unit tests for each command using `#[cfg(test)]` pattern from existing codebase

## Existing Code to Leverage

**Tauri Command Pattern from `src-tauri/src/commands/greet.rs`**
- Modular command organization: create separate files per functional area (account_management.rs, email_sync.rs, etc.)
- Use `#[tauri::command]` attribute macro for command definitions with type-safe parameters
- Include comprehensive unit tests in `#[cfg(test)]` modules with assertions for success and error cases
- Register commands in `commands/mod.rs` and expose via `tauri::generate_handler![]` in `lib.rs`
- Follow exact pattern for async commands returning `Result<T, String>` for error handling

**IPC Type Safety Pattern from `docs/IPC_PATTERN.md`**
- Create paired TypeScript types in `src/types/commands.ts` matching Rust command signatures exactly
- Implement type-safe invoke wrapper functions: `export async function invokeSyncEmails(accountId: number): Promise<SyncStats>`
- Use Tauri events for background operation progress: `emit('sync_progress', { account_id, messages_synced })`
- Mock Tauri API in frontend tests using vitest with `vi.mock('@tauri-apps/api/core')`
- Document all commands with examples showing Rust implementation and TypeScript usage

**Error Handling Standards from `agent-os/standards/global/error-handling.md`**
- User-friendly error messages: translate technical errors to actionable messages (e.g., "Failed to connect to mail server. Check your internet connection.")
- Fail fast and explicitly: return Result<T, E> for all fallible operations, never use unwrap() or expect() in production code
- Use specific error types: define custom error enums (SyncError, SmtpError, DatabaseError) with thiserror crate
- Implement exponential backoff for transient failures: network timeouts, rate limits, temporary server errors
- Graceful degradation: continue sync on single message failures, log errors, report summary to user

**Database Best Practices from `agent-os/standards/backend/`**
- Models (models.md): define clear ownership between tables, use foreign keys with ON DELETE CASCADE, include created_at/updated_at timestamps
- Migrations (migrations.md): reversible migrations with UP and DOWN, small focused changes per file, never modify existing migrations
- Queries (queries.md): always use parameterized queries, eager load related data to prevent N+1, add indexes on foreign keys and frequently queried columns

**Code Quality Tooling from `src-tauri/Cargo.toml`**
- Clippy lints configured for strict code quality: all = "warn", pedantic = "warn", nursery = "warn", unwrap_used = "warn"
- Rustfmt configuration in `src-tauri/rustfmt.toml` for consistent formatting
- Pre-commit hooks with Husky + lint-staged for automated quality checks
- Comprehensive testing setup: cargo test for Rust, vitest for TypeScript
- Follow existing patterns to maintain consistency across codebase

## Out of Scope
- Frontend UI components for email list, reader, or composer (deferred to later phases)
- Keyboard shortcuts system and command palette (deferred to Phase 2)
- Split Inbox rules engine for automatic message categorization (deferred to Phase 3)
- Snooze emails and Send Later scheduling features (deferred to Phase 3)
- Snippets and templates for quick replies (deferred to Phase 3)
- AI-powered features: compose assistance, thread summaries, semantic search (deferred to optional phases)
- OAuth authentication flow for Google/Microsoft accounts (initial version uses username/password only)
- Contact extraction and management from email headers (deferred to Phase 4)
- Draft management UI and auto-save functionality (deferred to Phase 2)
- Email filtering and custom folder rules beyond basic folder storage (deferred to Phase 3)
