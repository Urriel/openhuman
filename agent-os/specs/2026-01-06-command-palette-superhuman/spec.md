# Specification: Command Palette (Superhuman-Style)

## Goal

Transform OpenHuman into a keyboard-first email client by implementing a Superhuman-inspired command palette (Cmd+K) that serves as the primary interface for all actions, replacing the existing SearchBar and providing unified access to ~38 commands, email search, and keyboard shortcuts across the entire application.

## User Stories

- As a power user, I want to press Cmd+K and type "archive" to archive emails instantly, so that I can process my inbox 2x faster without touching my mouse
- As a keyboard-first user, I want to press single keys like `e`, `s`, or `#` to archive, star, or delete emails, so that I can achieve inbox zero through muscle memory
- As someone managing hundreds of emails, I want to search for emails by typing `/` and see results in the command palette, so that I have a unified interface for both actions and search

## Specific Requirements

**Command Palette Component**

- Opens with Cmd+K (or Ctrl+K on Windows/Linux), closes with Escape
- Built using shadcn-vue Command component (install via CLI: `pnpm dlx shadcn-vue@latest add command`)
- Displays commands grouped by category: Email Actions, Composition, Navigation, Selection, Labels, Search, System
- Fuzzy search filters commands as user types, showing results in <100ms
- Shows keyboard shortcut next to each command (right-aligned)
- Executes command on Enter key or mouse click
- Context-aware: shows different commands based on current view (inbox vs thread vs compose)
- Performance target: opens in <100ms from keypress to visible

**Keyboard Shortcuts System**

- Global composable `useKeyboardShortcuts.ts` using `@vueuse/core`'s `useMagicKeys` and `useEventListener`
- Supports single keys (e, r, a, c, #, s, j, k, x, etc.)
- Supports modifier combos (Cmd+K, Cmd+Enter, Shift+I, Shift+U, Shift+L)
- Supports sequential keys with timeout (g→i for inbox, g→s for sent, g→d for drafts, g→a for all mail, g→\* for starred, g→e for archive)
- Prevents shortcuts when input fields focused (except Escape and global Cmd+K)
- Context filtering: shortcuts only active in relevant views (e.g., `r` for reply only in thread view)
- Pattern: follow SidebarProvider.vue's keyboard handling approach (lines 46-51)

**Search Mode Integration**

- Press `/` to open command palette in search mode
- Reuse existing FTS5 search logic from SearchBar.vue (lines 30-49)
- Display email results directly in palette with highlighting
- Show email subject, sender, and snippet in results
- Navigate results with Up/Down arrows, open with Enter
- Performance target: <200ms to display search results
- Remove SearchBar.vue component from App.vue after implementing search mode

**Full Command Set (~38 commands)**

- Email Actions: Archive (e), Delete (#), Star (s), Mark Read (Shift+I), Mark Unread (Shift+U), Refresh (Cmd+R)
- Composition: Compose (c), Reply (r), Reply All (a), Forward (f), Send (Cmd+Enter in compose view)
- Navigation: Next (j), Previous (k), Open (Enter/o), Go to Inbox (g→i), Sent (g→s), Drafts (g→d), All Mail (g→a), Starred (g→\*), Archive (g→e)
- Selection: Select All (Cmd+A), Deselect (Cmd+D), Toggle Checkbox (x)
- Labels: Apply Label (l), Remove Label (Shift+L)
- System: Command Palette (Cmd+K), Help (? shows shortcuts modal), Toggle Theme (Cmd+Shift+D)
- Undo: Undo last action (z key)

**Nested Label Picker**

- Pressing `l` in command palette pushes label picker page onto navigation stack
- Display all available labels from `invokeListLabels()` with color indicators
- Filter labels by typing (fuzzy search)
- Press Backspace or Escape to return to main commands (pop navigation stack)
- Pattern: use page stack array similar to cmdk nested items example
- Apply label via `invokeApplyLabel(messageId, labelId)` when selected

**Toast Notifications with Undo**

- Install shadcn-vue Toast component (via `pnpm dlx shadcn-vue@latest add toast`)
- Show toast for all actions: archive, delete, star, mark read/unread, apply/remove label
- Toast displays action confirmation (e.g., "Archived 3 emails") with Undo button
- Undo system in composable `useUndoStack.ts`: single undo action stored (not array)
- Undo available until next action (not time-based): new action clears previous undo
- Pressing `z` key executes undo if available, shows "Undone" toast
- Store minimal state in undo: message IDs + undo callback function

**Email Actions Integration**

- Reuse existing Tauri commands: `invokeArchiveMessages`, `invokeDeleteMessage`, `invokeStarMessage`, `invokeMarkRead`, `invokeMarkUnread` (from src/types/commands.ts)
- Follow error handling pattern from EmailReader.vue (lines 56-92): try-catch with user-friendly error messages
- Optimistic UI updates: update local state immediately, revert on error
- Performance target: command execution completes in <1s (action + toast display)

**j/k Navigation in Email List**

- Add keyboard event listeners to EmailList.vue for j (next), k (previous), Enter/o (open), x (toggle checkbox)
- Maintain selectedIndex ref to track current selection
- Scroll selected item into view automatically
- Visual focus indicator on selected email
- Only active when email list has focus (not in compose or other input fields)

**Keyboard Shortcuts Help Modal**

- Triggered by `?` key, displays modal with all shortcuts grouped by category
- Show context-specific shortcuts (e.g., different shortcuts in inbox vs thread view)
- Searchable/filterable list of shortcuts
- Built using shadcn-vue Dialog component
- Close with Escape key

**Snooze Architecture Planning (Implementation Deferred)**

- Database schema: add `snoozed_until TEXT` and `is_snoozed BOOLEAN` columns to messages table (migration deferred)
- Rust command signatures: `snooze_message(message_id: i64, until: String) -> Result<()>` and `unsnooze_message(message_id: i64) -> Result<()>` (implementation deferred)
- UI flow: pressing `h` opens nested snooze picker with durations (Later Today, Tomorrow, Next Week, Custom)
- Background worker: tokio task checks every minute for messages where `snoozed_until <= now()` and moves back to inbox (implementation deferred)
- Command palette: show "Snooze (Coming Soon)" in commands list, but mark as disabled

**Testing Requirements**

- Unit tests (Vitest) for useKeyboardShortcuts, useCommandPalette, useEmailActions, useUndoStack composables (85%+ coverage)
- E2E tests (Playwright) for: open palette with Cmd+K, search mode with `/`, execute archive command, undo with z, j/k navigation
- Performance tests: measure command palette open time (<100ms), fuzzy search (<100ms), command execution (<1s)

## Existing Code to Leverage

**SidebarProvider.vue (src/components/ui/sidebar/SidebarProvider.vue)**

- Uses `useEventListener` from `@vueuse/core` to handle global Cmd+B shortcut (lines 46-51)
- Pattern: listen for keydown event, check `event.metaKey || event.ctrlKey`, prevent default, execute action
- Reuse this pattern for Cmd+K and other keyboard shortcuts in useKeyboardShortcuts composable

**SearchBar.vue (src/components/SearchBar.vue)**

- Implements FTS5 search with debouncing (300ms timeout) using watch on query ref (lines 57-70)
- Invokes `search_messages` Tauri command with query, accountId, and limit parameters (lines 38-42)
- Shows loading spinner during search (Loader2 component with animate-spin)
- Replace this component entirely with search mode in command palette, but reuse search logic

**EmailReader.vue and EmailList.vue (email action handlers)**

- Contains action handlers: `toggleStar()`, `archiveMessage()`, `deleteMessage()` in EmailReader.vue (lines 56-92)
- Contains bulk handlers: `handleBulkArchive()`, `handleBulkDelete()` in EmailList.vue (lines 89-108)
- Pattern: try-catch around Tauri invoke, update local state on success, log error on failure
- Reuse these patterns in useEmailActions composable with added toast notifications

**Type-safe Tauri Commands (src/types/commands.ts)**

- All email action command types and wrappers already defined (invokeArchiveMessages, invokeDeleteMessage, invokeStarMessage, etc.)
- Label commands: invokeListLabels, invokeApplyLabel, invokeRemoveLabel (lines 484-502)
- Search command: invokeSearchMessages (lines 464-470)
- Reuse all type-safe wrappers in command palette and email actions composable

**shadcn-vue Dialog Components (src/components/ui/dialog/)**

- Dialog, DialogContent, DialogOverlay already available for command palette overlay
- Follow existing pattern for modal behavior: open state, focus trap, close on Escape
- Command palette will use similar structure but with CommandDialog from shadcn-vue Command component

## Out of Scope

- Custom keyboard shortcut configuration UI (users cannot rebind keys, defaults only)
- Command history or recent commands feature (defer to future iteration)
- Command palette theming or color customization (inherits app theme automatically)
- Snooze feature implementation: backend commands, database migration, background worker (architecture planned, implementation deferred to Phase 2)
- AI-powered command suggestions or auto-complete (future enhancement)
- Accessibility testing with screen readers or ARIA compliance verification (basic ARIA labels included, but no testing)
- Send Later scheduling feature (separate roadmap item)
- Email snippets and templates system (separate roadmap item)
- Split Inbox automatic categorization (separate roadmap item)
- Command palette analytics or usage tracking (privacy-first, no telemetry)
