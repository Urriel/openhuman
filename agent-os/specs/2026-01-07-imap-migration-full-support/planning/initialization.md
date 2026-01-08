# Spec Initialization: IMAP Migration with Full Support

## Date

2026-01-07

## Raw Idea

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

### User Configuration Decisions

Based on requirements gathering discussion:

1. **Sync Frequency:** Prioritize INBOX (every 5 minutes), other folders sync every 30 minutes
2. **Folder Selection:** Sync all discovered folders by default
3. **Body Fetching:** Immediate full sync (download complete messages during sync)
4. **IDLE Support:** Include in Phase 1 for real-time email notifications
5. **Search Strategy:** Local search only (keep existing FTS5, no server-side search)

### Technical Context

- **Current Implementation:** Uses `async-pop3` crate for POP3 protocol
- **Target Implementation:** Will use `async-imap` crate for IMAP protocol
- **Database:** SQLite with sqlx ORM
- **Language:** Rust backend, TypeScript/Vue frontend
- **Architecture:** Tauri 2 desktop application

### Migration Impact

This is a significant architectural change affecting:

- Database schema (column renames, new IMAP-specific fields)
- Backend protocol implementation (complete rewrite of email fetching logic)
- Frontend UI (form fields, labels, placeholders)
- Type definitions (TypeScript and Rust)
- Documentation and specifications
- Existing user accounts (will require reconfiguration)

### Success Criteria

- All POP3 references replaced with IMAP
- IMAP connection working for Gmail, Outlook, and custom servers
- Multi-folder sync operational
- Real-time notifications via IDLE working
- Server-side flag changes reflected locally and vice versa
- All tests passing
- Documentation fully updated
- Migration guide provided for existing users
