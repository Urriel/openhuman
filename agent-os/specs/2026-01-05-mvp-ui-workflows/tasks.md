# Task Breakdown: MVP UI + Workflows

## Overview

Total Tasks: 4 Task Groups

## Task List

### Database Layer

#### Task Group 1: Labels Schema & Commands

**Dependencies:** None

- [x] 1.0 Complete labels database layer
  - [x] 1.1 Write 2-8 focused tests for labels functionality
    - Test message_labels junction table creation and constraints
    - Test label CRUD operations (create, list, delete labels)
    - Test apply/remove label operations for messages
    - Test label count queries (messages per label)
  - [x] 1.2 Create migration for message_labels table
    - Junction table with message_id and label_id foreign keys
    - Unique constraint on (message_id, label_id) to prevent duplicates
    - Indexes on message_id and label_id for fast lookups
    - Add cascade delete when message or label is deleted
  - [x] 1.3 Create migration for labels table
    - Fields: id, account_id, name, color (optional), created_at
    - Unique constraint on (account_id, name)
    - Foreign key to accounts table
  - [x] 1.4 Create Tauri commands for label operations
    - create_label(account_id, name, color) -> label_id
    - list_labels(account_id) -> Vec<Label>
    - delete_label(label_id)
    - apply_label(message_id, label_id)
    - remove_label(message_id, label_id)
    - get_message_labels(message_id) -> Vec<Label>
  - [x] 1.5 Create Tauri command for archive operation
    - archive_messages(message_ids) -> updates messages.folder to "Archive"
    - Batch support for multiple message_ids
  - [x] 1.6 Ensure database layer tests pass
    - Run ONLY the 2-8 tests written in 1.1
    - Verify migrations run successfully
    - Test label CRUD and message association

**Acceptance Criteria:**

- The 2-8 tests written in 1.1 pass
- Migrations create message_labels and labels tables correctly
- Label operations (create, apply, remove) work via Tauri commands
- Archive command updates folder field to "Archive"

### Frontend Core UI Components

#### Task Group 2: Three-Pane App Shell & Sidebar

**Dependencies:** None (can run parallel with Task Group 1)

- [x] 2.0 Complete app shell and sidebar
  - [x] 2.1 Write 2-8 focused tests for app shell components
    - Test AppShell layout renders three panes correctly
    - Test Sidebar renders folders and labels lists
    - Test sidebar collapse/expand functionality
    - Test navigation between folders/labels updates active state
  - [x] 2.2 Install and configure Vue Tiptap
    - Add @tiptap/vue-3 and required extensions (StarterKit, Link)
    - Configure Tiptap with bold, italic, underline, lists, links
    - Create utility function to strip HTML tags for plain text generation
  - [x] 2.3 Create AppShell.vue component
    - Three-pane flexbox layout (sidebar, list, reader)
    - Responsive column widths (sidebar: 240px, list: flexible, reader: flexible)
    - Route slots for sidebar, email list, and reader/composer content
    - State management for active view (list vs composer)
  - [x] 2.4 Create Sidebar.vue component using shadcn-vue sidebar-07 pattern
    - Folder list section (INBOX, Sent, Archive, Trash, custom folders)
    - Label list section with color indicators
    - Fetch folders from folders table via Tauri command
    - Fetch labels from labels table via list_labels Tauri command
    - Display message count for each folder/label
    - Collapsible sidebar toggle button
  - [x] 2.5 Create FolderItem.vue and LabelItem.vue components
    - Clickable items with active state styling
    - Display folder/label name and message count
    - Icon for folder/label type
    - Use shadcn-vue Button component for styling
  - [x] 2.6 Ensure app shell tests pass
    - Run ONLY the 2-8 tests written in 2.1
    - Verify three-pane layout renders
    - Verify sidebar shows folders and labels

**Acceptance Criteria:**

- The 2-8 tests written in 2.1 pass
- AppShell displays three-pane layout correctly
- Sidebar shows folders and labels with message counts
- Tiptap is installed and configured
- Sidebar collapse/expand works

### Frontend Email List & Reader

#### Task Group 3: Virtualized List & Conversation Reader

**Dependencies:** Task Group 1 (for archive command), Task Group 2 (for app shell)

- [x] 3.0 Complete email list and reader
  - [x] 3.1 Write 2-8 focused tests for list and reader components
    - Test EmailList renders using VirtualList.vue with email items
    - Test EmailListItem displays sender, subject, preview, timestamp, indicators
    - Test EmailReader displays HTML content (sanitized)
    - Test EmailReader shows attachments list
    - Test action toolbar buttons invoke correct Tauri commands
  - [x] 3.2 Create EmailList.vue component
    - Integrate existing VirtualList.vue component with email data
    - Fetch messages from SQLite via new list_messages Tauri command
    - Props: itemHeight (80px), containerHeight (full height), account_id filter
    - Click handler to select email and open in reader pane
    - Multi-select support (Shift+Click, Ctrl+Click) for batch operations
  - [x] 3.3 Create EmailListItem.vue slot component
    - Display sender name (from from_addr)
    - Display subject (bold if unread)
    - Display preview text (~100 chars from body_plain or snippet)
    - Display timestamp (formatted relative time)
    - Visual indicators: unread dot, star icon, attachment icon
    - Highlight selected state with background color
    - Minimal styling for performance (Tailwind utility classes)
  - [x] 3.4 Create EmailReader.vue component
    - Display selected email's full content
    - Render body_html with sanitization (call sanitize_html or use DOMPurify.js)
    - Display body_plain as fallback if no HTML
    - Show email headers: From, To, Cc, Subject, Date
    - Display conversation thread messages (query by thread_id)
    - Attachments list with filename, size, download button
  - [x] 3.5 Create EmailReaderToolbar.vue component
    - Action buttons: Archive, Delete, Star/Unstar, Mark Read/Unread
    - Move to Folder dropdown (list folders, call Tauri to update folder)
    - Apply Labels multi-select dropdown (list labels, call apply_label/remove_label)
    - Invoke existing Tauri commands: archive_messages, delete_message, star_message, mark_read
    - Batch action support when multiple emails selected in list
  - [x] 3.6 Create Tauri command list_messages
    - Query messages table with filters: account_id, folder, is_read, is_starred
    - Return Vec<MessageListItem> with id, subject, from_addr, preview, date, is_read, is_starred, has_attachments
    - Order by date DESC for inbox view
    - Limit to 10,000 messages for performance
  - [x] 3.7 Ensure list and reader tests pass
    - Run ONLY the 2-8 tests written in 3.1
    - Verify email list renders with VirtualList
    - Verify reader shows email content and toolbar

**Acceptance Criteria:**

- The 2-8 tests written in 3.1 pass
- EmailList displays emails using VirtualList with smooth scrolling
- EmailListItem shows all required fields and indicators
- EmailReader displays email content with sanitized HTML
- Toolbar actions (archive, delete, star, read/unread, folder, labels) work correctly
- Batch operations work for multi-selected emails

### Frontend Composer & Search

#### Task Group 4: Email Composer & Search UI

**Dependencies:** Task Group 2 (for Tiptap), Task Group 3 (for app shell integration)

- [x] 4.0 Complete composer and search
  - [x] 4.1 Write 2-8 focused tests for composer and search
    - Test EmailComposer renders Tiptap editor
    - Test composer sends email via send_email Tauri command
    - Test plain text auto-generation from HTML
    - Test file attachment upload and display
    - Test SearchBar filters email list via search_messages command
    - Test search results show highlighted snippets
  - [x] 4.2 Create EmailComposer.vue component (full-page view)
    - Replaces main content area when composing new email
    - Header inputs: To, Cc, Bcc (comma-separated), Subject (use Input.vue)
    - Tiptap WYSIWYG editor for HTML body composition
    - Rich text toolbar: Bold, Italic, Underline, BulletList, OrderedList, Link
    - Auto-generate body_plain by stripping HTML tags from Tiptap output
    - "Compose New Email" button in app shell to open composer
  - [x] 4.3 Create AttachmentUpload.vue component
    - Drag-and-drop file upload zone
    - File input button for manual selection
    - Display uploaded files list with filename, size, remove button
    - Store file data temporarily (FormData or base64) until send
    - Send attachments to backend (create save_attachment Tauri command if needed)
  - [x] 4.4 Implement send email flow
    - Collect To/Cc/Bcc recipients, Subject, Tiptap HTML body
    - Auto-generate plain text by stripping HTML (remove tags, keep text)
    - Call send_email Tauri command with SendEmailParams
    - Show loading state on Send button
    - Display success/error message after send
    - Close composer and return to email list on success
  - [x] 4.5 Create SearchBar.vue component
    - Input field in app shell header for search query
    - Call search_messages Tauri command on input change (debounced 300ms)
    - Pass account_id filter if specific account selected in sidebar
    - Update EmailList to display search results instead of inbox
    - Clear search button to return to normal inbox view
  - [x] 4.6 Implement search result highlighting
    - Use snippet field from search_messages response (contains `<mark>` tags)
    - Render snippet in EmailListItem preview text with HTML (v-html)
    - Highlight matches in sender and subject if included in search results
    - Style `<mark>` tags with yellow background color
  - [x] 4.7 Ensure composer and search tests pass
    - Run ONLY the 2-8 tests written in 4.1
    - Verify composer sends emails correctly
    - Verify search filters and highlights results

**Acceptance Criteria:**

- The 2-8 tests written in 4.1 pass
- EmailComposer displays full-page with Tiptap editor
- Composer sends emails with proper MIME multipart/alternative format
- Plain text is auto-generated from HTML
- File attachments upload and send correctly
- SearchBar filters email list via FTS5 search
- Search results show highlighted snippets with `<mark>` tags

### Testing & Integration

#### Task Group 5: End-to-End Testing & Polish

**Dependencies:** Task Groups 1-4

- [x] 5.0 Complete integration testing and polish
  - [x] 5.1 Review tests from Task Groups 1-4
    - Review database tests from Task 1.1
    - Review app shell tests from Task 2.1
    - Review list/reader tests from Task 3.1
    - Review composer/search tests from Task 4.1
    - Total existing tests: approximately 8-32 tests
  - [x] 5.2 Analyze test coverage gaps for MVP UI + Workflows
    - Identify critical user workflows lacking coverage
    - Focus on end-to-end flows: compose → send, search → read, list → archive
    - Check integration between components (sidebar → list → reader → composer)
  - [x] 5.3 Write up to 10 additional integration tests
    - Test complete compose and send workflow (To/Subject/Body → Send → Outbox)
    - Test complete search workflow (Query → Results → Open Email)
    - Test complete archive workflow (Select → Archive → Folder Updates)
    - Test complete label workflow (Select → Apply Label → Label List Updates)
    - Test multi-select batch operations (Select 5 → Archive All)
    - Test folder navigation (Click Folder → List Filters)
    - Test thread display (Open Email → Show Thread Messages)
  - [x] 5.4 UI/UX polish and responsive design
    - Ensure three-pane layout is responsive (min-width checks)
    - Add loading states for async operations (search, send, fetch messages)
    - Add empty states (no emails, no search results, no labels)
    - Add error handling UI (failed send, failed search)
    - Verify Tailwind CSS v4 styling consistency across all components
  - [x] 5.5 Performance optimization
    - Verify VirtualList scrolling at 60 FPS with 10,000+ items
    - Test search response time < 100ms for typical queries
    - Optimize re-renders (use Vue's memo, v-once where appropriate)
    - Test email reader with large HTML emails (sanitization performance)
  - [x] 5.6 Run feature-specific tests
    - Run ALL tests from Task Groups 1-5 (approximately 18-42 tests total)
    - Verify all critical workflows pass
    - Fix any failing tests or identified bugs
    - Do NOT run entire application test suite (only MVP UI + Workflows tests)

**Acceptance Criteria:**

- All feature-specific tests pass (approximately 18-42 tests total)
- Critical user workflows tested end-to-end
- UI is polished with loading states, empty states, error handling
- Performance targets met (60 FPS scrolling, <100ms search)
- Responsive design works across screen sizes
- No more than 10 additional tests added in 5.3

## Execution Order

Recommended implementation sequence:

**Phase 1: Foundation (Parallel)**

1. Database Layer (Task Group 1) - Labels schema and commands
2. Frontend Core UI (Task Group 2) - App shell, sidebar, Tiptap setup

**Phase 2: Core Features (Sequential)** 3. Email List & Reader (Task Group 3) - Depends on Task Groups 1 & 2 4. Composer & Search (Task Group 4) - Depends on Task Groups 2 & 3

**Phase 3: Integration (Sequential)** 5. Testing & Polish (Task Group 5) - Depends on Task Groups 1-4

**Notes:**

- Task Groups 1 and 2 can be developed in parallel (different engineers)
- Task Group 3 requires database commands (Group 1) and app shell (Group 2)
- Task Group 4 requires Tiptap setup (Group 2) and app shell integration (Group 3)
- Task Group 5 tests and polishes all previous work
