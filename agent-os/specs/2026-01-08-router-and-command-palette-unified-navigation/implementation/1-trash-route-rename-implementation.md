# Task Group 1 Implementation: Rename Deleted Folder Route to Trash

**Date:** 2026-01-08  
**Status:** ✅ Complete

## Changes Made

### 1. Updated Route Matcher (`src/router/routes.ts`)

- Replaced `deleted` with `trash` in folder route regex
- Route now matches: `inbox|sent|drafts|favorites|archive|trash|spam|junk`

### 2. Updated Router Tests (`src/router/__tests__/router.test.ts`)

- Updated folder enumeration test to include `trash` instead of `deleted`
- Added specific test for `/trash` route matching
- Added test verifying `/deleted` no longer matches folder route

## Test Results

All router tests passing (9 tests):

- ✅ Creates router instance
- ✅ Redirects root to /inbox
- ✅ Matches folder routes correctly (including trash)
- ✅ Matches /trash route specifically
- ✅ Does not match /deleted route
- ✅ Preserves query parameters
- ✅ Matches settings/accounts route
- ✅ Includes meta context for folder routes
- ✅ Includes meta context for settings routes

## Files Modified

- `src/router/routes.ts` - Updated folder route matcher
- `src/router/__tests__/router.test.ts` - Updated folder list and added trash-specific tests

## Acceptance Criteria Met

- ✅ Navigating to `/trash` renders the folder view
- ✅ Navigating to `/deleted` does not match the folder route
- ✅ Router tests updated and passing
