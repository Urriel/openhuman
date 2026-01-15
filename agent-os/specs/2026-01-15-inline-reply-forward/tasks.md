# Task Breakdown: Inline Reply/Forward + Thread Reader

## Overview

Total Tasks: 20

## Task List

### Backend & Data Layer

#### Task Group 1: Threaded Reader Data Support

**Dependencies:** None

- [ ] 1.0 Complete backend thread data support
  - [ ] 1.1 Write 2-8 focused tests for thread retrieval
    - Cover thread lookup, ordering, and missing thread handling
    - Limit to critical behaviors only
  - [ ] 1.2 Add API/command to fetch thread messages by selected message
    - Reuse threading structures in `src-tauri/src/email/threading.rs`
    - Return ordered list of messages with metadata and attachments
  - [ ] 1.3 Ensure backend thread tests pass
    - Run ONLY tests from 1.1

**Acceptance Criteria:**

- Thread command returns ordered messages for selected email
- Thread retrieval tests pass

#### Task Group 2: Draft Autosave Support

**Dependencies:** Task Group 1

- [ ] 2.0 Complete draft autosave backend
  - [ ] 2.1 Write 2-8 focused tests for draft save behavior
    - Validate save_draft updates and required fields
  - [ ] 2.2 Add or extend command to upsert drafts for autosave
    - Store draft metadata for reply/forward context
  - [ ] 2.3 Ensure draft autosave tests pass
    - Run ONLY tests from 2.1

**Acceptance Criteria:**

- Draft autosave endpoint accepts incremental updates
- Draft tests pass

### Frontend UI & Interaction

#### Task Group 3: Thread Reader UI

**Dependencies:** Task Group 1

- [ ] 3.0 Complete thread reader UI
  - [ ] 3.1 Write 2-8 focused tests for thread reader rendering
    - Ensure multiple messages render with ordering
    - Verify empty state when no selection
  - [ ] 3.2 Replace `EmailReader` with threaded reader component
    - Render conversation messages for selected email
    - Preserve existing reader layout styles
  - [ ] 3.3 Ensure thread reader tests pass
    - Run ONLY tests from 3.1

**Acceptance Criteria:**

- Thread view renders messages in order
- Thread reader tests pass

#### Task Group 4: Inline Reply/Forward Composer

**Dependencies:** Task Group 3

- [ ] 4.0 Complete inline composer UI
  - [ ] 4.1 Write 2-8 focused tests for inline composer
    - Reply actions open composer and focus editor
    - Cc/Bcc toggle works
  - [ ] 4.2 Build inline compose card anchored below message content
    - Overlay lower reader pane and match compact style
  - [ ] 4.3 Reuse editor + attachments from `EmailComposer`
    - Ensure forward includes attachments by default
  - [ ] 4.4 Ensure inline composer tests pass
    - Run ONLY tests from 4.1

**Acceptance Criteria:**

- Inline composer opens in thread view and focuses editor
- Cc/Bcc toggles
- Inline composer tests pass

#### Task Group 5: Reply/Forward Prefill Logic

**Dependencies:** Task Group 4

- [ ] 5.0 Complete Gmail-style prefill logic
  - [ ] 5.1 Write 2-8 focused tests for prefill rules
    - Reply, reply-all, forward recipients
    - Subject prefix and quoted content formatting
  - [ ] 5.2 Implement prefill helpers
    - Quote with “On DATE, NAME <email> wrote:” + blockquote
    - Strip HTML when needed
  - [ ] 5.3 Ensure prefill tests pass
    - Run ONLY tests from 5.1

**Acceptance Criteria:**

- Prefill matches Gmail rules
- Quoted content rendered correctly
- Prefill tests pass

#### Task Group 6: Command Palette + Shortcuts Integration

**Dependencies:** Task Group 4

- [ ] 6.0 Complete command palette and shortcut wiring
  - [ ] 6.1 Write 2-8 focused tests for reply actions
    - Ensure palette and shortcuts open inline composer
  - [ ] 6.2 Update `useCommandPalette` reply actions to open composer
  - [ ] 6.3 Add thread-view shortcut context handling
  - [ ] 6.4 Ensure command/shortcut tests pass
    - Run ONLY tests from 6.1

**Acceptance Criteria:**

- Reply actions work via palette and shortcuts
- Tests pass

#### Task Group 7: Folder Default Selection

**Dependencies:** Task Group 3

- [ ] 7.0 Default-select first email on folder load
  - [ ] 7.1 Write 2-8 focused tests for default selection
    - Only when no message query param
  - [ ] 7.2 Update selection behavior in EmailList/App wiring
    - Select first email when list loads and query absent
  - [ ] 7.3 Ensure selection tests pass
    - Run ONLY tests from 7.1

**Acceptance Criteria:**

- First email auto-selected when appropriate
- Tests pass

### Testing & Validation

#### Task Group 8: Integration Test Gap Check

**Dependencies:** Task Groups 1-7

- [ ] 8.0 Review tests and fill critical gaps
  - [ ] 8.1 Review tests from groups 1-7
  - [ ] 8.2 Add up to 10 additional integration tests if needed
    - Focus on thread view + inline reply workflow
  - [ ] 8.3 Run only feature-specific tests
    - Include newly added tests only

**Acceptance Criteria:**

- Feature-specific tests pass
- No more than 10 additional tests added

## Execution Order

Recommended implementation sequence:

1. Backend thread data support (Task Group 1)
2. Draft autosave backend (Task Group 2)
3. Thread reader UI (Task Group 3)
4. Inline composer UI (Task Group 4)
5. Prefill logic (Task Group 5)
6. Command palette + shortcuts (Task Group 6)
7. Folder default selection (Task Group 7)
8. Integration test gap check (Task Group 8)
