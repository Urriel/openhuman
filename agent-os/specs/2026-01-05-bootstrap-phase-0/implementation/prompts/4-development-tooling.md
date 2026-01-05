We're continuing our implementation of Phase 0 - Bootstrap by implementing task group number 4:

## Implement this task and its sub-tasks:

### Development Tooling

#### Task Group 4: Testing, Linting, and Pre-commit Hooks
**Dependencies:** Task Groups 1-3

- [ ] 4.0 Complete development tooling setup
  - [ ] 4.1 Install and configure Vitest for frontend
    - Install `vitest`, `@vue/test-utils`, `@vitest/ui`, `jsdom`
    - Create `vitest.config.ts` with Vue plugin and jsdom environment
    - Set up test utilities directory (`src/test/utils/`)
    - Configure test file patterns (`**/*.test.ts`, `**/*.spec.ts`)
  - [ ] 4.2 Write 2-4 focused frontend tests
    - Write 2-4 tests for VirtualList component (rendering, props handling)
    - Write 2-4 tests for greet command invocation example
    - Total: 4-8 focused tests maximum
    - Test only critical behaviors, skip exhaustive coverage
  - [ ] 4.3 Configure Rust backend testing
    - Verify `cargo test` command works
    - Create unit test for greet command in `src-tauri/src/commands/greet.rs`
    - Document Rust test module structure
    - Run `cargo test` to verify tests pass
  - [ ] 4.4 Set up ESLint and Prettier for frontend
    - Install ESLint with Vue plugin and TypeScript support
    - Install Prettier for code formatting
    - Create `.eslintrc.json` with recommended Vue + TypeScript rules
    - Create `.prettierrc` with project formatting preferences
    - Add `lint` and `format` scripts to `package.json`
  - [ ] 4.5 Configure Clippy and rustfmt for backend
    - Add `clippy` configuration in `Cargo.toml` or `clippy.toml`
    - Add `rustfmt.toml` with formatting preferences
    - Add scripts to run Clippy and rustfmt
    - Verify linters run successfully
  - [ ] 4.6 Install and configure Husky + lint-staged
    - Install `husky` and `lint-staged` packages
    - Initialize Husky with `npx husky install`
    - Create pre-commit hook script
    - Configure lint-staged in `package.json` to run ESLint on `.ts` and `.vue` files
    - Add Clippy check for Rust files in pre-commit hook
    - Test hook triggers on git commit and completes in under 10 seconds
  - [ ] 4.7 Create environment variable setup
    - Create `.env.example` with placeholder variables (e.g., `VITE_API_URL=`)
    - Document expected environment variables with comments
    - Verify `.env` is in `.gitignore`
    - Configure Vite to load environment variables with `VITE_` prefix
  - [ ] 4.8 Run feature-specific tests
    - Run ONLY the 4-8 frontend tests written in 4.2
    - Run the 1 Rust test for greet command
    - Verify all tests pass
    - Do NOT run entire test suite

**Acceptance Criteria:**
- Vitest configured and 4-8 focused frontend tests pass
- Rust testing works with greet command test passing
- ESLint, Prettier, Clippy, and rustfmt configured and running
- Pre-commit hooks installed and run linters on staged files in under 10 seconds
- Environment variable structure documented in `.env.example`
- All feature-specific tests pass (approximately 5-9 tests total)

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
