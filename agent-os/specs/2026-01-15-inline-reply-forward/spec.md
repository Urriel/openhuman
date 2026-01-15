# Specification: Inline Reply/Forward + Thread Reader

## Goal

Enable Gmail-style inline reply/reply-all/forward within a threaded reader, while auto-selecting the first email on folder load when none is selected.

## User Stories

- As a power user, I want to reply or forward directly from the thread view so that I can respond quickly without leaving the reader.
- As a keyboard-first user, I want reply actions accessible from shortcuts and command palette so that I can stay in flow.
- As a triaging user, I want the first email selected automatically when opening a folder so that I immediately see content.

## Specific Requirements

**Threaded reader as default**

- Replace the single-message reader with a threaded reader view as the default display.
- Thread view shows multiple messages in a conversation for the selected email.
- Inline composer lives within the thread view, not the legacy single-message reader.

**Inline reply composer placement**

- Render an embedded compose card anchored below the message content.
- Composer overlays the lower portion of the reader pane (Superhuman-like card, not full width).
- Opening a reply action reveals the inline composer within the thread view.

**Reply/reply-all/forward prefill**

- Reply: prefill `To` with the original sender only.
- Reply all: prefill `To` with original sender plus original `To`/`Cc` minus the user’s own address.
- Forward: leave `To` empty and reuse attachments from the original message.
- Subject prefixes mirror Gmail: `Re:` for replies and `Fwd:` for forwards.

**Quoted content formatting**

- Insert a line like “On DATE, NAME <email> wrote:” above quoted content.
- Quote original content in an indented blockquote style even for plain text.
- Use HTML-to-text stripping when the original HTML body is unavailable.

**Composer interactions**

- Auto-focus the editor at the top of the reply on open.
- Allow toggling `Cc`/`Bcc` fields (hidden by default).
- Support adding/removing attachments when forwarding.

**Draft autosave**

- Autosave drafts on a short debounce after edits (2–3 seconds).
- Save drafts on every change while the inline composer is open.

**Command palette + shortcut integration**

- Reply, reply-all, and forward are available in command palette and shortcuts.
- Actions are contextual to the currently selected email in the thread view.

**Folder default selection**

- When a folder loads with no `message` query param, auto-select the first email.
- If the folder is empty, keep selection unset.

## Visual Design

No visual assets provided.

## Existing Code to Leverage

**Email reader + list layout**

- `src/App.vue` shows the current reader/list split layout and message selection wiring.
- Use this structure to swap in the thread reader and keep list selection behavior.

**Email reader view**

- `src/components/EmailReader.vue` is the current single-message reader.
- Replace or extend this pattern to render a thread view and inline composer.

**Email composer**

- `src/components/EmailComposer.vue` contains the rich text editor and attachment upload.
- Reuse its editor configuration and attachment upload component for inline composer.

**Command palette actions**

- `src/composables/useCommandPalette.ts` already defines reply/reply-all/forward commands.
- Extend those actions to open the inline composer with context.

**Keyboard shortcuts system**

- `src/composables/useKeyboardShortcuts.ts` manages context-aware shortcuts.
- Use the same context wiring for thread-view reply shortcuts.

**Backend threading foundation**

- `src-tauri/src/email/threading.rs` contains the JWZ threading algorithm structures.
- Leverage this to build the thread view data source.

## Out of Scope

- Full attachment sending support beyond forwarding existing attachments.
- New draft management UI or draft list views beyond autosave integration.
- AI-assisted replies or summarization.
- Split inbox or other advanced mailbox features.
