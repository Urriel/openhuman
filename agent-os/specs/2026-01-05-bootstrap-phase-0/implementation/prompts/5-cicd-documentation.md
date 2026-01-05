We're continuing our implementation of Phase 0 - Bootstrap by implementing task group number 5:

## Implement this task and its sub-tasks:

### CI/CD & Documentation

#### Task Group 5: GitHub Actions and Copilot Configuration
**Dependencies:** Task Groups 1-4

- [ ] 5.0 Complete CI/CD and project documentation
  - [ ] 5.1 Create GitHub Actions workflow for linting
    - Create `.github/workflows/lint.yml`
    - Run ESLint on frontend code (`.ts`, `.vue` files)
    - Run TypeScript type checking (`tsc --noEmit`)
    - Run Clippy on Rust backend code
    - Trigger on push to main and pull requests
  - [ ] 5.2 Create GitHub Actions workflow for testing
    - Create `.github/workflows/test.yml`
    - Run Vitest for frontend tests
    - Run `cargo test` for backend tests
    - Trigger on push to main and pull requests
    - Target completion time under 5 minutes
  - [ ] 5.3 Create GitHub Actions workflow for build verification
    - Create `.github/workflows/build.yml`
    - Run `npm run build` for frontend
    - Run `cargo build --release` for backend
    - Verify builds complete successfully
    - Note: Defer platform-specific packaging to future phase
  - [ ] 5.4 Set up GitHub Copilot coding agent configuration
    - Create `.github/copilot-instructions.md`
    - Define Vue 3 Composition API conventions
    - Specify TypeScript strict mode requirements
    - Include Rust coding patterns and conventions
    - Reference tech stack (Tailwind v4, shadcn-vue, TanStack Virtual, Tauri 2)
    - Add keyboard-first design principle for future guidance
  - [ ] 5.5 Create project documentation
    - Update README.md with project overview and tech stack
    - Document development setup (install dependencies, run dev server)
    - Document testing commands (`npm run test`, `cargo test`)
    - Document linting and formatting commands
    - Add link to tech stack document at `agent-os/product/tech-stack.md`
  - [ ] 5.6 Verify CI/CD workflows
    - Push changes to trigger workflows
    - Verify lint workflow passes
    - Verify test workflow passes
    - Verify build workflow passes
    - Confirm all workflows complete in under 5 minutes

**Acceptance Criteria:**
- GitHub Actions workflows created for linting, testing, and building
- All CI/CD workflows pass successfully
- Workflows complete in under 5 minutes
- GitHub Copilot instructions configured with project standards
- README.md updated with setup and development instructions
- Project fully documented and ready for feature development

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
