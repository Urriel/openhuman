# Spec Requirements: Superhuman Keymap and Triage Focus

## Initial Description

"Superhuman-like IMAP client" parity roadmap focus: implement the **exact Superhuman macOS keymap** and a **triage focus/state machine** so the app feels like Superhuman.

Key constraints from the user:

- IMAP-first
- macOS only for now
- Snooze/Reminders are local-only overlay (no sync)
- Split Inbox deferred
- AI optional later

## Requirements Discussion

### First Round Questions

**Q1:** I assume this spec covers **both** (a) a canonical Superhuman macOS keymap registry and (b) an explicit **focus/state machine** (list vs reader vs palette vs compose), wired so `Esc` “pops” state predictably. Is that correct, or do you want to keep it **keymap-only** in this spec?
**Answer:** yes it is.

**Q2:** I’m assuming we should treat the keymap as the **single source of truth** for: keyboard handling, command palette shortcuts display, and the shortcuts help UI (so they never drift). Is that correct, or should palette/help remain partially manual?
**Answer:** yes right. only the keyboard shortcut should be disabled when on compose or other inputs and within the command palette.

**Q3:** For multi-key sequences: I’m thinking we should generalize beyond the current special-cased `g→x` to support arbitrary sequences (e.g. `g i`, `g s`, etc.) with a timeout and precedence rules. Should we do that now, or keep sequences limited to `g` for v1?
**Answer:** yes you should support that.

**Q4:** For “triage loop” behavior, I assume: list always has a selected row; `j/k` moves selection; `Enter` opens the selected message; `Esc` returns focus to list without losing selection; actions (`e/#/s`) apply to selection when list-focused and apply to open message when reader-focused. Is that correct, or what differs?
**Answer:** yes it is correct.

**Q5:** I assume “exact Superhuman keymap” means exact default bindings on macOS. How do you want to handle conflicts with browser/OS defaults inside Tauri (e.g. `Cmd+R` refresh vs “sync”)? Prefer **match Superhuman even if it overrides typical app shortcuts**, or keep some OS defaults?
**Answer:** Prefer match Superhuman even if it overrides typical app shortcuts

**Q6:** What’s explicitly out of scope for this spec? I’m assuming **no snooze/remind**, **no split inbox**, and **no send later** here—just keyboard/focus/palette alignment. Correct?
**Answer:** correct.

### Existing Code to Reference

No similar existing features identified for reference.

### Follow-up Questions

None.

## Visual Assets

### Files Provided:

No visual assets provided.

## Requirements Summary

### Functional Requirements

- Implement exact Superhuman macOS keymap support.
- Keymap must be the single source of truth for keyboard handling, command palette shortcut display, and shortcuts help UI.
- Keyboard shortcuts must be disabled when the user is in compose or other inputs, and within the command palette.
- Support multi-key sequences beyond the current special-cased `g→x` approach, including timeouts and precedence rules.
- Implement a triage focus/state machine so list/reader/palette/compose behavior is predictable.
- Triage loop behaviors: list selection, j/k navigation, Enter open, Esc returns focus, actions apply to selection based on focus.
- Keybinding conflicts should prefer Superhuman parity even if it overrides typical shortcuts.

### Reusability Opportunities

- Components that might exist already based on user's input
- Backend patterns to investigate
- Similar features to model after

### Scope Boundaries

**In Scope:**

- Canonical keymap registry (macOS)
- Focus/state machine (list/reader/palette/compose)
- Command palette shortcut display and shortcuts help UI powered from keymap
- Generalized multi-key sequences

**Out of Scope:**

- Snooze
- Remind Me
- Split Inbox
- Send Later
- AI

### Technical Considerations

- `useKeyboardShortcuts` must be a true singleton; multiple calls must share one registry and one `keydown` listener.
- Implement singleton via VueUse `createGlobalState` + listener via VueUse `useEventListener`.
- Shortcut registration should return cleanup functions and components must unregister on unmount to avoid duplicated handlers.
- Shortcut engine context must be synced from router meta: `route.meta.context` (`global`/`inbox`/`thread`/`compose`/`palette`).
- Tests must reset the singleton between runs (`clear()`), and dispatch events against the same target used by the listener (`window`).
