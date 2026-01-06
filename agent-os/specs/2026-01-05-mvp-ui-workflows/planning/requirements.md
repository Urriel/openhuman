# Spec Requirements: MVP UI + Workflows

## Initial Description

Phase 2 — MVP UI + workflows

App shell (sidebar / list / reader / composer).
Virtualized list (TanStack Virtual).
Conversation reader.
Composer (send, attachments).
Search (basic text).
Core actions: archive, delete, mark read/unread, star, move/folder/labels.

use shadcn-vue blocks from the website to nail the sidebard and other blocks. use vue tiptap wisiwig editor for the message make sure the message once written is in the exact format to be sent through smtp (it must be compatible with all major email clients, gmail, outlouk, etc…)

## Requirements Discussion

### First Round Questions

**Q1: App Shell Layout**
I assume we'll use a three-pane layout (sidebar with folders/accounts on left, email list in center, reader pane on right) similar to modern email clients, with the composer as an overlay/modal. Is that correct, or would you prefer a different layout (e.g., two-pane, drawer-based)?

**Answer:** Yes (three-pane layout confirmed)

**Q2: shadcn-vue Blocks**
I'm thinking we should use the sidebar-07 or similar pattern from shadcn-vue blocks for the main navigation. Should we reference specific block patterns from their website, or would you like me to identify the best matches for sidebar, list items, and reader layouts?

**Answer:** This sidebar is fine (sidebar-07 from shadcn-vue)

**Q3: Virtualized List Items**
I assume each email list item should show sender name, subject, preview text (first ~100 chars), timestamp, and indicators (unread dot, star, attachment icon). Should we also show labels/folder tags inline, or keep list items minimal for performance?

**Answer:** Keep it minimal

**Q4: Composer Position**
I'm thinking the composer should open as a modal overlay (like Gmail's compose) rather than replacing the reader pane. Should it support "minimize to dock" for multitasking, or stay as a full modal?

**Answer:** Prefer it to take the full page, not a modal (easier to handle shortcuts)

**Q5: Tiptap MIME Formatting**
I assume the Tiptap editor should output HTML that's wrapped in proper multipart/alternative MIME format (plain text fallback + HTML body) for maximum compatibility. Should we support inline images (base64 or cid attachments) in the initial version, or just file attachments?

**Answer:** (Context provided: When receiving emails from Gmail via POP3, messages with formatting are received as MIME multipart/alternative with plain text + HTML versions)

**Q5a: Plain Text Generation**
For the Tiptap composer, should we auto-generate the plain text version by stripping HTML tags, or let users edit both versions separately?

**Answer:** Auto-generate

**Q5b: Composer Mode**
Should the composer default to rich text mode (HTML) with an option to switch to plain text, or offer both modes equally?

**Answer:** Only HTML

**Q6: Search Scope**
For basic text search, I assume we're searching across sender, subject, and body text with results displayed in the existing list view. Should search highlight matches in the list, or just filter to matching emails?

**Answer:** Yes, highlight and filter

**Q7: Actions UI**
I'm thinking core actions (archive, delete, star, mark read/unread) should be accessible via keyboard shortcuts, toolbar buttons in the reader pane, and context menu on list items (right-click). Is this correct, or would you prefer a different action access pattern?

**Answer:** The keyboard shortcuts will be customizable, but for now leave it aside, we will consider this later (add it to the roadmap)

**Q8: Move/Folder/Labels**
Should we support both Gmail-style labels (multiple per email) AND traditional folders (single location), or standardize on one approach? Also, how should the UI for moving/labeling work (dropdown, command palette, drag-and-drop)?

**Answer:** Support both

**Q9: Out of Scope**
Are there any features you explicitly want to EXCLUDE from this phase? For example: email sync engine, account management UI, snooze/send-later, snippets, or AI features?

**Answer:** No (nothing explicitly excluded)

### Existing Code to Reference

No similar existing features identified for reference.

## Visual Assets

### Files Provided:
No visual assets provided.

## Requirements Summary

### Functional Requirements

#### App Shell
- Three-pane layout:
  - **Left pane:** Sidebar with folders/accounts navigation using shadcn-vue sidebar-07 pattern
  - **Center pane:** Virtualized email list (TanStack Virtual)
  - **Right pane:** Email reader/conversation view
- Full-page composer (not modal) for easier keyboard shortcut handling

#### Email List (Virtualized)
- Use TanStack Virtual for performance with large email lists
- Minimal list items showing:
  - Sender name
  - Subject
  - Preview text (~100 chars)
  - Timestamp
  - Indicators: unread dot, star, attachment icon
- No inline labels/folder tags (keep minimal for performance)

#### Conversation Reader
- Display full email content with HTML rendering
- Show conversation threads with message history
- Handle quoted replies and threading

#### Email Composer
- Full-page view (replaces main content area)
- Vue Tiptap WYSIWYG editor for HTML composition
- Rich text editing only (no plain text mode option)
- Auto-generate plain text version by stripping HTML tags
- File attachments with drag-and-drop support
- Send via SMTP with proper MIME formatting:
  - multipart/alternative structure
  - Plain text fallback (auto-generated)
  - HTML body from Tiptap output
  - Compatible with all major email clients (Gmail, Outlook, etc.)

#### Search Functionality
- Basic text search across:
  - Sender
  - Subject
  - Body text
- Filter results in email list view
- Highlight search matches in list preview text

#### Core Actions
- Archive
- Delete
- Mark read/unread
- Star/unstar
- Move to folder
- Apply labels
- Actions accessible via:
  - Toolbar buttons in reader pane
  - Context menu on list items (right-click)
  - Note: Customizable keyboard shortcuts deferred to future phase (add to roadmap)

#### Folders & Labels System
- Support both paradigms:
  - **Traditional Folders:** Single location per email (move between folders)
  - **Gmail-style Labels:** Multiple labels per email (apply/remove labels)
- UI for folder/label management (specific interaction pattern to be determined during spec creation)

### Reusability Opportunities

No existing components identified for reuse.

### Scope Boundaries

**In Scope:**
- Three-pane app shell UI
- Virtualized email list with TanStack Virtual
- Conversation reader with HTML rendering
- Full-page composer with Tiptap editor
- SMTP-compatible MIME message formatting (multipart/alternative)
- File attachment support in composer
- Basic text search with highlighting and filtering
- Core email actions (archive, delete, read/unread, star, move, label)
- Both folders and labels support

**Out of Scope (for this phase):**
- Customizable keyboard shortcuts system (deferred to future roadmap item)
- Email sync engine (separate roadmap item)
- Account management UI (separate roadmap item)
- Snooze/Send Later features (separate roadmap item)
- Snippets/templates (separate roadmap item)
- AI features (separate roadmap item)
- Inline images in composer (just file attachments for now)

**Deferred to Roadmap:**
- Customizable keyboard shortcuts system (already exists in roadmap as item #8 "Keyboard Shortcuts System" and item #15 "Settings & Preferences UI" includes keyboard shortcut configuration)

### Technical Considerations

**Frontend:**
- Vue 3 Composition API
- shadcn-vue components (specifically sidebar-07 block pattern)
- TanStack Virtual for email list virtualization
- Vue Tiptap for WYSIWYG HTML email composition
- Tailwind CSS v4 for styling

**Email Compatibility:**
- MIME multipart/alternative format for sent emails
- Auto-generated plain text fallback from HTML
- HTML output from Tiptap must be compatible with Gmail, Outlook, Apple Mail, etc.
- Proper MIME structure with appropriate Content-Type headers

**Backend Integration:**
- SMTP sending via Rust backend (lettre crate)
- SQLite storage for emails, folders, labels
- File attachment handling and storage
- Search indexing (SQLite FTS5 for full-text search)

**Performance:**
- Virtual scrolling for smooth handling of 10,000+ email list items
- 60 FPS scrolling target
- <100ms search response time
