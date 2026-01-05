# Spec Requirements: Backend Core (Provider-Agnostic)

## Initial Description
Phase 1 — Backend core (provider-agnostic)

- SQLite schema + migrations (backend-owned)
- Secrets storage (accounts/credentials)
- POP3 fetch engine
- SMTP send engine
- Message parsing (MIME) + threading model
- Sync orchestration + background jobs

## Requirements Discussion

### First Round Questions

**Q1: SQLite Schema Design** - I'm assuming we'll need tables for `accounts`, `messages`, `threads`, `folders`, and `sync_state`. Should we also include tables for `attachments`, `contacts`, and `draft_messages` in this initial schema, or defer those to later phases?

**Answer:** Stay lean - include only what is really needed first. We might need attachments though.

**Q2: Secrets Storage Strategy** - I'm thinking we'll use OS keychain integration (keyring-rs or tauri-plugin-keychain as mentioned in the tech stack) to store account credentials (username/password, OAuth tokens). Should we encrypt email content in the SQLite database as well, or just rely on filesystem encryption?

**Answer:** Rely on filesystem encryption.

**Q3: POP3 Fetch Engine - Incremental Sync** - I assume we'll track last sync timestamp per account and fetch only new messages using POP3's UIDL command. Should we support "fetch all historical emails" on first sync, or limit to last N days (e.g., 30 days)?

**Answer:** Let the user limit on the last N days. 30 max default.

**Q4: SMTP Send Engine - Queue Management** - I'm thinking we'll need a `outbox` table to queue outgoing emails with retry logic for failed sends. Should we support batch sending for multiple recipients, or process one email at a time?

**Answer:** Support batch too and the table outbox is a good guess.

**Q5: Message Parsing (MIME)** - I assume we'll extract plain text and HTML bodies, parse headers for threading (In-Reply-To, References), and handle basic attachment metadata. Should we also parse inline images and calendar invites (iCal) in this phase?

**Answer:** Yes, parse inline images and calendar invites too.

**Q6: Threading Model Algorithm** - I'm thinking we'll implement the Jamie Zawinski threading algorithm (References/In-Reply-To header matching with subject fallback). Is that correct, or do you prefer a simpler subject-based threading approach?

**Answer:** What is the common algorithm used in Gmail and Superhuman? Research shows both use the JWZ/REFERENCES algorithm (RFC 5256). Decision: Use the full algorithm.

**Q7: Sync Orchestration - Background Jobs** - I assume we'll use tokio timers to schedule periodic sync jobs (e.g., every 5 minutes) per account. Should sync be triggered manually on app launch, automatically on app launch + periodic intervals, or only periodic?

**Answer:** Automatically on app launch and then periodically.

**Q8: Migration System** - I'm thinking we'll use a custom migration system with version tracking in a `schema_migrations` table, storing numbered SQL migration files. Should we support rollback migrations, or just forward-only migrations?

**Answer:** Could we use an ORM for SQLite which handles SQLite migration? Research confirms: Yes, use SQLx - it has built-in migration support with `Migrator`, SQL migration files, `migrate!` macro, and rollback support via `undo()`.

**Q9: Error Handling & Retry Logic** - For POP3/SMTP failures (network errors, authentication failures), should we implement exponential backoff retry, or fail immediately and require user intervention?

**Answer:** Exponential backoff and then after 5 tries flag it as failed to be retried.

**Q10: Multi-Account Concurrency** - Should the sync engine process all accounts in parallel (concurrent syncs), or sequentially one account at a time to avoid resource contention?

**Answer:** Yes, all parallel.

**Q11: Scope Exclusions** - Are there any features we should explicitly exclude from this backend core phase? For example, should we defer folder/label management, search indexing setup, or read/unread tracking to later phases?

**Answer:** No (meaning include all core features - folder/label management, FTS5 search indexing, and read/unread tracking should be included).

### Follow-up Questions

**Follow-up 1: Lean Schema - Core Tables Only** - Based on your "stay lean" guidance, should our initial schema include ONLY: `accounts`, `messages`, `threads`, `outbox`, `sync_state`, and defer `folders`, `attachments` (as separate table), `contacts`, and `drafts` tables to later phases?

**Answer:** We might need attachments though.

**Follow-up 2: Sync History Limit Configuration** - Should the "last N days" limit be: a per-account setting stored in the `accounts` table, a global app preference, or configurable during initial account setup with 30 days as default?

**Answer:** A global app preference.

**Follow-up 3: JWZ Threading Implementation** - Now that we know Gmail/Superhuman use the JWZ/REFERENCES algorithm (RFC 5256), should we implement the full algorithm with Message-ID, References, In-Reply-To header parsing, dummy container nodes for missing messages, and subject-based fallback for messages without proper headers, or start simpler?

**Answer:** The full algorithm.

### Existing Code to Reference

No similar existing features identified for reference. This is a greenfield backend implementation.

## Visual Assets

### Files Provided:
No visual assets provided.

### Visual Insights:
Not applicable - this is a backend-focused specification without UI components in this phase.

## Requirements Summary

### Functional Requirements

**Database & Schema (SQLite with SQLx ORM)**
- Use SQLx as the ORM with built-in migration support
- Implement SQL migration files in `./migrations` folder with forward and rollback support
- Use `migrate!` macro for compile-time embedded migrations
- Create lean initial schema with core tables:
  - `accounts` - Store email account configuration (provider, username, sync preferences)
  - `messages` - Store complete email content with parsed MIME data (plain text, HTML, headers)
  - `threads` - Store conversation grouping based on JWZ threading algorithm
  - `attachments` - Store attachment metadata and file references
  - `outbox` - Queue outgoing emails with batch send support and retry tracking
  - `sync_state` - Track per-account sync status (last sync timestamp, UIDL mappings, error states)
  - `folders` - Store folder/label management data
- Include FTS5 (Full-Text Search) indexing setup for message content
- Include read/unread tracking fields in messages table
- All data stored locally with no cloud dependencies

**Secrets Storage**
- Use OS keychain integration via keyring-rs or tauri-plugin-keychain
- Store account credentials securely (username, password, OAuth tokens if needed)
- Support any POP3/SMTP provider (Gmail, Outlook, custom domains)
- Rely on filesystem encryption for database content (no additional encryption layer)

**POP3 Fetch Engine**
- Implement POP3 client using rust-pop3 library or custom implementation
- Use UIDL command for incremental sync (fetch only new messages)
- Track last sync timestamp per account in `sync_state` table
- Support configurable sync history limit (global app preference, default 30 days max)
- On initial sync, fetch emails from last N days (user-configurable, 30 day default)
- Handle network errors with exponential backoff (max 5 retries, then flag as failed)
- Support parallel sync across multiple accounts concurrently

**SMTP Send Engine**
- Implement SMTP client using lettre crate
- Queue outgoing emails in `outbox` table
- Support single and batch email sending
- Implement retry logic with exponential backoff (max 5 retries)
- After 5 failed attempts, flag email as "failed" for manual retry
- Format emails with proper MIME structure
- Support plain text and HTML email bodies

**Message Parsing (MIME)**
- Use mailparse or mail-parser crate for MIME parsing
- Extract and store:
  - Plain text body
  - HTML body
  - Email headers (From, To, Cc, Bcc, Subject, Date, Message-ID, References, In-Reply-To)
  - Attachment metadata (filename, size, MIME type, content reference)
  - Inline images (parse and store with attachment metadata)
  - Calendar invites (iCal/ICS parsing and storage)
- Parse all headers required for threading algorithm
- Handle malformed emails gracefully with error logging

**Threading Model (JWZ/REFERENCES Algorithm - RFC 5256)**
- Implement full Jamie Zawinski threading algorithm as used by Gmail/Superhuman
- Build parent-child relationships using Message-ID headers:
  - Parse `References` header (full ancestry chain)
  - Fall back to `In-Reply-To` header if References missing
  - Create dummy placeholder containers for missing parent messages
- Implement subject-based fallback threading:
  - Extract base subject (remove "Re:", "Fwd:", etc.)
  - Merge threads with identical base subjects
  - Case-insensitive subject comparison
- Create hierarchical conversation trees with unlimited nesting depth
- Handle missing messages, corrupted headers, and edge cases robustly
- Prevent infinite loops in threading logic

**Sync Orchestration & Background Jobs**
- Use tokio timers for job scheduling
- Trigger sync automatically on app launch
- Schedule periodic background sync (e.g., every 5 minutes) per account
- Process all accounts in parallel (concurrent syncs)
- Implement exponential backoff for sync failures:
  - Retry failed syncs up to 5 times
  - After 5 failures, mark account sync as "failed" in `sync_state`
  - Allow manual retry trigger for failed accounts
- Track sync progress and status per account
- Handle authentication failures separately (require user intervention)

**Error Handling & Retry Strategy**
- Network errors: Exponential backoff with max 5 retries
- Authentication failures: Fail immediately, flag for user intervention (no auto-retry)
- MIME parsing errors: Log error, skip message, continue sync
- Database errors: Log error, halt operation, report to user
- After 5 retry attempts: Flag operation as "failed" and require manual intervention

### Reusability Opportunities

No existing components identified for reuse. This is a greenfield backend implementation using:
- SQLx for database migrations and queries
- keyring-rs or tauri-plugin-keychain for secrets
- rust-pop3 or custom POP3 implementation
- lettre for SMTP
- mailparse or mail-parser for MIME parsing
- tokio for async runtime and timers

### Scope Boundaries

**In Scope:**
- SQLite database schema with migrations (SQLx)
- OS keychain integration for secrets storage
- POP3 fetch engine with incremental sync and UIDL tracking
- SMTP send engine with batch support and outbox queue
- Full MIME parsing (text, HTML, headers, attachments, inline images, iCal)
- JWZ/REFERENCES threading algorithm (RFC 5256) with full implementation
- Background job orchestration with tokio timers
- Automatic sync on app launch + periodic background sync
- Parallel multi-account syncing
- Exponential backoff retry logic (5 attempts max)
- Folder/label management storage
- FTS5 search indexing setup
- Read/unread tracking

**Out of Scope (Deferred to Later Phases):**
- Frontend UI for email list, reader, composer
- Keyboard shortcuts and command palette
- Split Inbox rules engine
- Snooze & Send Later scheduling
- Snippets & templates
- AI features (compose, summaries)
- Search UI and advanced query interface
- Contact extraction and management UI
- Draft management UI
- OAuth authentication flow (initial version uses username/password)

### Technical Considerations

**Technology Stack (from tech-stack.md):**
- **Language:** Rust (Tauri backend)
- **Database:** SQLite with SQLx ORM
- **Migrations:** SQLx built-in migration system
- **Email Protocols:** POP3 (rust-pop3 or custom), SMTP (lettre crate)
- **MIME Parsing:** mailparse or mail-parser crate
- **Secrets Storage:** keyring-rs or tauri-plugin-keychain
- **Background Jobs:** tokio timers and async runtime
- **Threading Algorithm:** Custom implementation of JWZ/REFERENCES (RFC 5256)

**Architecture Principles:**
- Local-first: All data in SQLite, no cloud dependencies
- Provider-agnostic: Support any POP3/SMTP provider
- Fast by default: Incremental sync, indexed search (FTS5), parallel operations
- Privacy-focused: OS keychain for secrets, filesystem encryption for data
- Resilient: Exponential backoff, error tracking, graceful degradation

**Performance Targets (from tech-stack.md):**
- Email sync: Process 1,000 emails in <10s
- Database queries: <50ms for inbox list queries
- SMTP send: <3s per email (network dependent)

**Configuration:**
- Sync history limit: Global app preference, default 30 days
- Sync interval: Configurable (default every 5 minutes)
- Retry attempts: Fixed at 5 attempts with exponential backoff
- Concurrent account syncs: All accounts in parallel

**Integration Points:**
- Tauri IPC commands for frontend-backend communication
- SQLite database as single source of truth
- OS keychain as secure credential store
- Filesystem for email attachment storage (paths in database)
