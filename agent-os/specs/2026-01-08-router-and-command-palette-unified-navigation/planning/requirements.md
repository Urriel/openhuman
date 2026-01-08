# Spec Requirements: Router and Command Palette Unified Navigation

## Initial Description

"just the previous plan"

Context from conversation:

- Fix vue-router navigation not working in sidebar links.
- Update folder route from `/deleted` to `/trash` (no compatibility redirect).
- Disable ALL app shortcuts while the command palette is open (allow only `Escape` and `Cmd+K`).
- Remove `/` shortcut; use only `Cmd+K`.
- Unify command filtering and email search into the same command palette input (Linear-style).

## Requirements Discussion

### First Round Questions

**Q1:** I assume the “Trash” folder is purely a URL rename (`/deleted` → `/trash`) and the underlying folder key should also become `trash` everywhere (routes, UI state, backend folder enum if any). Is that correct, or do you want UI to say Trash but internal folder key remain `deleted`?
**Answer:** 1. yes

**Q2:** For the unified command palette: I assume the main results should include **both** “Commands” and “Emails”, with email search starting only when query length ≥ 2. Should commands still show when query is non-empty, or should email results take priority (e.g., show both, but emails on top)?
**Answer:** 2. show both, email on top. start email filtering at 3 chars

**Q3:** For “no shortcuts while palette open”: I assume we still allow typing, arrows, enter, and backspace inside the palette (handled by the palette component), but we block _application_ shortcuts (e.g., `g→i`, `c`, `z`, `?`, etc.). Is that correct?
**Answer:** 3. yes that is correct.

**Q4:** For `Cmd+K` toggle behavior: when palette is open, should `Cmd+K` close it even if an input is focused (yes by your earlier answer), and should it also clear the query on close (current behavior), correct?
**Answer:** 4. yes cmd+k and escape should close the palette when the palette is open.

**Q5:** For sidebar navigation links: I assume you want all internal navigation to use router-aware links (`RouterLink` custom slot) instead of `<a href>`, and that “Projects” section links should either be real routes or become non-navigating buttons. Which do you prefer?
**Answer:** 5. yes you are right.

**Q6:** Exclusions: anything explicitly out of scope for this spec (e.g., email result click navigating to a full thread view, supporting old `/deleted` links, keyboard shortcut rebinding UI)?
**Answer:** 6. no

### Existing Code to Reference

No similar existing features identified for reference.

### Follow-up Questions

No follow-up questions asked.

## Visual Assets

No visual assets provided.

## Requirements Summary

### Functional Requirements

- Rename folder route from `/deleted` to `/trash` (no compatibility redirect), and standardize on `trash` as the underlying folder key.
- Update all internal navigation links to use router-aware navigation and correct URLs.
- Command palette should show both commands and email results, with email results shown above commands.
- Email filtering/search should only run when the query reaches 3 characters.
- While command palette is open, disable all application shortcuts; only `Cmd+K` and `Escape` should close the palette.
- Remove `/` shortcut; `Cmd+K` is the only command palette shortcut.

### Reusability Opportunities

- No similar existing features were identified.

### Scope Boundaries

**In Scope:**

- Router URL change `/deleted` → `/trash` across the app.
- Sidebar and other internal links updated to proper router navigation.
- Keyboard shortcut suppression while command palette is open, except `Cmd+K` and `Escape`.
- Unified command palette results ordering and email-search threshold.

**Out of Scope:**

- No explicit exclusions beyond the answers given.

### Technical Considerations

- Router integration should use vue-router best practices (router-aware links and programmatic navigation).
- Command palette search is unified within the existing command palette component.
- Avoid triggering email search until query length is at least 3 characters.
