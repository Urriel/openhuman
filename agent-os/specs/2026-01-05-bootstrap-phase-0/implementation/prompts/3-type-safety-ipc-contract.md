We're continuing our implementation of Phase 0 - Bootstrap by implementing task group number 3:

## Implement this task and its sub-tasks:

### Type Safety & IPC Contract

#### Task Group 3: Shared Types and Tauri Commands
**Dependencies:** Task Group 1

- [ ] 3.0 Complete type-safe IPC setup
  - [ ] 3.1 Create shared types folder structure
    - Create `src/types/` directory for TypeScript types
    - Create `src-tauri/src/commands/` directory for Rust command handlers
    - Set up module exports for shared types
  - [ ] 3.2 Implement example "greet" command in Rust
    - Create `src-tauri/src/commands/greet.rs` with greet function
    - Accept name parameter and return greeting string
    - Register command in Tauri builder in `main.rs`
    - Add proper error handling
  - [ ] 3.3 Define TypeScript types for greet command
    - Create `src/types/commands.ts` with GreetRequest and GreetResponse types
    - Ensure types match Rust command signature
    - Export types for use in frontend
  - [ ] 3.4 Implement frontend invocation example
    - Create example component that calls greet command
    - Use Tauri's `invoke` API with TypeScript types
    - Display result in UI to verify IPC works
    - Add error handling for failed invocations
  - [ ] 3.5 Document IPC pattern
    - Add comments explaining command structure
    - Document how to add new commands
    - Provide example of type-safe command pattern
    - Reference this pattern in README or docs

**Acceptance Criteria:**
- "greet" command implemented in Rust and callable from frontend
- TypeScript types ensure type safety between frontend and backend
- Example component successfully invokes command and displays result
- IPC pattern documented for future command implementations

## Understand the context

Read @agent-os/specs/2026-01-05-bootstrap-phase-0/spec.md to understand the context for this spec and where the current task fits into it.

Also read these further context and reference:
- @agent-os/specs/2026-01-05-bootstrap-phase-0/planning/requirements.md
- @agent-os/specs/2026-01-05-bootstrap-phase-0/planning/visuals

## Perform the implementation

Implement all tasks assigned to you and ONLY those task(s) that have been assigned to you.

## Implementation process:

1. Analyze the provided spec.md, requirements.md, and visuals (if any)
2. Analyze patterns in the codebase according to its built-in workflow
3. Implement the assigned task group according to requirements and standards
4. Update `agent-os/specs/2026-01-05-bootstrap-phase-0/tasks.md` to update the tasks you've implemented to mark that as done by updating their checkbox to checked state: `- [x]`

## Guide your implementation using:
- **The existing patterns** that you've found and analyzed in the codebase.
- **Specific notes provided in requirements.md, spec.md AND/OR tasks.md**
- **Visuals provided (if any)** which would be located in `agent-os/specs/2026-01-05-bootstrap-phase-0/planning/visuals/`
- **User Standards & Preferences** which are defined below.

## Self-verify and test your work by:
- Running ONLY the tests you've written (if any) and ensuring those tests pass.
- IF your task involves user-facing UI, and IF you have access to browser testing tools, open a browser and use the feature you've implemented as if you are a user to ensure a user can use the feature in the intended way.
  - Take screenshots of the views and UI elements you've tested and store those in `agent-os/specs/2026-01-05-bootstrap-phase-0/verification/screenshots/`.  Do not store screenshots anywhere else in the codebase other than this location.
  - Analyze the screenshot(s) you've taken to check them against your current requirements.
