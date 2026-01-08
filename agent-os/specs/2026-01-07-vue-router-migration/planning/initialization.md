# Vue Router Migration - Initial Idea

## Raw Description

Add vue-router to the app for better code organization with HTML5 mode and full migration from current state-based navigation.

## User Requirements

1. **Primary motivation:** Better code organization
2. **History mode:** HTML5 mode (not hash mode)
3. **Migration strategy:** Full migration (replace all state-based navigation with routes)
4. **Route structure:** Standard structure including:
   - `/` → Inbox (default)
   - `/inbox`, `/sent`, etc. → Folder views
   - `/compose` → Email composer
   - `/compose/:draftId` → Resume draft
   - `/message/:id` → Read specific email
   - `/settings` → Settings view
   - `/settings/accounts` → Account management

## Current State

The app currently uses:

- Manual state management via refs (`isComposing`, `showAccountManagement`, `selectedFolder`, `selectedMessageId`)
- Conditional rendering with `v-if` in `App.vue`
- Event-based navigation (`@compose`, `@select-folder`, `@select-message`)
- Placeholder URLs (`#`) in sidebar navigation items

## Date Initialized

2026-01-07
