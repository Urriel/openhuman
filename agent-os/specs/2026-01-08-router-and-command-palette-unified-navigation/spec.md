# Specification: Router and Command Palette Unified Navigation

## Goal

Make navigation consistent and immediate by converting internal links to vue-router navigation, renaming the Trash route to `/trash`, and unifying the command palette to search both commands and emails while suppressing app shortcuts when the palette is open.

## User Stories

- As a user, I want sidebar clicks to navigate instantly so that the app feels responsive and predictable.
- As a user, I want a single command palette that searches commands and emails so that I can do everything from one place.
- As a power user, I want app-wide shortcuts to not interfere while the command palette is open so that I can type safely.

## Specific Requirements

**Trash route rename**

- Replace folder route segment `deleted` with `trash` (no compatibility redirect).
- Update folder route matcher to include `trash` and exclude `deleted`.
- Update any internal navigation that targets Trash to use `/trash`.
- Update existing router tests that enumerate folder routes.

**Router-aware sidebar navigation**

- Replace placeholder `url: '#'` values for internal navigation items with actual route paths.
- Convert sidebar/menu internal links from `<a :href>` to router-aware navigation (`RouterLink`).
- Continue using shadcn-vue sidebar components with `as-child` by integrating with `RouterLink` using the `custom` slot pattern.
- Ensure navigation does not trigger full page reloads.

**Disable app shortcuts while command palette is open**

- While the command palette is open, application shortcuts must not trigger (including sequential shortcuts like `g→i`).
- Allow only `Escape` and `Cmd+K` to close the palette while open.
- Ensure suppressed shortcuts do not leave partial state (e.g., clear any active key sequence buffer).

**Command palette hotkeys**

- Remove the `/` hotkey for opening “search”.
- Keep `Cmd+K` as the only open/toggle shortcut.
- `Cmd+K` must toggle: open palette when closed, close palette when open.
- `Escape` must close the palette when open.

**Unified command + email search results**

- Keep a single command palette input that filters commands and searches emails.
- Show both result types concurrently when query is non-empty.
- Show email results above command results.
- Start email searching only when query length is at least 3 characters.
- Keep command filtering immediate (no debounce required for command filtering).

**Command palette rendering behavior**

- When query is empty: show command groups.
- When query is non-empty: show email results group (and loading/empty states) plus command groups.
- Preserve existing label sub-page behavior (apply label flow).

## Visual Design

No visual assets provided.

## Existing Code to Leverage

**`src/router/routes.ts` and `src/router/index.ts`**

- Current route definitions (folder route matcher and settings route).
- Update folder matcher to replace `deleted` with `trash`.

**`src/components/AppSidebar.vue`**

- Central source of sidebar navigation data.
- Replace placeholder URLs with real router paths.

**`src/components/NavMain.vue` and `src/components/NavProjects.vue`**

- Sidebar link rendering currently uses `<a :href="...">`.
- Replace with router-aware links, keeping the existing sidebar component structure.

**`src/composables/useKeyboardShortcuts.ts`**

- Existing global shortcut manager including sequential key support.
- Add a palette-open suppression gate that only allows `Escape` and `Cmd+K`.

**`src/composables/useCommandPalette.ts` and `src/components/CommandPalette.vue`**

- Existing command palette state, command filtering, and email search rendering.
- Adapt to unified results without a dedicated “search mode”, and implement email-search threshold at 3 characters.

## Out of Scope

- Any backward-compatible redirect route from `/deleted` to `/trash`.
- Keyboard shortcuts customization UI.
- Adding new folders beyond the route rename.
- New visual redesign of the sidebar or command palette beyond necessary behavioral changes.
