# Spec Requirements: IMAP Migration with Full Support

## Initial Description

Migrate the OpenHuman email client from POP3 protocol to IMAP protocol with full IMAP support.

### Context

Currently, the application uses POP3 (Post Office Protocol 3) for receiving emails. IMAP (Internet Message Access Protocol) is more modern and feature-rich, providing capabilities that POP3 doesn't support.

### Core Requirements from User

1. **Replace all POP3 references with IMAP** throughout the codebase
2. **Update default port** from 995 (POP3S) to 993 (IMAPS)
3. **Update all documentation** including README, specs, and code comments
4. **Implement full IMAP support** with the following features:
   - Multi-folder synchronization (INBOX, Sent, Drafts, Archive, custom folders)
   - Server-side flag syncing (read/unread, starred across devices)
   - Folder management (create, delete, rename)
   - Message move/copy operations
   - UID-based incremental sync with UIDVALIDITY checking
   - IDLE support for real-time push notifications (Phase 1 requirement)

### Technical Context

- **Current Implementation:** Uses `async-pop3` crate for POP3 protocol
- **Target Implementation:** Will use `async-imap` crate for IMAP protocol
- **Database:** SQLite with sqlx ORM
- **Language:** Rust backend, TypeScript/Vue frontend
- **Architecture:** Tauri 2 desktop application

## Requirements Discussion

### First Round Questions

**Q1: Existing POP3 users migration path**
I'm assuming existing users who have POP3-configured accounts will need to manually reconfigure their accounts with new IMAP settings (host change from `pop.provider.com` to `imap.provider.com` and port from 995→993). Should we provide an in-app migration wizard that guides them through this, or just show a notification with migration instructions linking to documentation?

**Answer:** No existing users on the product. Product has not been distributed yet. No migration needed.

**Q2: Database migration safety**
I'm thinking we should create a database backup before running the migration (automatic backup to `~/.openhuman/backups/` with timestamp). Should we also add a "rollback" option in case the migration fails, or is automatic backup sufficient?

**Answer:** No need for user migration considerations since there are no existing users.

**Q3: IDLE connection management**
For IDLE real-time notifications, I assume we should maintain one IDLE connection per account (prioritizing INBOX folder) and fall back to polling for other folders. Should IDLE be always-on by default, or should it be a user preference ("Enable real-time notifications") that they can toggle in settings?

**Answer:** IDLE should be always-on by default.

**Q4: Folder sync priority and throttling**
Based on your decision to prioritize INBOX (5 min) vs other folders (30 min), should we also add intelligent throttling - for example, pause syncing non-INBOX folders when the user is actively reading/composing emails to preserve system resources?

**Answer:** No need for intelligent throttling.

**Q5: Multi-folder UI navigation**
With IMAP supporting multiple folders (Sent, Drafts, Archive, Trash, custom folders), should we add a folder sidebar/tree navigation in the main UI, or keep the current single-view approach and access folders via the command palette (Cmd+K → "Switch to Sent") to maintain the keyboard-first philosophy?

**Answer:** There is already a sidebar (AppSidebar.vue). The behavior needed is to hide it and only expand when clicking the expand button at the top. Default the page to showing inbox emails.

**Q6: Sent folder handling**
When users send emails via SMTP, should we automatically save a copy to their IMAP "Sent" folder on the server (requires IMAP APPEND command), or just store locally in our database? Gmail and Outlook automatically do this server-side, but it requires additional IMAP commands.

**Answer:** Behave like Outlook and Gmail - save sent messages to IMAP Sent folder on server.

**Q7: Draft syncing strategy**
Should we sync the "Drafts" folder bidirectionally (local draft saves go to server, server drafts come down), or keep drafts local-only? This affects multi-device usage—if someone starts a draft on desktop and wants to continue on another device.

**Answer:** Yes, please implement bidirectional draft syncing.

**Q8: Error recovery for UID validity changes**
When IMAP UIDVALIDITY changes (rare but happens when server rebuilds a mailbox), we need to re-sync the entire folder. Should we prompt the user before re-downloading potentially thousands of messages, or automatically do it in the background with a notification?

**Answer:** Do it in the background without requiring too much resources to do so.

**Q9: Bandwidth and storage considerations**
With full immediate sync of all folders, users with large mailboxes (10,000+ messages across multiple folders) might face long initial sync times. Should we add a "Sync last N days only" option (like 30, 90, 180 days, or "All time") as a user preference, or keep it simple with the current 30-day default for all folders?

**Answer:** 30 days default. Once done, try to sync the rest in the background, 30 days at a time.

**Q10: Feature exclusions**
Are there any IMAP features we should explicitly NOT implement in Phase 1? For example: IMAP CONDSTORE/QRESYNC (advanced sync optimization), IMAP NOTIFY (extension to IDLE), server-side threading (THREAD command), or IMAP COMPRESS (bandwidth optimization)?

**Answer:** No exclusions - implement all standard IMAP features needed for full functionality.

### User Configuration Decisions

Based on requirements discussion:

1. **Sync Frequency:** Prioritize INBOX (every 5 minutes), other folders sync every 30 minutes
2. **Folder Selection:** Sync all discovered folders by default
3. **Body Fetching:** Immediate full sync (download complete messages during sync)
4. **IDLE Support:** Include in Phase 1, always-on by default for real-time email notifications
5. **Search Strategy:** Local search only (keep existing FTS5, no server-side search)

### Existing Code to Reference

**Similar Features Identified:**

- **Sidebar Component:** `src/components/AppSidebar.vue` - Already has collapsible sidebar with Mail folders (Inbox, Sent, Drafts, Archive, Trash) and Labels sections. This should be extended to show IMAP folders dynamically.

- **POP3 Client:** `src-tauri/src/email/pop3.rs` - Current email fetching implementation to be replaced with IMAP client

- **SMTP Client:** `src-tauri/src/email/smtp.rs` - Email sending implementation (will need integration with IMAP APPEND for Sent folder)

- **Sync Orchestrator:** `src-tauri/src/email/sync_orchestrator.rs` - Background sync management, needs update for multi-folder IMAP sync

- **Account Management:** `src-tauri/src/commands/account_management.rs` - Form and commands for managing email accounts, needs POP3→IMAP field updates

- **Database Migrations:** `src-tauri/migrations/` - Existing migration pattern to follow for schema updates

- **Error Handling:** `src-tauri/src/error.rs` - Custom error types pattern to extend for IMAP errors

- **Retry Logic:** `src-tauri/src/retry.rs` - Exponential backoff implementation to reuse

## Visual Assets

### Files Provided:

No visual assets provided.

### Visual Insights:

No visual analysis needed - leveraging existing sidebar UI pattern.

## Requirements Summary

### Functional Requirements

#### Core Protocol Migration

- Replace POP3 protocol with IMAP throughout codebase
- Update all references: database columns, type definitions, UI labels, documentation
- Change default port from 995 (POP3S) to 993 (IMAPS)
- Replace `async-pop3` dependency with `async-imap` crate

#### Multi-Folder Synchronization

- Discover and sync all IMAP folders automatically (INBOX, Sent, Drafts, Archive, Trash, custom folders)
- Display folders dynamically in existing AppSidebar component
- Default view: INBOX
- Sync priority: INBOX every 5 minutes, other folders every 30 minutes
- Full message download (immediate body fetch, not headers-only)

#### Bidirectional Server Sync

- **Read/Unread flags:** Mark read locally → update server, server changes → update local
- **Starred/Flagged:** Star locally → update server, server changes → update local
- **Sent messages:** Save to server's Sent folder via IMAP APPEND after SMTP send
- **Drafts:** Bidirectional sync - local drafts save to server, server drafts download locally
- **Message moves:** Move between folders updates both local DB and server
- **Deletions:** Delete locally → mark as \Deleted on server + EXPUNGE

#### Real-time Notifications (IDLE)

- Maintain IDLE connection per account (prioritize INBOX folder)
- Always-on by default (no user toggle)
- Instant notification when new mail arrives
- Fallback to polling if IDLE not supported by server
- Trigger immediate sync on IDLE notification

#### Incremental Sync Strategy

- UID-based sync with UIDVALIDITY tracking per folder
- Initial sync: Last 30 days of messages
- Background historical sync: Fetch older messages 30 days at a time in background
- Handle UIDVALIDITY changes: Full folder re-sync in background without user intervention
- Resource-conscious: Background syncs shouldn't impact active user operations

#### Folder Management

- Detect folder types automatically (INBOX, Sent, Drafts, Trash, Archive)
- Support custom user-created folders
- Create, delete, rename folders via IMAP commands (future enhancement, not Phase 1)

#### Error Handling & Recovery

- Graceful IDLE reconnection on connection drops
- Background UIDVALIDITY re-sync without blocking user
- Exponential backoff retry for transient failures
- Clear error messages for auth failures
- Preserve local data on sync failures

### Reusability Opportunities

**Components/Patterns to Reuse:**

- **AppSidebar.vue** - Extend existing folder structure to be dynamic based on IMAP folder discovery
- **Sync orchestrator pattern** - Adapt existing POP3 sync logic for multi-folder IMAP
- **Error handling patterns** - Extend existing error types for IMAP-specific errors
- **Retry logic** - Reuse exponential backoff from existing retry.rs
- **Database migration pattern** - Follow existing migration file structure
- **Account management UI** - Update form fields from POP3 to IMAP (labels, ports, validation)

**Backend Patterns to Follow:**

- Command structure from `src-tauri/src/commands/greet.rs`
- Error types from `src-tauri/src/error.rs` (thiserror pattern)
- Database queries from `src-tauri/src/db.rs`
- Async client pattern from `src-tauri/src/email/smtp.rs`

### Scope Boundaries

**In Scope (Phase 1):**

- Complete POP3 to IMAP migration (all code, types, docs)
- Multi-folder sync (all discovered folders)
- Bidirectional flag/state sync (read, starred, deleted)
- IDLE real-time notifications (always-on)
- Sent folder server-side save (IMAP APPEND)
- Draft folder bidirectional sync
- Incremental sync with UID tracking
- Background historical sync (30 days at a time)
- UIDVALIDITY change handling (background re-sync)
- Dynamic folder display in existing sidebar
- Local FTS5 search (no changes needed)

**Out of Scope (Future Enhancements):**

- User-initiated folder creation/deletion/rename UI
- Per-folder sync frequency customization
- IMAP CONDSTORE/QRESYNC optimization
- IMAP COMPRESS bandwidth optimization
- Server-side search (IMAP SEARCH command)
- IMAP THREAD server-side threading
- Selective folder sync (user choosing which folders to sync)
- IDLE for non-INBOX folders (polling only for Phase 1)

### Technical Considerations

**Integration Points:**

- Database schema changes: Rename `pop3_host/port` → `imap_host/port`, add IMAP-specific fields
- TypeScript types: Update all command interfaces in `src/types/commands.ts`
- Rust types: Update structs in account_management.rs and related files
- SMTP integration: Add IMAP APPEND after successful send
- Sidebar: Make folder list dynamic based on IMAP folder discovery

**Technology Stack:**

- **IMAP Library:** `async-imap` crate (replace `async-pop3`)
- **TLS:** `async-native-tls` for secure IMAP connections
- **Database:** Existing SQLite + sqlx (add new columns/tables)
- **Async Runtime:** Existing tokio setup

**Performance Requirements:**

- IDLE connection: Low resource overhead, reconnect on drop
- Background sync: Non-blocking, resource-conscious
- Multi-folder sync: Parallel folder fetching where possible
- Database queries: <50ms for inbox list (existing requirement)

**Architectural Constraints:**

- Follow existing IPC pattern (docs/IPC_PATTERN.md)
- Maintain keyboard-first design philosophy
- Local-first architecture (all data in local SQLite)
- Provider-agnostic (work with any IMAP server)

### IMAP Bidirectional Sync Clarification

**User Question:** "So IMAP protocol allows me to sync changes between my client and the email server right? No need for a sync mechanism on my end to do so right?"

**Answer:** IMAP enables bidirectional sync, but we still need a client-side sync mechanism:

**What IMAP Provides:**

- Server stores authoritative state (flags, folders, UIDs)
- Server tracks message changes
- Server notifies of changes (via IDLE or polling)
- Changes persist across devices

**What We Must Implement:**

1. **Fetch changes FROM server** - Download new messages, updated flags, folder changes
2. **Push changes TO server** - Send IMAP commands when user marks read, stars, moves, deletes
3. **Reconcile conflicts** - Handle cases where both client and server changed same message
4. **Track sync state** - Remember what we've synced (UIDs, UIDVALIDITY per folder)
5. **IDLE management** - Maintain connection, handle reconnects, trigger syncs on notifications
6. **Background sync orchestration** - Periodic polling for non-INBOX folders, historical sync

**Example Bidirectional Flow:**

```
User marks email as read in our app:
  1. Update local SQLite immediately (instant UI feedback)
  2. Send IMAP STORE command to server in background
  3. Server updates its state
  4. Other devices fetch the change on next sync

User marks read on phone (Gmail app):
  1. Gmail app updates server
  2. Our app receives IDLE notification OR polls
  3. Fetches updated flags from server
  4. Updates local SQLite
  5. UI reflects the change
```

**Key Point:** IMAP makes bidirectional sync much easier than POP3 (which had no server-side state), but we still need to implement the sync orchestration, state tracking, and command sending logic.
