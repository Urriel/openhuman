# Spec Requirements: Phase 0 - Bootstrap

## Initial Description
Bootstrap the foundation for the OpenHuman email client application:

- Repo scaffolding (Tauri 2 + Vue 3 + Vite)
- Tailwind v4 + shadcn-vue setup
- TanStack Vue Virtual setup
- Shared types + IPC contract

This is the foundational setup phase that establishes the project structure, build tooling, UI framework, and communication layer between the frontend (Vue) and backend (Rust/Tauri).

## Requirements Discussion

### First Round Questions

**Q1:** I assume we should use the **create-tauri-app** CLI to scaffold the initial Tauri 2 + Vue 3 project. Is that correct, or would you prefer a manual setup from scratch?
**Answer:** Yes, use create-tauri-app CLI.

**Q2:** For Tailwind v4, I'm thinking we should set up the new CSS-first configuration (using `@import "tailwindcss"` in CSS rather than tailwind.config.js). Should we follow the v4 alpha/beta approach, or would you prefer the stable v3 with plans to upgrade later?
**Answer:** Tailwind v4 is out of beta. Check Context7 for latest stable configuration.

**Q3:** For shadcn-vue integration, I assume we should initialize it with the **default theme** and install core primitives (Button, Input, Card, Dialog, etc.) that will be needed for the email UI. Is that correct, or do you have a specific set of components or custom theme in mind?
**Answer:** No custom theme, use default.

**Q4:** For TanStack Virtual setup, I'm thinking we should create a **reusable virtualized list component** as part of the bootstrap that can be used later for the email list. Should we build this as a generic wrapper, or just install the library and create the component later when needed?
**Answer:** Good thinking - create a reusable virtualized list component wrapper.

**Q5:** For the shared types and IPC contract, I assume we should set up:
- A `/src-tauri/src/commands/` folder for Tauri commands (Rust)
- A `/src/types/` folder for shared TypeScript types
- A basic IPC example (like a "greet" command) to verify the setup works

Is that the structure you want, or do you have a different organization in mind?
**Answer:** OK, that structure works.

**Q6:** Should we include any **development tooling** in this bootstrap phase, such as:
- Pre-commit hooks (husky, lint-staged)
- VS Code workspace settings
- Environment variable setup (.env files)
- CI/CD configuration (GitHub Actions)

Or should these be added in a later phase?
**Answer:** Yes, include pre-commit hooks, .env files, CI/CD, and Copilot coding agent setup. Use Context7 for understanding external libraries and needed context.

**Q7:** For **testing setup**, should we configure Vitest and Vue Test Utils now with a sample test, or defer testing configuration until we have actual features to test?
**Answer:** Yes, set it up now.

**Q8:** Is there anything you specifically want to **exclude** from this bootstrap phase that might typically be included in project setup (e.g., analytics, error tracking, feature flags, logging infrastructure)?
**Answer:** No, nothing to exclude.

### Existing Code to Reference

**Similar Features Identified:**
No similar existing features identified for reference. This is the first Tauri project for the user.

### Follow-up Questions
None required - all questions answered clearly.

## Visual Assets

### Files Provided:
No visual assets provided.

### Visual Insights:
Not applicable - no visual files found.

## Requirements Summary

### Functional Requirements

#### Core Project Scaffolding
- Use `create-tauri-app` CLI to initialize Tauri 2 + Vue 3 + Vite project
- Set up TypeScript for both frontend and backend (Rust already uses strong typing)
- Configure project structure following Tauri 2 conventions

#### Tailwind CSS v4 Setup
- Install `tailwindcss` and `@tailwindcss/vite` packages (v4 stable)
- Use CSS-first configuration with `@import "tailwindcss"` in main CSS file
- Add `@tailwindcss/vite` plugin to Vite configuration
- Verify Tailwind utilities work in Vue components

#### shadcn-vue Integration
- Initialize shadcn-vue with default theme
- Install core UI primitives needed for email client:
  - Button
  - Input
  - Card
  - Dialog
  - Additional components as needed for basic UI
- Configure Reka UI (underlying primitive library)
- Set up component aliases and import paths

#### TanStack Virtual Setup
- Install `@tanstack/vue-virtual` package
- Create a **reusable virtualized list component wrapper** (`VirtualList.vue`)
- Component should be generic enough to handle different item types
- Include basic example/demo to verify scrolling performance
- Document component API for future use

#### Shared Types & IPC Contract
- Create `/src-tauri/src/commands/` folder for Tauri command handlers
- Create `/src/types/` folder for shared TypeScript types
- Implement basic "greet" command as IPC example:
  - Rust command handler in backend
  - TypeScript type definitions
  - Frontend invocation example
- Ensure type safety between frontend and backend

#### Development Tooling
- **Pre-commit hooks:**
  - Install and configure husky
  - Set up lint-staged for running linters on staged files
  - Configure to run ESLint (frontend) and Clippy (Rust backend)
  
- **Environment variables:**
  - Create `.env.example` file with placeholder variables
  - Add `.env` to `.gitignore`
  - Document required environment variables
  
- **CI/CD (GitHub Actions):**
  - Create workflow for linting and type checking
  - Create workflow for running tests (frontend and backend)
  - Create workflow for building the application
  - Consider platform-specific builds (macOS, Windows, Linux)
  
- **GitHub Copilot Coding Agent:**
  - Set up custom instructions for the project
  - Create `.github/copilot-instructions.md` or similar configuration
  - Define coding standards and conventions for Copilot to follow
  - Research Context7 for best practices

#### Testing Setup
- **Frontend (Vitest + Vue Test Utils):**
  - Install `vitest`, `@vue/test-utils`, and related dependencies
  - Configure `vitest.config.ts`
  - Create sample component test to verify setup
  - Set up test utilities and helpers
  
- **Backend (Rust native testing):**
  - Verify `cargo test` works
  - Create sample unit test for a command
  - Document testing patterns

#### Code Quality Tools
- **Frontend:**
  - ESLint with Vue plugin
  - Prettier for formatting
  - TypeScript strict mode
  
- **Backend:**
  - Clippy for linting
  - rustfmt for formatting

### Reusability Opportunities
- **Virtualized list component** will be reused throughout the app for:
  - Email inbox list
  - Search results
  - Folder views
  - Any large lists of items
  
- **IPC patterns** established in bootstrap will be template for:
  - Email sync commands
  - Account management commands
  - Settings updates
  - All backend operations

- **Testing patterns** will serve as foundation for:
  - Feature component tests
  - Command handler tests
  - Integration tests

### Scope Boundaries

**In Scope:**
- Complete project scaffolding with Tauri 2 + Vue 3 + Vite
- Tailwind v4 CSS framework integration
- shadcn-vue component library setup with default theme
- TanStack Virtual library + reusable component wrapper
- Shared types and IPC contract structure with working example
- Pre-commit hooks (husky + lint-staged)
- Environment variable configuration
- CI/CD workflows (GitHub Actions)
- GitHub Copilot coding agent configuration
- Testing infrastructure (Vitest for frontend, cargo test for backend)
- Code quality tools (ESLint, Prettier, Clippy, rustfmt)
- Basic documentation for setup and development

**Out of Scope:**
- Actual email features (account management, sync, UI)
- Database/SQLite setup (covered in later phases)
- Email protocol implementations (POP3/SMTP)
- Authentication or secrets management
- Application icons, branding, or assets
- Production deployment configurations
- Performance optimization beyond basic setup
- Analytics, error tracking, or monitoring
- Feature flags or A/B testing infrastructure

**Future Enhancements:**
- Custom Tailwind theme once design system is defined
- Additional shadcn-vue components as features are built
- Advanced CI/CD workflows (automated releases, code signing)
- E2E testing setup (Playwright)
- More sophisticated linting rules as patterns emerge

### Technical Considerations

#### Integration Points
- Vite configuration must support both Vue and Tauri builds
- TypeScript must be configured for both frontend (`src/`) and Rust-generated types
- shadcn-vue components must work with Tailwind v4 utilities
- TanStack Virtual must integrate smoothly with Vue 3 Composition API
- Pre-commit hooks should run quickly to avoid developer frustration
- CI/CD must handle Rust compilation and frontend builds

#### Technology Choices (from Context7 Research)
- **Tailwind v4 (stable):**
  - Install: `npm install tailwindcss @tailwindcss/vite`
  - Use CSS-first config: `@import "tailwindcss"` in CSS
  - Add `@tailwindcss/vite` plugin to `vite.config.ts`
  
- **GitHub Copilot:**
  - Custom instructions for project-specific guidance
  - Configuration for consistent code generation
  - Integration with coding standards

#### Performance Expectations
- Development server should start in <5 seconds
- Hot module replacement (HMR) should be near-instant
- Linting and type checking in pre-commit should complete in <10 seconds
- CI/CD workflows should complete in <5 minutes for basic checks

#### Architecture Patterns
- Monorepo structure: frontend (`src/`) and backend (`src-tauri/`)
- Type safety: shared types between frontend and backend
- Component-driven: shadcn-vue components as building blocks
- Test-driven: infrastructure ready for TDD approach
- Keyboard-first: prepare for keyboard shortcut system (future phase)
