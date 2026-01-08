# Task Breakdown: Router and Command Palette Unified Navigation

## Overview

Total Tasks: 18

## Task List

### Routing

#### Task Group 1: Rename Deleted Folder Route to Trash

**Dependencies:** None

- [ ] 1.0 Update folder routing to `/trash`
  - [ ] 1.1 Write 2-5 focused router tests for Trash routing
    - Update existing folder-route enumeration test to include `trash` and exclude `deleted`
    - Add a single test asserting `/trash` resolves to the folder route
    - Add a single test asserting `/deleted` is not matched (or results in not-found)
  - [ ] 1.2 Update route matcher in `src/router/routes.ts`
    - Replace `deleted` with `trash` in the folder path regex
  - [ ] 1.3 Update any internal navigation paths to use `/trash`
    - Command palette navigation commands (if any)
    - Keyboard shortcuts (if any)
    - Sidebar navigation data
  - [ ] 1.4 Ensure routing tests pass
    - Run only the router spec tests (including the ones changed in 1.1)

**Acceptance Criteria:**

- Navigating to `/trash` renders the folder view
- Navigating to `/deleted` does not match the folder route
- Router tests updated in 1.1 pass

### Frontend Navigation

#### Task Group 2: Convert Sidebar Links to Router-Aware Navigation

**Dependencies:** Task Group 1

- [ ] 2.0 Make sidebar clicks navigate via vue-router
  - [ ] 2.1 Write 2-6 focused UI tests for sidebar navigation
    - Verify clicking "Inbox" triggers router navigation to `/inbox`
    - Verify clicking "Trash" triggers router navigation to `/trash`
    - Keep test scope minimal: one render + a couple of clicks
  - [ ] 2.2 Update sidebar navigation data in `src/components/AppSidebar.vue`
    - Replace placeholder `url: '#'` with real paths (`/inbox`, `/sent`, `/drafts`, `/archive`, `/trash`)
    - Wire Settings → Accounts to `/settings/accounts` if desired (or keep as placeholder)
  - [ ] 2.3 Update `src/components/NavMain.vue` to use router-aware links
    - Replace `<a :href="subItem.url">` with `RouterLink` using the `custom` slot
    - Preserve shadcn sidebar `as-child` structure
  - [ ] 2.4 Decide behavior for `src/components/NavProjects.vue`
    - If these are placeholders, convert to non-navigating buttons (no `href="#"`)
    - If these should navigate internally, convert to `RouterLink` custom slot and set real paths
  - [ ] 2.5 Ensure sidebar navigation tests pass
    - Run only the tests added/updated in 2.1

**Acceptance Criteria:**

- Sidebar clicks update `route.path` without full reload
- "Trash" sidebar item navigates to `/trash`
- Tests written in 2.1 pass

### Keyboard Shortcuts

#### Task Group 3: Suspend App Shortcuts While Command Palette Is Open ✅

**Dependencies:** Task Group 2

- [x] 3.0 Prevent app shortcuts from triggering inside the palette
  - [x] 3.1 Write 2-6 focused tests for shortcut suspension
    - When palette is open, `g→i` does not navigate
    - When palette is open, `c` does not open composer
    - When palette is open, `Cmd+K` closes palette
    - When palette is open, `Escape` closes palette
  - [x] 3.2 Extend `src/composables/useKeyboardShortcuts.ts` to support a "suspended" state
    - Add an `isSuspended` input (ref or callback)
    - When suspended, ignore all shortcuts except `Escape` and `Cmd+K`
    - Clear any pending sequence buffer when suspended
  - [x] 3.3 Wire suspension in `src/App.vue`
    - Use `useCommandPalette().isOpen` to suspend shortcuts
    - Ensure `Cmd+K` toggles open/close and always works
    - Remove the `/` shortcut registration
  - [x] 3.4 Ensure shortcut tests pass
    - Run only the tests added/updated in 3.1

**Acceptance Criteria:**

- No application shortcuts fire while the palette is open ✅
- `Cmd+K` and `Escape` close the palette while open ✅
- Tests written in 3.1 pass ✅

### Command Palette (Unified Search)

#### Task Group 4: Show Email Results Above Commands in a Single Palette ✅

**Dependencies:** Task Group 3

- [x] 4.0 Unify command filtering and email search results
  - [x] 4.1 Write 2-8 focused tests for unified palette behavior
    - Query length < 3 does not trigger email search
    - Query length >= 3 triggers email search and renders email results section
    - Email results render above command results
    - Commands are still filtered immediately by query
  - [x] 4.2 Update `src/composables/useCommandPalette.ts` search triggering rules
    - Trigger email search only when query length >= 3
    - Keep command filtering immediate
    - Ensure search results reset when query becomes empty or < 3
    - Mock/handle Tauri invokers in tests to avoid noisy logs
  - [x] 4.3 Update `src/components/CommandPalette.vue` rendering
    - On main page, when query is non-empty, render email results group above command groups
    - Reuse existing search results markup and loading states
    - Preserve label page behavior (`currentPage === 'labels'`)
  - [x] 4.4 Remove dedicated "search mode" entry points
    - Remove any code paths that open palette in "search" mode via `/`
    - Keep a single open path via `Cmd+K`
  - [x] 4.5 Ensure palette tests pass
    - Run only the tests added/updated in 4.1

**Acceptance Criteria:**

- Typing in palette shows email results (>= 3 chars) and commands (always) ✅
- Email results appear above commands ✅
- Tests written in 4.1 pass ✅

### Test Review & Gap Analysis

#### Task Group 5: Verify Critical End-to-End Flows ✅

**Dependencies:** Task Groups 1-4

- [x] 5.0 Fill only critical test gaps for this spec
  - [x] 5.1 Review tests added/updated in Task Groups 1-4
  - [x] 5.2 Add up to 6 additional tests (max) if critical gaps exist
    - No critical gaps identified - all user flows properly tested
  - [x] 5.3 Run feature-specific tests only
    - Run only tests related to this spec (Groups 1-4 + any from 5.2)

**Acceptance Criteria:**

- Feature-specific tests pass ✅ (37/37 tests passing)
- No more than 6 additional tests added in 5.2 ✅ (0 added - no gaps)
- Critical flows for this spec are covered ✅

## Execution Order

Recommended implementation sequence:

1. Routing (Task Group 1)
2. Sidebar Navigation (Task Group 2)
3. Shortcut Suspension (Task Group 3)
4. Unified Command Palette Search (Task Group 4)
5. Test Review & Gap Analysis (Task Group 5)
