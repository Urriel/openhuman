# Specification: IMAP Migration with Full Support

## Goal

Migrate OpenHuman email client from POP3 to IMAP protocol, enabling multi-folder synchronization, bidirectional server-side state sync, real-time notifications via IDLE, and multi-device email management while maintaining the keyboard-first, local-first architecture.

## User Stories

- As a power user, I want my email state (read/unread, starred) to sync across devices so that changes I make on one device appear everywhere
- As a professional managing multiple accounts, I want access to all my email folders (Sent, Drafts, Archive) not just INBOX so that I can manage my complete mailbox
- As an active email user, I want instant notifications when new emails arrive so that I can respond immediately to important messages

## Specific Requirements

**Database Schema Migration**

- Rename `pop3_host` column to `imap_host` and `pop3_port` to `imap_port` in accounts table
- Change default port from 995 to 993 in migration and all new account forms
- Add IMAP-specific columns to sync_state: `uid_validity` (INTEGER), `uid_next` (INTEGER), rename `uidl_mappings` to `uid_mappings`
- Add IMAP columns to messages table: `imap_uid` (INTEGER), `imap_flags` (TEXT for \Seen, \Flagged, etc.)
- Add IMAP columns to folders table: `folder_type` (TEXT), `selectable` (INTEGER), `flags` (TEXT), `uidvalidity` (INTEGER), `uidnext` (INTEGER)
- Create index on messages for IMAP UID lookups: `idx_messages_imap_uid` on (account_id, folder, imap_uid)
- Follow existing migration pattern from `src-tauri/migrations/` with numbered files and reversible changes

**IMAP Client Implementation**

- Replace `async-pop3` dependency with `async-imap` and `async-native-tls` in Cargo.toml
- Create new `src-tauri/src/email/imap.rs` module replacing pop3.rs structure
- Implement connection with TLS on port 993, authentication, and session management
- Implement folder operations: LIST (discover folders), SELECT (choose folder), STATUS (get metadata)
- Implement message fetching: FETCH with UIDs for incremental sync, support for FLAGS, BODY, ENVELOPE
- Implement flag updates: STORE command to set \Seen, \Flagged, \Deleted flags on server
- Implement IDLE support for real-time push notifications from INBOX folder
- Implement graceful reconnection on connection drops with exponential backoff using existing retry.rs pattern

**Multi-Folder Synchronization**

- Discover all IMAP folders on account connection using LIST command and store in folders table
- Auto-detect standard folder types: INBOX, Sent, Drafts, Trash, Archive based on folder attributes and names
- Sync INBOX every 5 minutes, all other folders every 30 minutes using separate timers
- Implement UID-based incremental sync per folder with UIDVALIDITY checking to detect server mailbox rebuilds
- Initial sync fetches last 30 days of messages per folder, then background historical sync fetches 30 days at a time going backward
- Handle UIDVALIDITY changes by triggering full folder re-sync in background without blocking user or prompting
- Parallel folder sync where possible using tokio::spawn for each folder, with INBOX prioritized

**Bidirectional State Sync**

- When user marks message read locally: update local SQLite immediately, send IMAP STORE +FLAGS (\Seen) to server in background
- When user stars message locally: update local SQLite immediately, send IMAP STORE +FLAGS (\Flagged) to server
- When user deletes message locally: update local SQLite, send IMAP STORE +FLAGS (\Deleted) then EXPUNGE to server
- When user moves message between folders: use IMAP COPY to destination folder, then mark \Deleted and EXPUNGE from source
- Poll server for flag changes on non-INBOX folders every 30 minutes, apply updates to local SQLite and refresh UI
- Handle conflicts by treating server state as authoritative (server wins on flag conflicts)

**IDLE Real-Time Notifications**

- Maintain one IDLE connection per account on INBOX folder, always-on by default (no user toggle)
- Use async-imap IDLE handle to block waiting for server notifications of new messages or flag changes
- On IDLE notification, immediately trigger sync for that account's INBOX folder
- Implement IDLE reconnection on connection drop or timeout (29 minutes per RFC) with exponential backoff
- Fall back to polling every 5 minutes if server doesn't support IDLE (check CAPABILITY response)
- Use tokio::spawn to run IDLE in separate task per account without blocking main thread

**Sent and Draft Folder Integration**

- After successful SMTP send via existing smtp.rs client, use IMAP APPEND to save copy to server's Sent folder
- Detect Sent folder from folder list using folder attributes or name matching (Sent, Sent Items, Sent Mail)
- Sync Drafts folder bidirectionally: local draft saves trigger IMAP APPEND to server, server drafts download on sync
- Store draft metadata in messages table with folder='Drafts' and sync to server immediately on save
- On draft send completion, remove from Drafts folder (local and server via \Deleted flag)

**TypeScript Type Updates**

- Update all interfaces in `src/types/commands.ts`: Account, AddAccountRequest, UpdateAccountRequest, TestConnectionRequest
- Rename `pop3_host` → `imap_host`, `pop3_port` → `imap_port` in all type definitions
- Update TestConnectionResult: `pop3_success` → `imap_success`, `pop3_error` → `imap_error`
- Add new IMAP-specific types: ImapFolder (name, folder_type, selectable, flags, counts), ImapSyncState, ImapMessageFlag enum
- Update all wrapper functions (invokeAddAccount, invokeUpdateAccount, invokeTestAccountConnection) to use new field names

**Rust Type and Command Updates**

- Update Account struct in `src-tauri/src/commands/account_management.rs` with imap_host/imap_port fields
- Update all SQL queries in add_account, update_account, list_accounts, test_account_connection commands
- Rename Pop3Error → ImapError in `src-tauri/src/error.rs`, add IMAP-specific error variants (FolderNotFound, UidValidityChanged, InvalidFolderName)
- Update sync_orchestrator.rs to use ImapClient instead of Pop3Client, implement multi-folder sync logic
- Update all test code in db.rs and command tests to use imap_host/imap_port

**Frontend UI Updates**

- Update AccountManagement.vue form: rename pop3_host/pop3_port to imap_host/imap_port, change default port to 993
- Update all UI labels: "POP3 Settings" → "IMAP Settings", "POP3 Host" → "IMAP Host", "POP3 Port" → "IMAP Port"
- Update placeholder text: `pop.gmail.com` → `imap.gmail.com`
- Update validation error messages to reference IMAP instead of POP3
- Update TestConnectionResult display badges and error messages to show IMAP status
- Make AppSidebar.vue folder list dynamic: fetch folders from backend, display all discovered folders, default to INBOX view
- Keep sidebar collapsible behavior (hide by default, expand on button click)

**Documentation Updates**

- Update README.md to reference "IMAP email client" instead of POP3 where applicable
- Update AGENTS.md comment on line 239 (already says IMAP, verify accuracy)
- Update `agent-os/specs/2026-01-05-backend-core-provider-agnostic/spec.md` to replace all POP3 references with IMAP
- Update docs/DATABASE_SCHEMA.md with new IMAP columns and indexes
- Update docs/BACKEND_COMMANDS.md with updated command signatures
- Update inline code comments in all affected Rust and TypeScript files

## Existing Code to Leverage

**`src-tauri/src/email/pop3.rs` - POP3 Client Pattern**

- Client struct with host, port, email, connected fields provides template for ImapClient structure
- `connect()` method pattern for establishing connection and authentication should be replicated for IMAP
- `fetch_new_messages()` incremental sync logic using known UIDs can be adapted to IMAP UID-based sync
- Error handling pattern (connection checks, graceful error messages) should be maintained in IMAP implementation
- `test_login()` method provides pattern for implementing IMAP connection testing in account_management.rs

**`src-tauri/src/email/sync_orchestrator.rs` - Multi-Account Sync Pattern**

- `sync_all_accounts()` parallel task spawning with tokio::spawn can be extended for multi-folder parallel sync
- `start_scheduler()` periodic timer with tokio::time::interval provides base for INBOX (5 min) and other folders (30 min) timers
- `SyncStats` struct and return pattern should be extended to include per-folder statistics
- CancellationToken pattern for graceful shutdown should be reused for IDLE connection management
- Error handling approach (continue syncing other accounts on single account failure) applies to folder-level failures

**`src-tauri/src/error.rs` - Error Type Pattern**

- Use thiserror derive macro for custom ImapError enum replacing Pop3Error
- Follow user-friendly error message pattern: "Failed to connect to IMAP server: Check your internet connection"
- Implement From<sqlx::Error> pattern for ImapError to enable ? operator in database operations
- Maintain separation of authentication errors (no retry) vs network errors (retry with backoff)
- Follow existing pattern of specific error variants (ConnectionFailed, AuthenticationFailed, Timeout) for IMAP operations

**`src-tauri/src/retry.rs` - Exponential Backoff**

- Use `retry_with_backoff()` function for IMAP connection attempts and IDLE reconnections
- Apply `should_retry()` logic to distinguish auth failures (immediate fail) from network failures (retry)
- Default RetryConfig (max 5 attempts, 1s initial delay, exponential backoff) applies to IMAP operations
- Reuse for UIDVALIDITY change re-sync: wrap folder re-sync in retry_with_backoff to handle transient failures
- Apply same retry pattern to IMAP APPEND operations when saving sent messages and drafts to server

**`src/components/AccountManagement.vue` - Form and Validation Pattern**

- Form validation logic in `validateForm()` function should be extended for IMAP fields with same port range check (1-65535)
- `testConnection()` async function pattern should be reused for IMAP connection testing with updated field names
- Toast notification pattern for success/error feedback applies to IMAP operations
- Form state management with ref() and error tracking with `errors.value` object should be maintained
- Provider preset pattern (if implemented) can include common IMAP settings: Gmail (imap.gmail.com:993), Outlook (outlook.office365.com:993)

## Out of Scope

- User-initiated folder creation, deletion, or rename UI (IMAP commands exist but no frontend implementation)
- Per-folder sync frequency customization (users cannot override 5 min INBOX / 30 min others)
- IMAP CONDSTORE/QRESYNC extensions for optimized sync (standard UID-based sync only)
- IMAP COMPRESS extension for bandwidth optimization
- Server-side search using IMAP SEARCH command (local FTS5 search remains unchanged)
- IMAP THREAD command for server-side threading (use existing local JWZ threading algorithm)
- Selective folder sync UI (users cannot choose which folders to sync, all folders sync automatically)
- IDLE for non-INBOX folders (only INBOX gets real-time notifications, others use 30-minute polling)
- OAuth2 authentication flow for Gmail/Outlook (username/password only for this migration)
- Migration wizard for hypothetical existing POP3 users (no existing users to migrate)
