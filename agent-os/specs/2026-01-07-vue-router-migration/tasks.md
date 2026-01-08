# Task Breakdown: Vue Router Migration

## Overview

Total Tasks: 4 task groups with 42 actionable sub-tasks

This breakdown migrates OpenHuman from state-based navigation to vue-router 4 with HTML5 history mode, improving code organization while maintaining keyboard-first workflows and split-pane layout.

## Task List

### Router Foundation

#### Task Group 1: Router Installation and Configuration

**Dependencies:** None

- [ ] 1.0 Complete router foundation setup
  - [ ] 1.1 Write 2-8 focused tests for router configuration
    - Test router instance creation with createWebHistory()
    - Test default route redirect (/ → /inbox)
    - Test folder route matching (inbox, sent, drafts, etc.)
    - Test query param preservation on navigation
    - Test scrollBehavior returns savedPosition for back/forward
    - Limit to critical router setup behaviors only
  - [ ] 1.2 Install vue-router@4 as production dependency
    - Run: `npm install vue-router@4`
    - Verify package.json updated
  - [ ] 1.3 Update TypeScript configuration
    - Add `@/router/*` path alias to tsconfig.json
    - Add `@/views/*` path alias to tsconfig.json
    - Ensure strict mode compliance
  - [ ] 1.4 Create router directory structure
    - Create `src/router/` directory
    - Create `src/views/` directory
  - [ ] 1.5 Define route configuration (src/router/routes.ts)
    - Import route types from vue-router
    - Create routes array with RouteRecordRaw type
    - Add root redirect: `{ path: '/', redirect: '/inbox' }`
    - Add folder route with dynamic matching: `/:folder(inbox|sent|drafts|favorites|archive|deleted|spam|junk)`
    - Add settings route: `/settings/accounts`
    - Include meta properties for context: `meta: { context: 'inbox' }`
    - Export routes array
  - [ ] 1.6 Create router instance (src/router/index.ts)
    - Import createRouter, createWebHistory from vue-router
    - Import routes from routes.ts
    - Create router with createWebHistory()
    - Configure scrollBehavior function (preserve savedPosition, else scroll to top)
    - Export router as default
  - [ ] 1.7 Register router in main.ts
    - Import router from @/router
    - Add `.use(router)` before `.mount('#app')`
    - Verify app compiles without errors
  - [ ] 1.8 Ensure router foundation tests pass
    - Run ONLY the 2-8 tests written in 1.1
    - Verify router instance created successfully
    - Verify routes match correctly
    - Do NOT run entire test suite

**Acceptance Criteria:**

- The 2-8 tests written in 1.1 pass
- vue-router@4 installed and registered
- TypeScript path aliases configured
- Routes defined with proper types and meta
- scrollBehavior configured for state preservation

---

### View Components Layer

#### Task Group 2: Create View Components

**Dependencies:** Task Group 1

- [ ] 2.0 Complete view components
  - [ ] 2.1 Write 2-8 focused tests for view components
    - Test InboxView renders with folder from route params
    - Test InboxView passes query params to child components
    - Test message selection updates when route query changes
    - Test SettingsView renders AccountManagement component
    - Test KeepAlive preserves InboxView state on route change
    - Limit to critical view rendering and state behaviors
  - [ ] 2.2 Create InboxView.vue (src/views/InboxView.vue)
    - Use `<script setup lang="ts">` with TypeScript
    - Import useRoute, useRouter from vue-router
    - Import FolderNavigation, EmailList, EmailReader, TopHeader, SearchBar
    - Compute current folder from route.params.folder
    - Compute selected messageId from route.query.message
    - Watch route.query changes to update EmailList selection
    - Maintain split-pane layout (list left, reader right)
    - Include TopHeader, FolderNavigation, SearchBar in template
    - Pass folder and messageId as props to child components
  - [ ] 2.3 Create SettingsView.vue (src/views/SettingsView.vue)
    - Use `<script setup lang="ts">` with TypeScript
    - Import AccountManagement component
    - Wrap AccountManagement with proper layout
    - Handle close event to navigate back using router.push()
  - [ ] 2.4 Add route transitions with Transition component
    - Wrap RouterView with `<Transition>` in App.vue (after refactor)
    - Set transition name to subtle fade (50-100ms duration)
    - Define transition CSS classes for fade effect
    - Test transition performance maintains 60 FPS
  - [ ] 2.5 Ensure view component tests pass
    - Run ONLY the 2-8 tests written in 2.1
    - Verify views render correctly with route data
    - Verify query param reactivity works
    - Do NOT run entire test suite

**Acceptance Criteria:**

- The 2-8 tests written in 2.1 pass
- InboxView combines all inbox components in split-pane layout
- SettingsView wraps AccountManagement properly
- Views read route params/query and pass to children
- Route transitions configured with 50-100ms timing

---

### Component Refactoring Layer

#### Task Group 3: Refactor Navigation Components and Composables

**Dependencies:** Task Groups 1-2

- [ ] 3.0 Complete component and composable refactoring
  - [ ] 3.1 Write 2-8 focused tests for refactored components
    - Test FolderNavigation navigates using router.push() on click
    - Test EmailList updates route query on message selection
    - Test keyboard shortcuts trigger router navigation (g→i, g→s)
    - Test command palette executes router.push() for navigation commands
    - Test App.vue renders RouterView with KeepAlive
    - Limit to critical navigation behavior changes
  - [ ] 3.2 Refactor App.vue to use RouterView
    - Remove state refs: selectedFolder, selectedMessageId, isComposing, showAccountManagement
    - Remove event handlers: handleFolderSelect, handleMessageSelect, openComposer, etc.
    - Import useRoute from vue-router
    - Compute activeRoute from route.path for AppSidebar
    - Replace conditional v-if blocks with `<RouterView>` wrapped in `<KeepAlive>` and `<Transition>`
    - Keep SidebarProvider, AppSidebar, Toaster, CommandPalette, KeyboardShortcutsHelp
    - Update keyboard shortcuts to use router.push() instead of manual state
    - Target ~30-50 lines (down from ~183)
  - [ ] 3.3 Refactor FolderNavigation.vue
    - Import useRouter from vue-router
    - Replace `emit('selectFolder')` with `router.push('/' + folder.toLowerCase())`
    - Replace `emit('compose')` with router navigation (determine behavior)
    - Use route.params.folder to determine active folder
    - Remove emits definitions
    - Maintain existing UI and styling
  - [ ] 3.4 Refactor EmailList.vue
    - Import useRoute, useRouter from vue-router
    - Replace `emit('selectMessage')` with `router.push({ query: { message: id } })`
    - Watch route.query.message to update selected state
    - Maintain TanStack Virtual scrolling performance
    - Preserve keyboard navigation (j/k) functionality
    - Remove selectMessage emit definition
  - [ ] 3.5 Refactor AppSidebar.vue
    - Replace `#` placeholder URLs with actual route paths
    - Update navigation items to use router-link or @click with router.push()
    - Use route.path to determine active menu item
    - Maintain existing sidebar UI structure
  - [ ] 3.6 Update SearchBar.vue (if needed)
    - Determine if search needs dedicated route or stays inline
    - If inline, maintain existing behavior
    - If route, implement router.push({ path: '/search', query: { q } })
  - [ ] 3.7 Refactor useKeyboardShortcuts.ts
    - Update ViewContext type to match route meta.context values
    - Accept router instance as optional parameter
    - Update context detection to derive from route.name or route.meta
    - Ensure sequential shortcuts (g→i, g→s) work with router
  - [ ] 3.8 Refactor useCommandPalette.ts
    - Import useRouter composable
    - Replace toast.info() navigation placeholders with router.push()
    - Update go-inbox command: `router.push('/inbox')`
    - Update go-sent command: `router.push('/sent')`
    - Update go-drafts command: `router.push('/drafts')`
    - Update go-archive command: `router.push('/archive')`
    - Add closePalette() after navigation commands
    - Maintain existing action commands (archive, delete, star)
  - [ ] 3.9 Update App.vue keyboard shortcuts with router
    - Register g→i shortcut: `router.push('/inbox')`
    - Register g→s shortcut: `router.push('/sent')`
    - Register g→d shortcut: `router.push('/drafts')`
    - Register g→a shortcut: `router.push('/archive')`
    - Maintain other global shortcuts (Cmd+K, /, z, etc.)
    - Remove manual folder selection logic
  - [ ] 3.10 Ensure refactored component tests pass
    - Run ONLY the 2-8 tests written in 3.1
    - Verify router navigation works in components
    - Verify keyboard shortcuts navigate correctly
    - Do NOT run entire test suite

**Acceptance Criteria:**

- The 2-8 tests written in 3.1 pass
- App.vue simplified to ~30-50 lines with RouterView
- All navigation components use router.push() instead of emits
- Keyboard shortcuts integrated with router navigation
- Command palette navigation commands use router
- Split-pane layout and virtual scrolling preserved

---

### Testing & Integration

#### Task Group 4: Test Updates and End-to-End Verification

**Dependencies:** Task Groups 1-3

- [ ] 4.0 Complete testing and verification
  - [ ] 4.1 Review tests from Task Groups 1-3
    - Review the 2-8 tests written for router foundation (Task 1.1)
    - Review the 2-8 tests written for view components (Task 2.1)
    - Review the 2-8 tests written for refactored components (Task 3.1)
    - Total existing tests: approximately 6-24 tests
  - [ ] 4.2 Analyze test coverage gaps for vue-router migration only
    - Identify critical navigation workflows lacking coverage
    - Focus on integration between router and existing features
    - Check keyboard shortcuts → router → view updates flow
    - Check command palette → router → navigation flow
    - Prioritize end-to-end navigation workflows
    - Do NOT assess entire application test coverage
  - [ ] 4.3 Write up to 10 additional strategic tests maximum
    - Test full navigation flow: folder switch → state preservation → scroll maintenance
    - Test query param flow: message selection → URL update → component update
    - Test keyboard shortcut flow: g→i → route change → inbox view renders
    - Test command palette flow: "Go to Sent" → route change → sent view renders
    - Test back/forward navigation preserves scroll position
    - Test KeepAlive preserves EmailList state on folder switch
    - Test route transitions complete within 100ms
    - Maximum 10 tests to fill critical gaps only
  - [ ] 4.4 Update existing component tests for router
    - Install @vue/test-utils if not present
    - Update test setup to include router mock with createMemoryHistory()
    - Replace emit assertions with route change assertions
    - Update component mounts to include router: `mount(Component, { global: { plugins: [router] } })`
    - Ensure tests don't break from emit removal
  - [ ] 4.5 Manual testing checklist
    - Verify all folder navigation works (inbox, sent, drafts, etc.)
    - Verify message selection updates URL query params
    - Verify scroll position preserved when switching messages
    - Verify scroll resets when switching folders
    - Verify keyboard shortcuts navigate correctly (g→i, g→s, g→d)
    - Verify command palette navigation commands work
    - Verify back/forward browser buttons work (if applicable in Tauri)
    - Verify settings/accounts route works
    - Verify transitions are 50-100ms and smooth
    - Verify virtual scrolling maintains 60 FPS
  - [ ] 4.6 Run feature-specific tests only
    - Run ONLY tests related to vue-router migration
    - Expected total: approximately 16-34 tests maximum
    - Verify all tests pass
    - Do NOT run entire application test suite
    - Fix any failing tests before proceeding
  - [ ] 4.7 Code quality checks
    - Run TypeScript type checking: `vue-tsc --noEmit`
    - Run ESLint: `npm run lint`
    - Run Prettier check: `npm run format:check`
    - Fix any linting or formatting issues
    - Ensure no TypeScript errors introduced

**Acceptance Criteria:**

- All feature-specific tests pass (approximately 16-34 tests total)
- Critical navigation workflows covered by tests
- No more than 10 additional tests added in 4.3
- Component tests updated to work with router
- Manual testing checklist completed successfully
- TypeScript, ESLint, and Prettier checks pass
- Performance maintained (60 FPS scrolling, <100ms transitions)

---

## Execution Order

Recommended implementation sequence:

1. **Router Foundation** (Task Group 1) - Install, configure, and define routes
2. **View Components Layer** (Task Group 2) - Create InboxView and SettingsView
3. **Component Refactoring Layer** (Task Group 3) - Refactor App.vue and all navigation components
4. **Testing & Integration** (Task Group 4) - Update tests and verify end-to-end workflows

## Success Criteria

The vue-router migration is complete when:

- ✅ All navigation uses router.push() instead of emits and state refs
- ✅ App.vue reduced from ~183 lines to ~30-50 lines
- ✅ Keyboard shortcuts integrated with router (g→i, g→s, etc.)
- ✅ Command palette navigation commands use router
- ✅ Split-pane layout and scroll position preserved
- ✅ Route transitions are 50-100ms and smooth
- ✅ Virtual scrolling maintains 60 FPS performance
- ✅ All tests pass (16-34 tests covering critical workflows)
- ✅ TypeScript, ESLint, and Prettier checks pass
- ✅ Manual testing checklist completed successfully

## Notes

- Keep composition (reply/forward) inline within EmailReader (no route change)
- Use query params for message selection and label filtering
- Defer settings subroutes beyond /settings/accounts
- Maintain keyboard-first design principles throughout
- Preserve existing functionality while improving code organization
