# Tasks: Superhuman Keymap and Triage Focus

## Task Group 1: Global Shortcut Engine (VueUse Singleton)

- [x] Refactor `useKeyboardShortcuts` to a VueUse singleton
  - [x] Use `createGlobalState` so all callers share state
  - [x] Attach one `keydown` listener via `useEventListener(window, ...)`
  - [x] Add `setContext` / `context` API for `KeymapContext`
  - [x] Make `register(...)` return cleanup `() => void`
  - [x] Keep `clear()` for tests (reset registry + sequence)

## Task Group 2: Context Wiring (Router → Shortcut Engine)

- [x] Sync shortcut engine context from `route.meta.context`
  - [x] Watch route meta and set `global` fallback
  - [x] Ensure this runs immediately on app start

## Task Group 3: Component Registration Lifecycle

- [x] Ensure component-scoped shortcuts unregister on unmount
  - [x] Update `EmailList.vue` to store cleanup fns
  - [x] Unregister shortcuts in `onUnmounted` (or `tryOnUnmounted`)

## Task Group 4: Tests + Verification

- [x] Update `useKeyboardShortcuts` unit tests for singleton API
  - [x] Use `setContext('global')`
  - [x] Dispatch keydown events on `window`
  - [x] Reset singleton state between tests via `clear()`

- [x] Run lint + unit tests
  - [x] `npm run lint`
  - [x] `npm run test`
