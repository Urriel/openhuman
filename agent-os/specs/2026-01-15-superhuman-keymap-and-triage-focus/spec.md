# Specification: Superhuman Keymap and Triage Focus

## Goal

Make OpenHuman feel like Superhuman by implementing an exact macOS keymap with predictable focus/triage behavior across inbox, reader, compose, and command palette.

## User Stories

- As a power user, I want the default keyboard shortcuts to match Superhuman so that I can use existing muscle memory.
- As a power user, I want focus (list/reader/palette/compose) to be predictable so that navigation and actions feel instant.

## Specific Requirements

**Canonical keymap registry (single source of truth)**

- Define a canonical keymap for macOS that covers `global`, `inbox`, `thread`, `compose`, and `palette` contexts.
- The keymap must drive keyboard handling, command palette shortcut display, and the shortcuts help UI.
- Shortcut handling must be centralized through a single shortcut engine instance (singleton).
- Registering shortcuts should be declarative and centralized (avoid scattering `register(...)` calls across components).
- Prefer Superhuman parity even when it overrides typical app shortcuts (e.g. `Cmd+R`).
- The system must support adding new commands without manually updating multiple places.

**Input-aware shortcut suppression**

- Keyboard shortcuts must be disabled when focus is inside compose inputs, other text inputs, or contenteditable regions.
- Keyboard shortcuts must be disabled while the command palette is open (palette should consume input).
- Suppression must be driven by the canonical keymap:
  - bindings can opt-in to work while typing (e.g. `Escape`, `Cmd+K`) via an explicit flag (e.g. `allowInInput`).
- Global close behavior (e.g. `Escape`) should still work to exit/close overlays in a predictable order.

**Generalized multi-key sequences**

- Support multi-key sequences beyond the current `g→x` special-case.
- Sequences should allow arbitrary chains (e.g. `g i`, `g s`) with a timeout.
- Sequences are represented as space-separated key tokens in the engine and keymap (e.g. `"g i"`).
- Sequence matching must be context-aware and follow deterministic precedence rules.
- Ensure sequences do not interfere with typing into inputs when shortcuts are suppressed.

**Focus and triage state machine**

- Define an explicit focus model with clear targets: inbox list, reader, command palette, compose.
- `Escape` should “pop” state predictably (e.g., close palette → return focus to previous target).
- Inbox list should maintain a persistent selected message when messages exist.
- Switching routes/views should result in consistent selection and focus behavior.

**Triage loop behaviors (Superhuman feel)**

- `j/k` moves selection in the inbox list.
- `Enter` opens the selected message in the reader.
- `Escape` returns focus to the inbox list without losing selection.
- Email actions (e.g. archive/delete/star) apply to selection when list-focused, and apply to the open message when reader-focused.
- Undo should remain available for reversible actions (e.g. via `z`).

**Shortcut help is generated from the keymap**

- The keyboard shortcuts help UI must be generated from the canonical keymap (no hardcoded shortcut list).
- It should remain searchable by key, description, and category.
- It should display context information when relevant.

**Command palette consistency**

- Command palette items must display shortcut hints sourced from the canonical keymap.
- Command palette actions and keyboard-triggered actions should resolve to the same underlying action identifiers.

## Visual Design

No visual assets provided.

## Existing Code to Leverage

**`src/composables/useKeyboardShortcuts.ts`**

- Provides the current shortcut manager, including input-focus detection and basic context gating.
- Uses VueUse `useEventListener`, but currently creates isolated instances per call; this must become a singleton so app-wide shortcuts share one registry + one listener.
- Implements generalized multi-key sequences (space-separated tokens like `g i`), but context must be driven from route/view state.

**`src/composables/useCommandPalette.ts`**

- Defines a command catalog with categories, keywords, and per-command shortcut strings.
- Needs to stop being its own shortcut source and instead consume shortcuts from the new canonical keymap.
- Contains the notion of context (`inbox`, `thread`, etc.) that should align with the new keymap contexts.

**`src/components/KeyboardShortcutsHelp.vue`**

- Currently hardcodes shortcut definitions for display.
- Should be refactored to render from the canonical keymap so it cannot drift.

**`src/App.vue`**

- Registers global shortcuts (`Cmd+K`, `/`, `g ...`, `Cmd+R`, `?`, etc.).
- Must synchronize the shortcut engine context from the router (`route.meta.context`) so that inbox/thread/compose-scoped shortcuts resolve correctly.
- Is a key integration point to centralize shortcut registration and coordinate focus/palette/undo behavior.

**`src/composables/useUndoStack.ts`**

- Provides single-step undo behavior via a stored callback.
- Must remain compatible with action execution in the new shortcut and focus model.

## Out of Scope

- Snooze
- Remind Me
- Split Inbox
- Send Later
- AI features
- Redesigning UI visuals beyond what is necessary for focus/keyboard behavior
