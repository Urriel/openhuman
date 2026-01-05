# OpenHuman Email Client

A modern, keyboard-first email client built with Tauri 2, Vue 3, and TypeScript.

## Tech Stack

- **Frontend**: Vue 3 (Composition API) + TypeScript
- **Build Tool**: Vite
- **Desktop Framework**: Tauri 2
- **Backend**: Rust
- **UI Framework**: Tailwind CSS v4 + shadcn-vue
- **Virtualization**: TanStack Virtual
- **Type-Safe IPC**: Custom pattern (see `docs/IPC_PATTERN.md`)

For complete tech stack details, see `agent-os/product/tech-stack.md`

## Development Setup

### Prerequisites

- Node.js (v18 or higher)
- Rust toolchain (latest stable)
- npm or yarn

### Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build
```

### Development Commands

```bash
# Start Vite dev server only
npm run dev

# Start Tauri development environment (desktop app)
npm run tauri dev

# Build frontend
npm run build

# Preview production build
npm run preview

# Type checking
npx vue-tsc --noEmit

# Build Rust backend
cargo build --manifest-path src-tauri/Cargo.toml
```

### Testing

```bash
# Run frontend tests
npm run test

# Run frontend tests with UI
npm run test:ui

# Run Rust backend tests
cargo test --manifest-path src-tauri/Cargo.toml
```

### Linting and Formatting

```bash
# Lint frontend code
npm run lint

# Fix linting issues
npm run lint:fix

# Format frontend code
npm run format

# Check formatting
npm run format:check

# Run Clippy on Rust code
npm run clippy

# Format Rust code
npm run rustfmt

# Check Rust formatting
npm run rustfmt:check
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

- **Lint Workflow** (`.github/workflows/lint.yml`): Runs ESLint, Prettier, TypeScript type checking, Clippy, and rustfmt
- **Test Workflow** (`.github/workflows/test.yml`): Runs Vitest for frontend and `cargo test` for backend
- **Build Workflow** (`.github/workflows/build.yml`): Verifies that both frontend and backend build successfully
- **Package Workflow** (`.github/workflows/package.yml`): Builds platform-specific installers for macOS (Intel & Apple Silicon), Windows, and Linux

All workflows run on push to `main` and on pull requests. Pre-commit hooks ensure code quality before commits.

### Creating a Release

To create a new release with packaged installers:

1. Create and push a tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. The Package workflow will automatically:
   - Build the app for all platforms (macOS Intel/ARM, Windows, Linux)
   - Create a draft GitHub release
   - Upload all installers to the release

3. Review the draft release and publish when ready

You can also manually trigger the package workflow from the GitHub Actions tab.

## GitHub Copilot Configuration

This project includes custom GitHub Copilot instructions in `.github/copilot-instructions.md` that define:
- Vue 3 Composition API conventions
- TypeScript strict mode requirements
- Rust coding patterns and conventions
- Tech stack guidelines (Tailwind v4, shadcn-vue, TanStack Virtual, Tauri 2)
- Keyboard-first design principles

## Type-Safe IPC Pattern

This project uses a type-safe pattern for communication between the frontend and backend. 

For detailed documentation on adding new commands and using the IPC system, see:
- **[IPC Pattern Documentation](docs/IPC_PATTERN.md)** - Complete guide to the type-safe IPC pattern
- **Example Implementation**: `src/components/GreetExample.vue` - Working example
- **Command Definitions**: `src-tauri/src/commands/` - Backend command handlers
- **Type Definitions**: `src/types/commands.ts` - Frontend type definitions

## Project Structure

```
openhuman/
├── src/                    # Frontend (Vue 3 + TypeScript)
│   ├── components/         # Vue components
│   ├── types/             # TypeScript type definitions
│   ├── assets/            # Static assets
│   ├── App.vue            # Root component
│   └── main.ts            # Entry point
├── src-tauri/             # Backend (Rust + Tauri)
│   ├── src/               # Rust source code
│   │   ├── commands/      # Tauri command handlers
│   │   ├── lib.rs         # Library entry
│   │   └── main.rs        # Binary entry
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── agent-os/              # Project specifications and planning
└── public/                # Public static files
```

## Path Aliases

TypeScript path aliases are configured for clean imports:

```typescript
import Component from '@/components/Component.vue'
import { SomeType } from '@/types/commands'
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

See LICENSE file for details.
