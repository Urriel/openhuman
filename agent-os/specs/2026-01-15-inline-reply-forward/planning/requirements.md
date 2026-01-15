# Spec Requirements: Inline Reply/Forward

## Initial Description

implement the reply, reply all and the forward with a small inline composer (Gmail-style prefill) available via keyboard shortcuts or command palette, contextual to the current email; and when opening a folder default to showing the first email if there is no query.

## Requirements Discussion

### First Round Questions

**Q1:** I’m assuming the inline composer opens inside the reader pane (not a modal or full screen). Is that correct, or should it dock elsewhere?
**Answer:** embedded modal like superhuman, it is located at the end of the mail content and displayed above.

**Q2:** For Gmail-style prefill, I’m assuming: Reply = `To` original sender only; Reply All = original sender + all original `To`/`Cc` minus the user’s own address; Forward = empty `To`. Is that correct, or do you want a different rule?
**Answer:** Correct.

**Q3:** I’m assuming the quoted content includes the original message’s plain text (or HTML stripped to text), prefixed by a “On DATE, NAME wrote:” line. Is that correct, or should we include HTML quoting instead?
**Answer:** As does Gmail. what does it do ?

**Q4:** Should the inline composer allow adding/removing `Cc/Bcc` fields (default hidden), or always show them?
**Answer:** Yes, it should.

**Q5:** When triggering reply/reply-all/forward via command palette or shortcut, should it auto-focus the editor and place the cursor at the top of the reply?
**Answer:** yes

**Q6:** I’m assuming we only auto-select the first email on folder load when there is no `message` query param (as you said). Is it ok to skip if the list is empty?
**Answer:** Yes, sure.

**Q7:** Any exclusions or follow-ups you want to call out (e.g., attachments in forward, draft autosave, or threading view)?
**Answer:** We should add the attachment to the forward. We should also autosave the drafts and I want to implement the thread view too.

### Existing Code to Reference

No similar existing features identified for reference.

### Follow-up Questions

**Follow-up 1:** For “embedded modal like Superhuman,” I’m assuming a compact compose card anchored just below the message body, overlaying the lower part of the reader pane (not full-width). Is that the right layout?
**Answer:** Yes.

**Follow-up 2:** Gmail quoting: it uses a “On DATE, NAME <email> wrote:” line and then inserts the original content in a blockquote with indentation. Should we match that structure (blockquote + line) even if we only have plain text available?
**Answer:** Yes.

**Follow-up 3:** Draft autosave: should it trigger after a short debounce (e.g., 2–3 seconds after edits) and save to Drafts on every change?
**Answer:** Yes.

**Follow-up 4:** Thread view: do you want the inline composer to appear within a single-message reader now, or should it live in a new thread view showing multiple messages? (Scope-wise, you said you want thread view too; confirming if that’s in this spec.)
**Answer:** The composer should be within the thread UI instead of the single message view. The thread view should replace the single message reader since we want the threaded view as default and no need for the single view.

## Visual Assets

### Files Provided:

No visual assets provided.

## Requirements Summary

### Functional Requirements

- Add inline reply/reply-all/forward embedded compose card inside the thread view, positioned below the message content and overlaying the lower part of the reader pane.
- Prefill logic mimics Gmail: Reply targets original sender, Reply All includes original sender plus original To/Cc minus the user’s own address, Forward starts with empty To.
- Quote original message content with “On DATE, NAME <email> wrote:” and blockquote indentation; use this structure even for plain text.
- Inline composer supports toggling Cc/Bcc fields.
- Reply/reply-all/forward auto-focuses the editor at the top of the reply.
- On folder load, if there is no `message` query param, auto-select the first email (no selection if folder is empty).
- Forward should include attachments.
- Drafts auto-save with a short debounce, saving to Drafts on changes.
- Thread view becomes the default reader, replacing the single-message reader.

### Reusability Opportunities

- None identified by user.

### Scope Boundaries

**In Scope:**

- Inline reply/reply-all/forward UI in thread view.
- Gmail-style prefill behavior.
- Draft autosave.
- Forwarding attachments.
- Default thread view in place of single-message reader.
- Default selection of first email when no message is selected.

**Out of Scope:**

- None specified beyond future enhancements.

### Technical Considerations

- Inline composer behaves like an embedded modal below content, overlaying the lower reader pane.
- Prefill behavior and quoting should align with Gmail semantics.
- Draft autosave debounce and write-to-drafts behavior required.
