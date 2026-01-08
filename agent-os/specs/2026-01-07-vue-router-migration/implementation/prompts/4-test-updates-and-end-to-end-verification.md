We're continuing our implementation of Vue Router Migration by implementing task group number 4:

## Implement this task and its sub-tasks:

### Task Group 4: Testing & Integration

**Goal**: Ensure all existing tests pass with router changes, add new router-specific tests, and verify end-to-end functionality.

**Sub-tasks**:

1. **Update existing component tests**
   - [ ] Find tests that relied on event emitters (e.g., `wrapper.emitted('navigate')`)
   - [ ] Replace with router mocks or router navigation verification
   - [ ] Update test setup to provide router context (`createRouter`, `createMemoryHistory`)
   - [ ] Ensure tests use `@vue/test-utils` mount with router plugin

2. **Add router integration tests**
   - [ ] Test route transitions (e.g., `/inbox` → `/compose` → `/inbox`)
   - [ ] Test query param updates (e.g., selecting email updates `?message=123`)
   - [ ] Test navigation guards (if any were added)
   - [ ] Test route metadata and matched routes

3. **Add keyboard shortcut tests**
   - [ ] Test global shortcuts trigger correct router navigation
   - [ ] Test command palette shortcuts (Cmd+K → search, etc.)
   - [ ] Test email list shortcuts (j/k navigation, Enter to select)

4. **Test scroll preservation**
   - [ ] Verify `<KeepAlive>` preserves component state
   - [ ] Test virtual list scroll position after navigation
   - [ ] Test that returning to inbox restores previous scroll position

5. **Run full test suite**
   - [ ] Execute `npm run test` and ensure all tests pass
   - [ ] Execute `npm run lint` and fix any linting errors
   - [ ] Execute `vue-tsc --noEmit` and fix any type errors
   - [ ] Execute `cargo test` (if any Rust code was affected)

6. **Manual end-to-end verification**
   - [ ] Run `npm run tauri dev`
   - [ ] Test complete user flows:
     - Navigate to inbox → select email → read → compose reply → send
     - Use command palette (Cmd+K) → search → view results
     - Use keyboard shortcuts (g i, c, j/k) for navigation
     - Test browser back/forward buttons
     - Test direct URL navigation (paste `/inbox?message=123` in address bar if accessible)
   - [ ] Verify transitions are smooth (50-100ms)
   - [ ] Verify no visual regressions

7. **Update documentation**
   - [ ] Update README.md if navigation patterns changed
   - [ ] Update AGENTS.md if new patterns were introduced
   - [ ] Document any breaking changes or migration notes

**Testing Requirements**:

- Minimum 8 tests, maximum 10 tests
- Focus on: end-to-end flows, router integration, keyboard shortcuts, regression prevention

**Dependencies**:

- Requires Task Group 3 (component refactoring) to be completed

---

## Understand the context

Read @agent-os/specs/2026-01-07-vue-router-migration/spec.md to understand the full specification, requirements, and user stories.

Read @agent-os/specs/2026-01-07-vue-router-migration/tasks.md to understand how this task group fits into the overall implementation plan and what dependencies exist.

Read @agent-os/specs/2026-01-07-vue-router-migration/planning/requirements.md to understand the detailed requirements gathered from the planning phase.

---

## Perform the implementation

Implement the task and all sub-tasks listed above according to the specification.

## Implementation process:

1. **Audit existing tests**: Use Glob/Grep to find all test files (`.test.ts`, `.spec.ts`)
2. **Update component tests**: Use Edit tool to update tests that need router mocks
3. **Create new test files**: Use Write tool for new router integration tests
4. **Run tests iteratively**: Fix failing tests one by one
5. **Manual testing**: Follow user flows in the running app
6. **Document changes**: Update relevant documentation files
7. **Final verification**: Run all quality checks (`test`, `lint`, `tsc`)

## Guide your implementation using:

- OpenHuman development standards in `AGENTS.md`
- Testing standards in `agent-os/standards/testing.md` (if exists)
- Vitest patterns for Vue component testing
- Router testing best practices from spec.md
- Focus on user-facing behavior, not implementation details

## Self-verify and test your work by:

1. **Run tests**: `npm run test` - all tests must pass
2. **Type checking**: `vue-tsc --noEmit` - no type errors
3. **Linting**: `npm run lint` - no linting errors
4. **Manual verification**: Run `npm run tauri dev` and verify:
   - Complete user flows work end-to-end
   - No console errors or warnings
   - Keyboard shortcuts work as expected
   - Browser navigation (back/forward) works
   - URL state matches UI state
   - Transitions are smooth (50-100ms)
   - Scroll position preserved correctly
5. **Build verification**: Run `npm run build` - build succeeds without errors
6. **Code review**: Ensure test coverage is adequate for critical paths
7. **Update tasks.md**: Mark all completed sub-tasks with `[x]`
8. **Mark spec as implemented**: Update spec.md status to IMPLEMENTED

---

## Final Checklist

Before considering this task group complete, verify:

- [ ] All 42 sub-tasks across all 4 task groups are marked `[x]` in tasks.md
- [ ] All tests pass (`npm run test`)
- [ ] No type errors (`vue-tsc --noEmit`)
- [ ] No linting errors (`npm run lint`)
- [ ] App builds successfully (`npm run build`)
- [ ] Manual testing confirms all user flows work
- [ ] Documentation updated (README.md, AGENTS.md if needed)
- [ ] No regressions in existing functionality
- [ ] Spec.md marked as IMPLEMENTED

**Congratulations!** The Vue Router Migration is complete. 🎉
