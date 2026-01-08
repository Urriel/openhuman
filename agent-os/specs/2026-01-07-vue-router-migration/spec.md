# Specification: Vue Router Migration

## Goal

Migrate OpenHuman from manual state-based navigation to vue-router 4 with HTML5 history mode, improving code organization by replacing event emits and refs with declarative route-based navigation while maintaining keyboard-first workflows and split-pane layout.

## User Stories

- As a developer, I want declarative route definitions so that navigation logic is centralized and easier to maintain than scattered event handlers
- As a power user, I want URL-based navigation with query parameters for message selection so that scroll position is preserved when navigating between emails

## Specific Requirements

**Install and Configure Vue Router**

- Install `vue-router@4` as production dependency
- Create router instance with `createWebHistory()` for HTML5 mode (no hash URLs)
- Configure router with subtle transitions (50-100ms CSS transitions)
- Update `tsconfig.json` to add `@/router/*` and `@/views/*` path aliases
- Register router in `src/main.ts` before mounting app

**Define Route Structure**

- Root route `/` redirects to `/inbox`
- Folder routes use lowercase paths: `/inbox`, `/sent`, `/drafts`, `/favorites`, `/archive`, `/deleted`, `/spam`, `/junk`
- Message selection via query param: `/inbox?message=123` (not route param)
- Label filtering via query param: `/inbox?label=work`
- Settings route: `/settings/accounts` (defer other settings subroutes)
- Use dynamic route matching for all folder variations with single route definition
- Create `src/router/index.ts` for router instance and `src/router/routes.ts` for route definitions

**Create View Components**

- Create `src/views/` directory for page-level components
- Build `InboxView.vue` combining FolderNavigation, EmailList, EmailReader in split-pane layout
- Build `SettingsView.vue` wrapping AccountManagement component
- Views should read route params/query and pass to child components as props
- Maintain existing component hierarchy (TopHeader, SearchBar, etc.)
- Use query params to control which message is selected in EmailReader

**Refactor App.vue**

- Remove all manual routing refs: `selectedFolder`, `selectedMessageId`, `isComposing`, `showAccountManagement`
- Remove event handler functions: `handleFolderSelect`, `handleMessageSelect`, `openComposer`, etc.
- Simplify to only contain: SidebarProvider, AppSidebar, RouterView, and global components
- Use `useRoute()` to compute active route for AppSidebar highlighting
- Reduce App.vue from ~183 lines to ~30-50 lines
- Keep global keyboard shortcut registration but update to use router navigation

**Update Navigation Components to Use Router**

- FolderNavigation: Replace `emit('selectFolder')` with `router.push('/folder-name')`
- FolderNavigation: Replace `emit('compose')` with router navigation (determine compose behavior)
- EmailList: Replace `emit('selectMessage')` with `router.push({ query: { message: id } })`
- SearchBar: Navigate to `/search?q=query` on submit (if search gets dedicated route)
- AppSidebar: Replace `#` placeholder URLs with actual route paths
- All components must use `useRouter()` and `useRoute()` composables from vue-router

**Integrate Keyboard Shortcuts with Router**

- Update `useKeyboardShortcuts.ts` ViewContext type to match route names
- Register router navigation shortcuts in App.vue: `g→i` for `/inbox`, `g→s` for `/sent`, etc.
- Ensure keyboard navigation (j/k) continues working within EmailList component
- Maintain global shortcuts (Cmd+K, /) while adding route-based navigation
- Update ViewContext detection to derive from `route.name` or `route.meta.context`

**Add Route-Based Commands to Command Palette**

- Update `useCommandPalette.ts` to replace placeholder toast notifications with `router.push()` calls
- Navigation commands (go-inbox, go-sent, go-drafts, etc.) should use router navigation
- Add new route-based commands dynamically if needed
- Ensure command palette closes after navigation command execution
- Maintain existing action commands (archive, delete, star) alongside navigation commands

**Preserve State and Scroll Position**

- Use `<keep-alive>` wrapper in App.vue around RouterView to preserve folder view state
- Configure router scrollBehavior to maintain scroll position when using browser back/forward
- Ensure EmailList virtual scroll position is preserved when navigating via query params
- When changing folders, scroll should reset to top; when changing message selection, scroll should persist

**Handle Composition Within Thread**

- Reply and Forward actions stay on current message route (no route change)
- Composition UI renders inline within EmailReader component
- Do not create separate `/compose` route for reply/forward (composition happens in-context)
- New email composition behavior to be determined during implementation

**Update Tests for Router**

- Install vue-router testing utilities if not already present
- Update component tests to mount with router mock using `createMemoryHistory()`
- Replace emit assertions with route change assertions
- Test navigation flows: folder switching, message selection, settings access
- Ensure keyboard shortcut tests work with router integration

## Visual Design

No visual assets provided.

## Existing Code to Leverage

**useKeyboardShortcuts.ts (src/composables/useKeyboardShortcuts.ts)**

- Already supports ViewContext type ('inbox' | 'thread' | 'compose' | 'global')
- Has sequential key detection for g→i, g→s shortcuts
- Provides `register()` function for adding new shortcuts
- Update ViewContext to derive from route.name/meta instead of manual tracking
- Integrate router.push() calls into existing shortcut callbacks

**useCommandPalette.ts (src/composables/useCommandPalette.ts)**

- Contains placeholder navigation commands (go-inbox, go-sent, etc.) with toast notifications
- Replace toast.info() calls with router.push() for actual navigation
- Commands array structure at lines 48-393 ready for router integration
- Already has command execution pattern via executeCommand() function
- Maintain existing command categories and keyboard shortcuts

**FolderNavigation.vue (src/components/FolderNavigation.vue)**

- Defines FolderKey type and folders array with icons and labels
- Uses emit('selectFolder') and emit('compose') for navigation
- Replace emits with router.push() calls
- Use route.params or computed property to determine activeFolder from URL
- Maintain existing UI styling and button layout

**EmailList.vue (src/components/EmailList.vue)**

- Has message selection logic via emit('selectMessage')
- Replace emit with router.push({ query: { message: id } })
- Watch route.query.message to update selected state when URL changes
- Maintain virtual scrolling with TanStack Virtual
- Preserve keyboard navigation (j/k) functionality

**App.vue Current Structure (src/App.vue)**

- Lines 21-27: State refs to be removed (selectedFolder, selectedMessageId, isComposing, etc.)
- Lines 31-74: Event handler functions to be deleted
- Lines 87-122: Keyboard shortcut registration to be updated with router navigation
- Lines 126-181: Template with conditional v-if blocks to be replaced with RouterView
- Keep SidebarProvider, Toaster, CommandPalette, KeyboardShortcutsHelp as global components

## Vue Router Best Practices (from Context7)

**HTML5 History Mode Setup**

- Use `createWebHistory()` in Vue Router 4 (modern API, not legacy `mode: 'history'`)
- Requires server-side configuration to handle direct route access (Tauri handles this automatically)
- Cleaner URLs without hash symbols for better UX

**Programmatic Navigation**

- Use `router.push()` for navigation that adds to history stack
- Support multiple formats: string path, route object with name/params, or path with query
- Query parameters: `router.push({ path: '/inbox', query: { message: '123' } })`
- Named routes preferred for refactoring safety: `router.push({ name: 'inbox', query: { ... } })`
- Avoid mixing path with params (use name with params instead)

**Scroll Behavior Configuration**

- Implement `scrollBehavior` function in router config to control scroll on navigation
- Return `savedPosition` for back/forward navigation to preserve scroll position
- Return `{ top: 0 }` for new navigations to scroll to top
- Can conditionally handle scroll based on `to` and `from` route objects

**Route Transitions**

- Wrap `<RouterView>` with `<Transition>` component for animated route changes
- Use dynamic transition names based on route depth or meta for directional animations
- Watch `$route` changes to compute transition direction (e.g., slide-left vs slide-right)
- Keep transitions subtle (50-100ms) for keyboard-first speed optimization

**Composition API Integration**

- Use `useRoute()` composable to access current route reactively (read-only)
- Use `useRouter()` composable to access router instance for navigation
- Watch route changes with `watch(() => route.query.message, ...)` for reactive updates
- Combine with `toValue()` helper for normalizing refs and getters

**KeepAlive for State Preservation**

- Wrap `<RouterView>` with `<KeepAlive>` to cache component instances
- Use `onActivated()` and `onDeactivated()` lifecycle hooks for cache events
- Preserves scroll position and component state without re-mounting
- Triggered on initial mount and when re-inserted from cache

**Route Configuration Best Practices**

- Define routes in separate file (`routes.ts`) for organization
- Use `meta` property for route-specific data (e.g., `meta: { context: 'inbox' }`)
- Leverage dynamic route matching for similar routes (e.g., `/:folder(inbox|sent|drafts)`)
- Use `redirect` for default routes and route aliases for multiple paths

## Out of Scope

- Settings subroutes beyond `/settings/accounts` (defer `/settings/appearance`, `/settings/shortcuts`)
- Separate `/compose` route for new email composition (unless determined necessary during implementation)
- Deep linking from external sources via custom URL schemes
- Route-based code splitting and lazy loading (can be added later)
- Browser back/forward button handling (may not be relevant in Tauri desktop)
- Navigation guards for unsaved changes (can be added as future enhancement)
- Animations beyond 50-100ms subtle transitions
- Search as separate route (may keep inline search in InboxView)
- Route meta tags for SEO or analytics
- Multi-window routing support
