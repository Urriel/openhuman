We're continuing our implementation of Phase 0 - Bootstrap by implementing task group number 2:

## Implement this task and its sub-tasks:

### UI Framework Setup

#### Task Group 2: Tailwind v4, shadcn-vue, and TanStack Virtual
**Dependencies:** Task Group 1

- [ ] 2.0 Complete UI framework integration
  - [ ] 2.1 Install and configure Tailwind CSS v4
    - Install `tailwindcss` and `@tailwindcss/vite` packages (stable v4)
    - Add `@tailwindcss/vite` plugin to `vite.config.ts`
    - Create main CSS file with `@import "tailwindcss"` (CSS-first config)
    - Import CSS file in main Vue app entry point
  - [ ] 2.2 Verify Tailwind utilities work
    - Add test utilities to a Vue component (e.g., `bg-blue-500`, `p-4`, `text-white`)
    - Run dev server and confirm styles render correctly
    - Test responsive utilities (e.g., `md:text-lg`, `lg:p-8`)
  - [ ] 2.3 Initialize shadcn-vue with default theme
    - Run shadcn-vue CLI initialization
    - Select default theme (no custom theme)
    - Configure Reka UI as primitive library
    - Set up component aliases and import paths
  - [ ] 2.4 Install core shadcn-vue components
    - Install Button component
    - Install Input component
    - Install Card component
    - Install Dialog component
    - Verify each component renders with Tailwind v4 utilities
  - [ ] 2.5 Install and set up TanStack Virtual
    - Install `@tanstack/vue-virtual` package
    - Create `src/components/VirtualList.vue` wrapper component
    - Use Vue 3 Composition API with TypeScript generics
    - Support props: items (generic array), itemHeight, itemRenderer (slot)
  - [ ] 2.6 Create TanStack Virtual demo
    - Create example component with 10,000+ items
    - Verify smooth scrolling at 60 FPS
    - Document component API (props, slots, usage) in comments
    - Add demo to main app for visual verification

**Acceptance Criteria:**
- Tailwind v4 utilities render correctly in Vue components
- shadcn-vue components (Button, Input, Card, Dialog) work with Tailwind v4
- VirtualList.vue component created and handles 10,000+ items smoothly
- Demo component demonstrates virtualized scrolling performance

## Understand the context

Read @agent-os/specs/2026-01-05-bootstrap-phase-0/spec.md to understand the context for this spec and where the current task fits into it.

Also read these further context and reference:
- @agent-os/specs/2026-01-05-bootstrap-phase-0/planning/requirements.md
- @agent-os/specs/2026-01-05-bootstrap-phase-0/planning/visuals

## Perform the implementation

Implement all tasks assigned to you and ONLY those task(s) that have been assigned to you.

## Implementation process:

1. Analyze the provided spec.md, requirements.md, and visuals (if any)
2. Analyze patterns in the codebase according to its built-in workflow
3. Implement the assigned task group according to requirements and standards
4. Update `agent-os/specs/2026-01-05-bootstrap-phase-0/tasks.md` to update the tasks you've implemented to mark that as done by updating their checkbox to checked state: `- [x]`

## Guide your implementation using:
- **The existing patterns** that you've found and analyzed in the codebase.
- **Specific notes provided in requirements.md, spec.md AND/OR tasks.md**
- **Visuals provided (if any)** which would be located in `agent-os/specs/2026-01-05-bootstrap-phase-0/planning/visuals/`
- **User Standards & Preferences** which are defined below.

## Self-verify and test your work by:
- Running ONLY the tests you've written (if any) and ensuring those tests pass.
- IF your task involves user-facing UI, and IF you have access to browser testing tools, open a browser and use the feature you've implemented as if you are a user to ensure a user can use the feature in the intended way.
  - Take screenshots of the views and UI elements you've tested and store those in `agent-os/specs/2026-01-05-bootstrap-phase-0/verification/screenshots/`.  Do not store screenshots anywhere else in the codebase other than this location.
  - Analyze the screenshot(s) you've taken to check them against your current requirements.
