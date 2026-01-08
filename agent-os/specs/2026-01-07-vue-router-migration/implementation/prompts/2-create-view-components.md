We're continuing our implementation of Vue Router Migration by implementing task group number 2:

## Implement this task and its sub-tasks:

### Task Group 2: Create View Components Layer

**Goal**: Build the routable view components that will serve as route targets, orchestrating existing components.

**Sub-tasks**:

1. **Create `src/views/InboxView.vue`**
   - [ ] Extract inbox layout from App.vue
   - [ ] Use `<RouterView>` for nested message display
   - [ ] Accept route params/query (folder, labels, filters)
   - [ ] Render EmailList and EmailReader in split-pane
   - [ ] Handle empty state (no message selected)

2. **Create `src/views/ComposeView.vue`**
   - [ ] Wrap ComposeEmail component
   - [ ] Read initial state from route query (reply, forward, draft ID)
   - [ ] Navigate back on send/cancel using `router.push()`

3. **Create `src/views/SearchView.vue`**
   - [ ] Display search results using EmailList
   - [ ] Read search query from route query params
   - [ ] Show search input with pre-filled query
   - [ ] Handle empty results state

4. **Create `src/views/SettingsView.vue`**
   - [ ] Use `<RouterView>` for nested routes
   - [ ] Create SettingsAccountsView.vue as child route
   - [ ] Placeholder for future settings sections

5. **Update `src/App.vue`**
   - [ ] Replace manual state management with `<RouterView>`
   - [ ] Keep CommandPalette integration (update to use `router.push()`)
   - [ ] Remove selectedFolder, selectedMessageId, isComposing refs
   - [ ] Target: reduce from ~183 lines to ~30-50 lines

**Testing Requirements**:

- Minimum 2 tests, maximum 10 tests
- Focus on: view component rendering, route param handling, navigation on user actions

**Dependencies**:

- Requires Task Group 1 (router installation) to be completed

---

## Understand the context

Read @agent-os/specs/2026-01-07-vue-router-migration/spec.md to understand the full specification, requirements, and user stories.

Read @agent-os/specs/2026-01-07-vue-router-migration/tasks.md to understand how this task group fits into the overall implementation plan and what dependencies exist.

Read @agent-os/specs/2026-01-07-vue-router-migration/planning/requirements.md to understand the detailed requirements gathered from the planning phase.

---

## Perform the implementation

Implement the task and all sub-tasks listed above according to the specification.

## Implementation process:

1. **Read the current code**: Use Read tool to examine `src/App.vue` and understand the existing manual state management
2. **Create view components**: Use Write tool to create each view component in `src/views/`
3. **Follow patterns**: Adhere to Vue 3 Composition API + `<script setup>` patterns from AGENTS.md
4. **Update App.vue**: Use Edit tool to replace state management with `<RouterView>`
5. **Write tests**: Create tests for each view component following Vitest patterns
6. **Verify**: Run `npm run test`, `npm run lint`, and `vue-tsc --noEmit`

## Guide your implementation using:

- OpenHuman development standards in `AGENTS.md`
- Vue Router 4 best practices from the spec.md
- TypeScript strict mode requirements
- Tailwind CSS for styling (no custom CSS)
- shadcn-vue components where applicable

## Self-verify and test your work by:

1. **Run tests**: `npm run test` - all tests must pass
2. **Type checking**: `vue-tsc --noEmit` - no type errors
3. **Linting**: `npm run lint` - no linting errors
4. **Manual verification**: Run `npm run tauri dev` and verify:
   - Views render correctly
   - Route params are read properly
   - Navigation works between views
   - App.vue is significantly simplified
5. **Code review**: Ensure code follows keyboard-first, high-density design principles
6. **Update tasks.md**: Mark all completed sub-tasks with `[x]`
