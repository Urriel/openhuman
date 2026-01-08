# Spec Requirements: Vue Router Migration

## Initial Description

Add vue-router to the app for better code organization with HTML5 mode and full migration from current state-based navigation.

**User's core requirements:**

1. Primary motivation: Better code organization
2. History mode: HTML5 mode (not hash mode)
3. Migration strategy: Full migration (replace all state-based navigation with routes)
4. Route structure: Standard structure for inbox, compose, messages, settings

## Requirements Discussion

### First Round Questions

**Q1:** I'm assuming `/message/:id` should remain in a split-pane layout (email list on left, reader on right) rather than becoming a full-screen view, keeping the inbox visible for quick j/k navigation to other messages. Is that correct, or should message routes be full-screen?

**Answer:** Yes, keep split-pane layout.

**Q2:** I'm thinking we use nested routes like `/inbox/:messageId?` where the messageId is optional, so you can view `/inbox` (no message selected) or `/inbox/123` (message 123 selected in the split pane). Should we do this, or would you prefer query params like `/inbox?message=123`?

**Answer:** `/inbox?message=123` is perfect so we keep the scroll behaviour.

**Q3:** I assume the command palette should auto-populate route-based commands (e.g., "Go to Inbox" → `/inbox`, "Go to Sent" → `/sent`) in addition to its existing action commands. Is that correct, or keep manual command definitions only?

**Answer:** Yes, auto-populate route-based commands.

**Q4:** For keyboard-first speed optimization, I'm assuming we want instant transitions (no fade/slide animations between routes) to maintain the <2s target for common actions. Should we skip animations, or would subtle transitions (50-100ms) enhance the UX?

**Answer:** Subtle transitions (50-100ms).

**Q5:** When navigating from `/inbox/123` back to `/inbox`, should we preserve scroll position and selection state using `<keep-alive>` on the InboxView component, or is a fresh list reload acceptable?

**Answer:** No, keep the scroll position (preserve state).

**Q6:** I'm thinking labels should use query parameters like `/inbox?label=work` rather than separate routes like `/label/:labelId`, since labels are filters on the inbox rather than distinct destinations. Is that the right approach?

**Answer:** Yes, query params.

**Q7:** When composing a reply or forward, should the URL be `/compose?replyTo=123` or `/compose?forwardFrom=456` to capture the context, or should reply/forward stay on the message route and open a modal composer?

**Answer:** When composing an answer or a forward it will be done within the same thread of the email (stay on message route).

**Q8:** Are there any features you want to explicitly exclude from this migration? For example, should we defer settings subroutes (like `/settings/appearance`, `/settings/shortcuts`) and just implement `/settings/accounts` for now?

**Answer:** Yes defer and use settings/accounts.

### Existing Code to Reference

**Similar Features Identified:**

- `src/composables/useKeyboardShortcuts.ts` - Existing keyboard navigation patterns to integrate with router
- `src/composables/useCommandPalette.ts` - Command palette system that needs route-based commands added
- `src/components/FolderNavigation.vue` - Current folder navigation using emits (to be replaced with router.push)
- `src/components/EmailList.vue` - Message selection logic (to be updated to use query params)
- `src/App.vue` - Current state-based navigation with refs (to be simplified)

### Follow-up Questions

No follow-up questions needed. Requirements are clear.

## Visual Assets

### Files Provided:

No visual assets provided.

### Visual Insights:

Not applicable.

## Requirements Summary

### Functional Requirements

**Core Navigation:**

- Implement vue-router 4 with HTML5 history mode (createWebHistory)
- Replace all state-based navigation (refs like `isComposing`, `showAccountManagement`) with route-based navigation
- Support folder navigation: `/inbox`, `/sent`, `/drafts`, `/favorites`, `/archive`, `/deleted`, `/spam`, `/junk`
- Support message selection via query params: `/inbox?message=123`
- Support label filtering via query params: `/inbox?label=work`
- Settings route: `/settings/accounts` (defer other settings subroutes)

**Layout & UX:**

- Maintain split-pane layout (email list left, reader right) when viewing messages
- Preserve scroll position and selection state when navigating within folder views
- Implement subtle route transitions (50-100ms) for visual polish
- Keep keyboard-first navigation (j/k, shortcuts) working seamlessly with router

**Composition Behavior:**

- Reply/Forward stays within the message thread (no separate `/compose` route)
- Composition UI appears inline within the email reader context
- New email composition may use `/compose` route (clarify if needed)

**Command Palette Integration:**

- Auto-populate route-based commands ("Go to Inbox", "Go to Sent", etc.)
- Maintain existing action commands alongside new route commands
- Use router.push() for navigation commands

**State Management:**

- Use query parameters for transient UI state (selected message, label filter)
- Preserve email list scroll position across route changes
- Consider `<keep-alive>` for folder views to maintain state

### Reusability Opportunities

**Existing Patterns to Maintain:**

- Keyboard shortcuts system (`useKeyboardShortcuts.ts`) - integrate with router navigation
- Command palette (`useCommandPalette.ts`) - add route-based commands
- Folder navigation component - replace emits with router.push()
- Email list selection - update to read/write query params instead of emits

**Components to Refactor:**

- `App.vue` - simplify from 183 lines to ~30-50 lines, remove manual routing logic
- `FolderNavigation.vue` - replace `emit('selectFolder')` with `router.push()`
- `EmailList.vue` - replace `emit('selectMessage')` with query param updates
- `SearchBar.vue` - navigate to `/search?q=query` on submit
- `AppSidebar.vue` - replace `#` URLs with actual route paths

### Scope Boundaries

**In Scope:**

- Install and configure vue-router 4
- Create router instance with HTML5 history mode
- Define routes for folders, messages (via query params), settings/accounts
- Create view components: `InboxView.vue`, `SettingsView.vue`
- Refactor `App.vue` to use `<RouterView>`
- Update all navigation components to use `router.push()` and `route.query`
- Integrate keyboard shortcuts with router
- Add route-based commands to command palette
- Implement subtle route transitions (50-100ms)
- Preserve scroll position and state for folder views
- Handle reply/forward composition within message thread (no route change)
- Update tests to work with router

**Out of Scope:**

- Settings subroutes beyond `/settings/accounts` (defer `/settings/appearance`, `/settings/shortcuts`, etc.)
- Separate `/compose` route (unless needed for new email composition - to be clarified during implementation)
- Deep linking from external sources (custom URL schemes) - future enhancement
- Route-based code splitting (lazy loading) - can be added later if needed
- Browser back/forward button handling (may not be relevant in Tauri desktop app)
- Route guards for unsaved changes (can be added later if needed)

### Technical Considerations

**Integration Points:**

- Tauri 2 desktop app with Vite dev server
- Vue 3 Composition API with `<script setup>`
- TypeScript strict mode
- Existing composables: `useKeyboardShortcuts`, `useCommandPalette`, `useEmailActions`, `useUndoStack`
- TanStack Virtual for email list (maintain performance with routing)
- shadcn-vue components for UI

**Technology Decisions:**

- Use `vue-router@4` (latest stable)
- HTML5 history mode: `createWebHistory()`
- Query params for message selection and label filtering
- No hash mode URLs
- Subtle transitions (50-100ms) via CSS or Vue transition components
- `<keep-alive>` for preserving folder view state

**Performance Considerations:**

- Maintain 60 FPS scrolling with virtual list during route transitions
- Keep route transitions under 100ms for keyboard-first speed
- Preserve email list state to avoid re-fetching data on navigation
- Ensure keyboard shortcuts remain responsive with router integration

**Design Principles Alignment:**

- **Keyboard First:** All routes accessible via keyboard shortcuts (g→i, g→s, etc.)
- **Speed Through Reduction:** Minimal route transitions (50-100ms), no heavy animations
- **Density Over Whitespace:** Maintain split-pane layout for efficient screen use
- **Local-First:** Router changes only affect UI state, not data fetching patterns

**Existing Constraints:**

- Must work within Tauri 2 desktop environment
- Path aliases: `@/` for `src/`, `@/components/*`, `@/types/*`
- TypeScript strict mode compliance
- ESLint + Prettier formatting
- Existing test suite (Vitest) must be updated
