# Specification: Phase 0 - Bootstrap

## Goal
Establish the foundational architecture for the OpenHuman email client by scaffolding a Tauri 2 + Vue 3 + Vite project with Tailwind v4, shadcn-vue, TanStack Virtual, comprehensive development tooling (testing, linting, CI/CD, pre-commit hooks), and a type-safe IPC contract between frontend and backend.

## User Stories
- As a developer, I want a fully configured development environment so that I can immediately start building email features without setup friction
- As a developer, I want reusable UI components and patterns established so that future features have consistent foundations to build upon

## Specific Requirements

**Project Scaffolding with Tauri 2 + Vue 3 + Vite**
- Use `create-tauri-app` CLI to initialize project with Tauri 2, Vue 3, and Vite
- Configure TypeScript strict mode for frontend code
- Set up monorepo structure with `src/` for frontend and `src-tauri/` for Rust backend
- Ensure Vite configuration supports both Vue HMR and Tauri desktop builds
- Verify development server starts in under 5 seconds
- Configure Rust toolchain and verify `cargo build` works

**Tailwind CSS v4 Integration**
- Install `tailwindcss` and `@tailwindcss/vite` packages (stable v4 release)
- Use CSS-first configuration with `@import "tailwindcss"` in main CSS file
- Add `@tailwindcss/vite` plugin to `vite.config.ts`
- Verify Tailwind utilities render correctly in Vue components
- Do not create custom theme; use default Tailwind configuration

**shadcn-vue Component Library Setup**
- Initialize shadcn-vue with default theme (no custom theme)
- Install core UI primitives: Button, Input, Card, Dialog
- Configure Reka UI as the underlying primitive library
- Set up component aliases and import paths for easy access
- Verify components render with Tailwind v4 utilities

**TanStack Virtual - Reusable Virtualized List Component**
- Install `@tanstack/vue-virtual` package
- Create `VirtualList.vue` component wrapper using Vue 3 Composition API
- Component should accept generic item type via TypeScript generics
- Support customizable row height and item renderer slot
- Include basic example/demo component to verify 60 FPS scrolling with 10,000+ items
- Document component props and usage patterns for future developers

**Shared Types and IPC Contract**
- Create `/src-tauri/src/commands/` folder for Tauri command handlers in Rust
- Create `/src/types/` folder for shared TypeScript type definitions
- Implement example "greet" command with Rust handler and TypeScript types
- Frontend should invoke command and display result to verify IPC works
- Ensure type safety between frontend TypeScript and Rust backend
- Document IPC pattern for future command implementations

**Pre-commit Hooks with Husky and Lint-Staged**
- Install and configure `husky` for Git hooks
- Set up `lint-staged` to run linters only on staged files
- Configure hook to run ESLint on frontend `.ts` and `.vue` files
- Configure hook to run Clippy on Rust code in `src-tauri/`
- Ensure pre-commit checks complete in under 10 seconds

**Environment Variables Configuration**
- Create `.env.example` file with placeholder variables for future use
- Add `.env` to `.gitignore` to prevent committing secrets
- Document expected environment variable structure in comments
- Configure Vite to load environment variables with `VITE_` prefix

**CI/CD with GitHub Actions**
- Create workflow for linting and type checking (frontend ESLint + TypeScript, backend Clippy)
- Create workflow for running tests (Vitest for frontend, `cargo test` for backend)
- Create workflow for building application on push to main branch
- Target completion time under 5 minutes for basic checks
- Consider platform-specific builds (macOS, Windows, Linux) but defer full packaging

**GitHub Copilot Coding Agent Configuration**
- Create `.github/copilot-instructions.md` with project-specific coding standards
- Define conventions for Vue 3 Composition API, TypeScript strict mode, and Rust patterns
- Reference tech stack document for consistency (Tailwind v4, shadcn-vue, TanStack Virtual)
- Include keyboard-first design principle for future feature guidance

**Frontend Testing with Vitest and Vue Test Utils**
- Install `vitest`, `@vue/test-utils`, and related dependencies
- Configure `vitest.config.ts` for Vue component testing
- Create sample component test to verify setup works
- Set up test utilities and helpers directory structure
- Document testing patterns for future features

**Backend Testing with Rust Native Tests**
- Verify `cargo test` runs successfully
- Create sample unit test for the "greet" command handler
- Document Rust testing patterns and module structure
- Configure test output formatting

**Code Quality Tools**
- Configure ESLint with Vue plugin for frontend linting
- Configure Prettier for consistent code formatting (frontend)
- Enable TypeScript strict mode in `tsconfig.json`
- Configure Clippy (Rust linter) with recommended rules
- Configure rustfmt for Rust code formatting
- Ensure all tools integrate with pre-commit hooks

## Visual Design
No visual assets provided for this specification.

## Existing Code to Leverage

**Existing .gitignore patterns**
- Current `.gitignore` only contains `.opencode/elf`
- Extend with standard patterns for Node.js, Rust, Tauri, environment files, and build artifacts
- Include patterns for `.env`, `node_modules/`, `dist/`, `target/`, platform-specific files

## Out of Scope
- Actual email features (account management, sync engine, email list UI, reader, composer)
- Database or SQLite setup (covered in future phases)
- Email protocol implementations (POP3/SMTP libraries and logic)
- Authentication, secrets management, or keychain integration
- Application icons, branding, splash screens, or visual assets
- Production deployment configurations or code signing
- Performance optimization beyond basic setup verification
- Analytics, error tracking, monitoring, or telemetry
- Feature flags or A/B testing infrastructure
- Custom Tailwind theme (use default only)
- E2E testing setup with Playwright (defer to future phase)
- Automated release workflows or version bumping in CI/CD
