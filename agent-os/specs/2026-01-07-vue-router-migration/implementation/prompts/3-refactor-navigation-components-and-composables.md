We're continuing our implementation of Vue Router Migration by implementing task group number 3:

## Implement this task and its sub-tasks:

### Task Group 3: Refactor Navigation Components and Composables

**Goal**: Update existing components and composables to use vue-router instead of emitting navigation events.

**Sub-tasks**:

1. **Update `src/components/Sidebar.vue`**
   - [ ] Replace `@click` handlers that emit events with `router.push()`
   - [ ] Highlight active route using `route.path` or `route.name`
   - [ ] Use `<RouterLink>` for folder navigation where appropriate
   - [ ] Update keyboard shortcuts to call `router.push()`

2. **Update `src/components/EmailList.vue`**
   - [ ] Remove `@click` event that emits `select-email`
   - [ ] Use `router.push({ query: { message: email.id } })` instead
   - [ ] Highlight selected email using `route.query.message`
   - [ ] Preserve keyboard navigation (j/k keys) to update route

3. **Update `src/components/EmailReader.vue`**
   - [ ] Accept `messageId` from route query instead of props
   - [ ] Remove prop: `selectedMessageId`
   - [ ] Use `const route = useRoute()` and `const messageId = computed(() => route.query.message)`
   - [ ] Update reply/forward actions (stay in-thread, no route change for now)

4. **Update `src/components/CommandPalette.vue`**
   - [ ] Replace navigation actions to use `router.push()`
   - [ ] Update search command to navigate to `/search?q=...`
   - [ ] Update compose command to navigate to `/compose`
   - [ ] Ensure keyboard shortcuts (Cmd+K) still work

5. **Create `src/composables/useEmailNavigation.ts`**
   - [ ] Export functions: `navigateToInbox()`, `navigateToFolder()`, `selectEmail()`, `composeEmail()`, `searchEmails()`
   - [ ] Centralize router logic for consistency
   - [ ] Use TypeScript types for function parameters
   - [ ] Example:
     ```typescript
     export function useEmailNavigation() {
       const router = useRouter();

       function selectEmail(messageId: string) {
         router.push({ query: { ...router.currentRoute.value.query, message: messageId } });
       }

       return { selectEmail, navigateToInbox /* ... */ };
     }
     ```

6. **Update keyboard shortcut handlers**
   - [ ] Identify all keyboard shortcuts in components (e.g., `c` for compose, `g i` for inbox)
   - [ ] Replace event emits with router navigation
   - [ ] Ensure shortcuts work globally (consider composable or event bus)

7. **Remove obsolete event emitters**
   - [ ] Search codebase for `emit('navigate-*')` or similar patterns
   - [ ] Remove unused `defineEmits` declarations
   - [ ] Clean up event listeners in parent components

8. **Update prop interfaces**
   - [ ] Remove navigation-related props (e.g., `selectedMessageId`, `currentFolder`)
   - [ ] Replace with route-aware computed properties
   - [ ] Update TypeScript interfaces accordingly

9. **Preserve scroll position logic**
   - [ ] Wrap `<RouterView>` with `<KeepAlive>` in appropriate views
   - [ ] Test that email list scroll position is preserved when navigating back
   - [ ] Ensure virtual scrolling (VirtualList.vue) works correctly

10. **Test integration**
    - [ ] Verify all navigation paths work end-to-end
    - [ ] Test keyboard shortcuts
    - [ ] Test browser back/forward buttons
    - [ ] Test direct URL navigation (e.g., `/inbox?message=123`)

**Testing Requirements**:

- Minimum 4 tests, maximum 10 tests
- Focus on: router integration, keyboard shortcuts, scroll preservation, component prop updates

**Dependencies**:

- Requires Task Group 2 (view components) to be completed

---

## Understand the context

Read @agent-os/specs/2026-01-07-vue-router-migration/spec.md to understand the full specification, requirements, and user stories.

Read @agent-os/specs/2026-01-07-vue-router-migration/tasks.md to understand how this task group fits into the overall implementation plan and what dependencies exist.

Read @agent-os/specs/2026-01-07-vue-router-migration/planning/requirements.md to understand the detailed requirements gathered from the planning phase.

---

## Perform the implementation

Implement the task and all sub-tasks listed above according to the specification.

## Implementation process:

1. **Read existing components**: Use Read tool to examine all components that need updates
2. **Identify patterns**: Find all event emitters and navigation logic
3. **Create composable first**: Build `useEmailNavigation.ts` as the foundation
4. **Update components systematically**: Work through each component, replacing emits with router calls
5. **Preserve behavior**: Ensure keyboard shortcuts and interactions still work
6. **Write tests**: Create tests for navigation composable and updated components
7. **Verify**: Run `npm run test`, `npm run lint`, and `vue-tsc --noEmit`
8. **Manual testing**: Test all navigation flows in the running app

## Guide your implementation using:

- OpenHuman development standards in `AGENTS.md`
- Vue Router 4 best practices from the spec.md
- TypeScript strict mode requirements
- Keyboard-first design principles (every action has a shortcut)
- Preserve existing UX (smooth transitions, scroll position)

## Self-verify and test your work by:

1. **Run tests**: `npm run test` - all tests must pass
2. **Type checking**: `vue-tsc --noEmit` - no type errors
3. **Linting**: `npm run lint` - no linting errors
4. **Manual verification**: Run `npm run tauri dev` and verify:
   - All navigation works (sidebar, email list, command palette)
   - Keyboard shortcuts work (c, g i, j/k, etc.)
   - Browser back/forward buttons work
   - Scroll position preserved when navigating back
   - URL updates reflect current state
   - Direct URL navigation works
5. **Code review**: Ensure no event emitters remain for navigation
6. **Update tasks.md**: Mark all completed sub-tasks with `[x]`
