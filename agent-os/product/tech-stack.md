# Tech Stack

## Application Framework & Runtime

### Desktop Application

- **Framework:** Tauri 2
- **Language (Backend):** Rust
- **Language (Frontend):** TypeScript
- **Package Manager (Frontend):** npm / pnpm
- **Build Tool:** Vite

## Frontend

### Core Framework

- **JavaScript Framework:** Vue 3 (Composition API)
- **Build Tool:** Vite
- **Language:** TypeScript

### UI & Styling

- **CSS Framework:** Tailwind CSS v4
- **Component Library:** shadcn-vue
- **UI Primitives:** Reka UI (via shadcn-vue)

### Performance & Optimization

- **Virtual Scrolling:** TanStack Virtual (Vue)
- **State Management:** Vue Composition API (Pinia if needed)
- **Form Handling:** Vue reactivity system

## Backend (Tauri/Rust)

### Database & Storage

- **Database:** SQLite (backend-owned)
- **ORM/Query Builder:** sqlx or rusqlite
- **Migrations:** Custom migration system or refinery

### Email Protocols

- **Email Fetching:** POP3 (custom or rust-pop3 library)
- **Email Sending:** SMTP (lettre crate)
- **MIME Parsing:** mailparse or mail-parser crate
- **Message Threading:** Custom threading algorithm

### Security & Secrets

- **Secrets Storage:** OS keychain integration (keyring-rs or tauri-plugin-keychain)
- **Encryption:** AES-256 for local data (if needed)

### Background Jobs & Scheduling

- **Job Queue:** Custom background job system
- **Scheduler:** tokio timers for snooze/send-later features

### IPC (Inter-Process Communication)

- **IPC Layer:** Tauri commands and events
- **Type Safety:** Shared TypeScript types between frontend and Rust backend

## Search & Indexing

### Text Search

- **Full-Text Search:** SQLite FTS5 (Full-Text Search extension)
- **Semantic Search (Optional AI):** Vector embeddings with tantivy or custom implementation

## AI Features (Optional)

### AI Services

- **LLM Integration:** OpenAI API, Anthropic Claude, or local models
- **AI Gateway:** Custom Rust service to proxy AI requests
- **Embeddings:** text-embedding-ada-002 or similar for semantic search

## Testing & Quality

### Frontend Testing

- **Unit Tests:** Vitest
- **Component Tests:** Vue Test Utils + Vitest
- **E2E Tests:** Playwright or Tauri's built-in testing

### Backend Testing

- **Unit Tests:** Rust native testing (cargo test)
- **Integration Tests:** Rust integration test modules

### Code Quality

- **Frontend Linting:** ESLint + Vue ESLint plugin
- **Frontend Formatting:** Prettier
- **Backend Linting:** Clippy (Rust linter)
- **Backend Formatting:** rustfmt
- **Type Checking:** TypeScript (strict mode), Rust compiler

## Build & Distribution

### Packaging

- **Desktop Bundler:** Tauri bundler
- **Supported Platforms:** macOS, Windows, Linux
- **Code Signing:** Platform-specific signing (Apple Developer, Windows cert)

### Updates

- **Auto-Update:** Tauri updater plugin
- **Release Management:** GitHub Releases or custom update server

## Development Tools

### Development Environment

- **Hot Reload:** Vite HMR for frontend, cargo watch for backend
- **Debugging:** Chrome DevTools (frontend), rust-lldb/gdb (backend)

### Version Control

- **VCS:** Git
- **Branching Strategy:** GitHub Flow or Git Flow

## Performance Budgets

### Frontend

- **Initial Load:** <2s to interactive
- **Virtual List:** 60 FPS scrolling with 10,000+ items
- **Search Response:** <100ms for local search

### Backend

- **Email Sync:** Process 1,000 emails in <10s
- **Database Queries:** <50ms for inbox list queries
- **SMTP Send:** <3s per email (network dependent)

## Architecture Principles

1. **Local-First:** All data stored locally in SQLite, offline-capable
2. **Provider-Agnostic:** Work with any POP3/SMTP provider
3. **Keyboard-First:** Every action accessible via keyboard
4. **Fast by Default:** Virtual scrolling, indexed search, incremental sync
5. **Privacy-Focused:** No telemetry, optional read tracking, local AI options

## Design Principles

OpenHuman follows a coherent design philosophy that prioritizes power users, speed, and keyboard-first workflows. See `agent-os/product/design-principles.md` for comprehensive guidelines.

**Core Principles:**

1. **Keyboard First, Mouse Optional** - The UI feels closer to a terminal or code editor than a web app. Every action has a keyboard shortcut, command palette access, and visible keyboard hints.

2. **Density Over Whitespace** - Information density is high but organized—no excessive padding, but clear visual hierarchy. Show 15-20 emails per screen, use compact layouts (8-12px padding), and efficient use of screen real estate.

3. **AI as a Collaborator, Not a Replacement** - AI drafts, suggests, and refines, but the human always makes the final call. All AI suggestions are clearly labeled with accept/reject options, and no AI actions are automatic.

4. **Speed Through Reduction** - Remove what slows you down (heavy modals, redundant UI elements, visual clutter), keep what accelerates (shortcuts, previews, smart defaults). Target <2 seconds to archive an email, <5 seconds for quick replies.

**Target Audience:** Power users—founders, sales teams, customer success managers, and executives who measure productivity in emails-per-hour and see email as a high-stakes communication channel, not a notification feed.

**Implementation:** All UI components must pass the design principles checklist (keyboard shortcuts, information density, AI transparency, speed optimization) before being considered complete.
