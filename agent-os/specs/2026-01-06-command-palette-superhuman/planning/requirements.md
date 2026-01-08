# Spec Requirements: Command Palette (Superhuman-Style)

## Initial Description

Implement the command palette from superhuman:

A. Command Palette → Product DNA
Current state: Likely missing or suboptimal

Superhuman reality: The command palette (Cmd+K / Ctrl+K) IS the interface

What to do:

- Make command palette the PRIMARY navigation (not secondary)
- Fuzzy search with context awareness (different actions for inbox vs. thread view)
- ALL major actions ONLY accessible via command palette + keyboard shortcuts (not buttons)
- Examples: "Mark done", "Snooze", "Star", "New split", "Reply all", "Search"

Additional context from initial discussion:

1. **Search Integration:** Replace SearchBar with unified command palette (Cmd+K for commands, / for search)
2. **Command Set:** Full Superhuman-inspired command set (~38 commands)
3. **User Feedback:** Toast notifications for all actions with undo support
4. **Testing:** Include tests throughout implementation
5. **Label Picker:** Sub-page within command palette (nested items pattern)
6. **Undo Window:** Until next action (not time-based)
7. **Search Mode:** Email search results shown directly in command palette
8. **Snooze:** Plan architecture but defer implementation to Phase 2

## Requirements Discussion

### First Round Questions

**Q1:** I assume we need to build the **Keyboard Shortcuts System (Roadmap Item #8)** as a dependency before the command palette itself, since the palette needs to register and display all shortcuts. Should we tackle both in this spec, or split them into separate specs?

**Answer:** Yes, include keyboard shortcuts system in this spec since they're tightly coupled—the command palette IS the visual interface for discovering and executing keyboard shortcuts.

---

**Q2:** You mentioned replacing the SearchBar component with the unified command palette. I assume the current SearchBar functionality (FTS5 search) should be integrated as a **search mode** within the command palette (opened with `/` key), showing email results inline. Is that correct, or should search remain separate?

**Answer:** Yes, unified search in the command palette.

---

**Q3:** Based on our discussion, I'm planning to include the **full Superhuman-inspired command set (~38 commands)** covering:

- Email actions (archive, delete, star, mark read/unread)
- Composition (compose, reply, reply-all, forward)
- Navigation (j/k, g→i inbox shortcuts, folder navigation)
- Selection (select all, multi-select, checkboxes)
- Labels (apply/remove labels with nested picker)
- Search (integrated search mode)
- System (theme toggle, shortcuts help)
- Undo (z key, undo-until-next-action)

Should we include **Snooze architecture planning** in this spec (Roadmap Item #12) even though implementation is deferred, or completely exclude it for now?

**Answer:** Yes, plan the snooze part.

---

**Q4:** I'm assuming we'll use **shadcn-vue Toast component** for action feedback with undo buttons. The undo behavior should be "until next action" (not time-based), clearing the undo stack when the user performs any new action. Is this approach correct?

**Answer:** Yes.

---

**Q5:** For the "Apply Label" command, I'm planning a **nested sub-page within the command palette** (similar to cmdk's nested items example). When user presses `l`:

- Main commands disappear
- Label picker appears with all available labels
- User can filter labels by typing
- Backspace/Escape returns to main commands

Is this the desired UX?

**Answer:** Perfect.

---

**Q6:** Based on the design principles and tech stack performance budgets, I'm assuming these targets for the command palette:

- **Open time:** <100ms from Cmd+K press to visible
- **Fuzzy search filtering:** <100ms to show results
- **Command execution:** <1s to complete action + show toast
- **Search mode:** <200ms to display email results

Are these targets aligned with your expectations?

**Answer:** Yes.

---

**Q7:** I assume we should include:

- **Unit tests** for keyboard shortcuts composable, command registry, email actions (85%+ coverage goal)
- **E2E tests** for critical flows (open palette, execute command, undo, search mode)
- **Performance tests** to verify <100ms open time

Should we also include **accessibility testing** (screen reader compatibility, ARIA labels) as part of this spec?

**Answer:** The test coverage is good as mentioned, no need for accessibility testing.

---

**Q8:** What should we explicitly exclude from this spec to keep scope manageable? For example:

- ❌ Custom keyboard shortcut configuration (use defaults only)
- ❌ Command history / recent commands (defer to future enhancement)
- ❌ Command palette themes/customization (use app theme)
- ❌ Snooze implementation (architecture only)
- ❌ AI-powered command suggestions (future)

Anything you'd like to add to or remove from this exclusion list?

**Answer:** Yes, you are right there.

---

### Existing Code to Reference

**Similar Features Identified:**

Based on initial code analysis, the following existing code patterns should be referenced:

- **Keyboard Event Handler:** `src/components/ui/sidebar/SidebarProvider.vue` - Uses `useEventListener` from `@vueuse/core` to handle Cmd+B shortcut
- **Email Actions:**
  - `src/components/EmailReader.vue` - Contains `toggleStar()`, `archiveMessage()`, `deleteMessage()` handlers
  - `src/components/EmailList.vue` - Contains `handleBulkArchive()`, `handleBulkDelete()` handlers
  - `src/components/BulkActionsToolbar.vue` - UI for bulk actions (archive, delete, mark read/unread)
- **Search Functionality:** `src/components/SearchBar.vue` - Existing FTS5 search implementation to be replaced/integrated
- **Virtual Scrolling:** `src/components/VirtualList.vue` and `src/components/EmailList.vue` - Pattern for rendering large lists efficiently
- **Dialog Components:** shadcn-vue Dialog components already in use (see `src/components/ui/dialog/`)
- **Tauri Commands:** `src/types/commands.ts` - All IPC command definitions and type-safe wrappers

**Components to potentially reuse:**

- shadcn-vue Dialog for command palette overlay
- Existing Tauri command invocations (`invokeArchiveMessages`, `invokeDeleteMessage`, `invokeStarMessage`, etc.)
- Virtual scrolling pattern if command list becomes very large
- Search query logic from SearchBar component

**Backend logic to reference:**

- All email action commands already exist in Rust backend
- Search command (`search_messages`) already implemented with FTS5
- Label commands (`create_label`, `apply_label`, `remove_label`, etc.) already exist

---

## Visual Assets

### Files Provided:

No visual assets provided.

---

## Requirements Summary

### Functional Requirements

**Core Command Palette:**

- Open with Cmd+K (or Ctrl+K on Windows/Linux)
- Display all available commands with fuzzy search filtering
- Group commands by category (Email Actions, Composition, Navigation, Selection, Labels, Search, System, Undo)
- Show keyboard shortcuts next to each command
- Execute commands on Enter key or click
- Close on Escape key
- Context-aware command visibility (different commands in inbox vs. thread vs. compose views)

**Keyboard Shortcuts System:**

- Global keyboard shortcut manager using `@vueuse/core`
- Support single-key shortcuts (e, r, a, c, #, s, etc.)
- Support modifier combos (Cmd+K, Cmd+Enter, Shift+?)
- Support sequential keys (g→i, g→s, g→d, etc.)
- Prevent shortcuts when input fields are focused (except Escape)
- Context-based shortcut filtering (only show relevant shortcuts per view)

**Search Mode:**

- Open command palette in search mode with `/` key
- Replace existing SearchBar component
- Display email search results directly in palette
- Use existing FTS5 search backend
- Show email previews with highlighting
- Navigate results with Up/Down arrows, open with Enter

**Email Actions (via commands or shortcuts):**

- Archive (e) - Move to archive folder
- Delete (#) - Move to trash
- Star (s) - Toggle star status
- Mark as Read (Shift+I)
- Mark as Unread (Shift+U)
- Compose (c) - New email
- Reply (r) - Reply to sender
- Reply All (a) - Reply to all
- Forward (f) - Forward email
- Send (Cmd+Enter in compose)
- Refresh (Cmd+R) - Sync emails

**Navigation (via shortcuts):**

- Next Email (j) - Select next in list
- Previous Email (k) - Select previous in list
- Open Email (Enter or o) - Open selected email
- Go to Inbox (g→i)
- Go to Sent (g→s)
- Go to Drafts (g→d)
- Go to All Mail (g→a)
- Go to Starred (g→\*)
- Go to Archive (g→e)

**Selection (via shortcuts):**

- Select All (Cmd+A)
- Deselect All (Cmd+D)
- Toggle Checkbox (x)

**Labels:**

- Apply Label (l) - Opens nested label picker in command palette
- Remove Label (Shift+L) - Opens nested label remover
- Label picker shows all available labels
- Filter labels by typing
- Backspace/Escape returns to main commands

**Toast Notifications:**

- Install shadcn-vue Toast component
- Show toast for all actions (archive, delete, star, etc.)
- Include undo button in toast
- Undo available until next action (not time-based)
- Clear undo stack when new action is performed

**Undo System:**

- Undo with `z` key
- Undo last action (if available)
- Store minimal state (message IDs + undo callback)
- Only one undo action available at a time

**Keyboard Shortcuts Help:**

- Show help modal with `?` key
- Display all available shortcuts grouped by category
- Show context-specific shortcuts based on current view

**Snooze (Architecture Planning Only - Implementation Deferred):**

- Plan database schema (add `snoozed_until` and `is_snoozed` fields)
- Plan Rust commands (`snooze_message`, `unsnooze_message`)
- Plan UI flow (nested snooze picker with durations)
- Plan background worker to un-snooze messages
- Mark as "Coming Soon" in command palette
- Full implementation deferred to Phase 2

### Reusability Opportunities

**Components:**

- shadcn-vue Dialog for command palette overlay
- shadcn-vue Command components (install via CLI)
- shadcn-vue Toast for notifications
- Existing email action handlers from EmailReader.vue and EmailList.vue

**Backend Patterns:**

- Existing Tauri commands for all email actions
- Existing search_messages command with FTS5
- Existing label management commands
- Type-safe command wrappers in `src/types/commands.ts`

**Frontend Patterns:**

- `useEventListener` from `@vueuse/core` for keyboard handling (see SidebarProvider.vue)
- Virtual scrolling pattern from EmailList.vue (if needed for large command lists)
- Existing search debouncing logic from SearchBar.vue

### Scope Boundaries

**In Scope:**

- Command palette component with command mode and search mode
- Keyboard shortcuts system (global manager, single keys, modifiers, sequential)
- Full command set (~38 commands covering all email actions)
- Toast notifications with undo support
- Undo system (z key, until next action)
- Nested label picker in command palette
- Integration with existing email actions
- Replace SearchBar with unified search in palette
- j/k navigation in email list
- Keyboard shortcuts help modal (? key)
- Sequential key navigation (g→i, g→s, etc.)
- Unit tests (85%+ coverage)
- E2E tests (critical flows)
- Performance tests (<100ms open time)
- Snooze architecture planning (database, Rust commands, UI flow)

**Out of Scope:**

- Custom keyboard shortcut configuration (use defaults only)
- Command history / recent commands (defer to future)
- Command palette themes/customization (use app theme)
- Snooze implementation (architecture only, defer to Phase 2)
- AI-powered command suggestions (future)
- Accessibility testing (ARIA labels, screen readers)
- Send Later feature (future)
- Snippets & Templates (future)
- Split Inbox (future)

### Technical Considerations

**Dependencies to Install:**

- shadcn-vue Command component (via `pnpm dlx shadcn-vue@latest add command`)
- shadcn-vue Toast component (via `pnpm dlx shadcn-vue@latest add toast`)
- Already have: `@vueuse/core` (for keyboard handling)
- Already have: Tauri 2, Vue 3, TypeScript, Tailwind CSS v4

**Integration Points:**

- Replace `SearchBar.vue` component in `App.vue`
- Add `CommandPalette.vue` to `App.vue`
- Integrate with existing email actions in EmailReader.vue and EmailList.vue
- Add j/k navigation to EmailList.vue
- Provide app context to command palette (current view, selected messages, etc.)
- Use existing Tauri commands for all backend operations

**Existing System Constraints:**

- Must use Vue 3 Composition API with `<script setup>` pattern
- Must use TypeScript in strict mode
- Must use Tailwind CSS v4 for styling (no custom CSS)
- Must use shadcn-vue components for UI
- Must use path aliases (`@/` for `src/`)
- Must follow IPC pattern (type-safe Tauri commands)
- Must handle errors gracefully with user-friendly messages

**Performance Targets:**

- Command palette open time: <100ms
- Fuzzy search filtering: <100ms
- Command execution: <1s (action + toast)
- Search mode: <200ms to display results
- Keyboard shortcut response: <16ms (60 FPS)

**Technology Preferences:**

- Use `useMagicKeys` from `@vueuse/core` for keyboard handling
- Use cmdk-based Command component from shadcn-vue
- Use Vue's provide/inject for context sharing
- Use Vitest for unit tests
- Use Playwright for E2E tests

**Similar Code Patterns to Follow:**

- Keyboard event handling: See `SidebarProvider.vue` (uses `useEventListener`)
- Email actions: See `EmailReader.vue` and `EmailList.vue` (Tauri command invocations with error handling)
- Search: See `SearchBar.vue` (debounced search, FTS5 integration)
- Dialog: See existing shadcn-vue Dialog usage throughout app
- Type-safe commands: See `src/types/commands.ts` (invoke wrappers)

### Design Principles Compliance

All implementation must follow the design principles from `agent-os/product/design-principles.md`:

1. **Keyboard First, Mouse Optional** - Every action has a keyboard shortcut, command palette is PRIMARY interface
2. **Density Over Whitespace** - Show 10-15 commands per screen, compact padding (8-12px)
3. **AI as Collaborator, Not Replacement** - Not applicable for this feature
4. **Speed Through Reduction** - No confirmation dialogs, optimistic updates, undo instead of confirm, <100ms performance targets
