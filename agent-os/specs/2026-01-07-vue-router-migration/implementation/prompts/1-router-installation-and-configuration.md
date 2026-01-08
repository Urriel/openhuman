We're continuing our implementation of Vue Router Migration by implementing task group number 1:

## Implement this task and its sub-tasks:

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

## Understand the context

Read @agent-os/specs/2026-01-07-vue-router-migration/spec.md to understand the context for this spec and where the current task fits into it.

Also read these for further context and reference:

- @agent-os/specs/2026-01-07-vue-router-migration/planning/requirements.md
- @agent-os/specs/2026-01-07-vue-router-migration/planning/visuals

## Perform the implementation

Implement all tasks assigned to you and ONLY those task(s) that have been assigned to you.

## Implementation process:

1. Analyze the provided spec.md, requirements.md, and visuals (if any)
2. Analyze patterns in the codebase according to its built-in workflow
3. Implement the assigned task group according to requirements and standards
4. Update `agent-os/specs/2026-01-07-vue-router-migration/tasks.md` to update the tasks you've implemented to mark them as done by updating their checkbox to checked state: `- [x]`

## Guide your implementation using:

- **The existing patterns** that you've found and analyzed in the codebase.
- **Specific notes provided in requirements.md, spec.md AND/OR tasks.md**
- **Visuals provided (if any)** which would be located in `agent-os/specs/2026-01-07-vue-router-migration/planning/visuals/`
- **User Standards & Preferences** which are defined in AGENTS.md and related files.

## Self-verify and test your work by:

- Running ONLY the tests you've written (if any) and ensuring those tests pass.
- IF your task involves user-facing UI, and IF you have access to browser testing tools, open a browser and use the feature you've implemented as if you are a user to ensure a user can use the feature in the intended way.
  - Take screenshots of the views and UI elements you've tested and store those in `agent-os/specs/2026-01-07-vue-router-migration/implementation/screenshots/`. Do not store screenshots anywhere else in the codebase other than this location.
  - Analyze the screenshot(s) you've taken to check them against your current requirements.
