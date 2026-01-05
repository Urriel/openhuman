# GitHub Copilot Instructions for OpenHuman

## Project Overview
OpenHuman is a local-first email client built with Tauri 2, Vue 3, and Rust. The application prioritizes keyboard-first navigation, performance, and privacy.

## Tech Stack

### Frontend
- **Framework:** Vue 3 with Composition API (script setup)
- **Language:** TypeScript (strict mode enabled)
- **Build Tool:** Vite
- **CSS Framework:** Tailwind CSS v4 (CSS-first configuration)
- **Component Library:** shadcn-vue (built on Reka UI primitives)
- **Virtual Scrolling:** TanStack Virtual for large lists
- **State Management:** Vue Composition API (use Pinia only when necessary)

### Backend
- **Framework:** Tauri 2
- **Language:** Rust
- **IPC:** Tauri commands and events
- **Type Safety:** Shared TypeScript types between frontend and backend

## Vue 3 Coding Conventions

### Component Structure
- Always use `<script setup lang="ts">` syntax
- Use Composition API with composables for reusable logic
- Define props using `defineProps<T>()` with TypeScript interface
- Define emits using `defineEmits<T>()` with TypeScript interface
- Use `ref` for reactive primitives, `reactive` for objects (prefer `ref` for consistency)

### TypeScript
- Enable strict mode in all TypeScript files
- Define explicit types for all props, emits, and function signatures
- Use interfaces for complex types, type aliases for unions/primitives
- Avoid `any` type; use `unknown` when type is truly dynamic

### Naming Conventions
- Components: PascalCase (e.g., `EmailList.vue`, `ComposeDialog.vue`)
- Composables: camelCase with `use` prefix (e.g., `useEmailSync.ts`)
- Constants: UPPER_SNAKE_CASE (e.g., `MAX_RESULTS`)
- Variables/functions: camelCase (e.g., `fetchEmails`, `emailCount`)

## Rust Coding Conventions

### Code Organization
- Place Tauri command handlers in `src-tauri/src/commands/` directory
- Create separate modules for domain logic (e.g., `email/`, `sync/`)
- Use descriptive names for commands (e.g., `fetch_emails`, `send_email`)

### Error Handling
- Use `Result<T, E>` for all fallible operations
- Define custom error types using `thiserror` crate
- Return descriptive error messages to frontend via IPC

### Naming Conventions
- Functions: snake_case (e.g., `fetch_emails`)
- Structs/Enums: PascalCase (e.g., `EmailMessage`, `SyncStatus`)
- Constants: UPPER_SNAKE_CASE (e.g., `MAX_RETRIES`)
- Follow Rust API guidelines for public APIs

## Styling with Tailwind v4

### Configuration
- Use CSS-first configuration with `@import "tailwindcss"` in CSS files
- Use default Tailwind theme (no custom theme modifications)
- Prefer utility classes over custom CSS

### Best Practices
- Use Tailwind utilities directly in Vue templates
- Leverage shadcn-vue components for consistent UI patterns
- Use `class-variance-authority` (CVA) for component variants
- Use `tailwind-merge` to handle conditional class merging

## Component Library (shadcn-vue)

### Usage
- Prefer shadcn-vue components over custom implementations
- Available components: Button, Input, Card, Dialog, and more
- Components are built on Reka UI primitives for accessibility
- Customize via Tailwind utilities, not by modifying component source

## Virtual Scrolling (TanStack Virtual)

### When to Use
- Use `VirtualList.vue` component for any list with 100+ items
- Required for email inbox, search results, folder views

### Best Practices
- Provide consistent item heights for best performance
- Use dynamic heights only when necessary (degrades performance)
- Implement proper key generation for list items

## IPC Pattern (Frontend ↔ Backend)

### Command Structure
1. **Backend:** Create command handler in `src-tauri/src/commands/`
2. **Backend:** Register command in `main.rs` Tauri builder
3. **Frontend:** Define TypeScript types in `src/types/commands.ts`
4. **Frontend:** Invoke command using `import { invoke } from '@tauri-apps/api/core'`

### Example
```rust
// src-tauri/src/commands/example.rs
#[tauri::command]
pub fn example_command(param: String) -> Result<String, String> {
    Ok(format!("Processed: {}", param))
}
```

```typescript
// src/types/commands.ts
export interface ExampleRequest {
  param: string;
}
export type ExampleResponse = string;

// src/composables/useExample.ts
import { invoke } from '@tauri-apps/api/core';

const result = await invoke<ExampleResponse>('example_command', { param: 'test' });
```

## Design Principles

### Keyboard-First Design
- Every UI action must be accessible via keyboard
- Implement keyboard shortcuts for common operations
- Use proper focus management and tab order
- Provide visual focus indicators

### Performance
- Target 60 FPS scrolling for all virtualized lists
- Keep frontend bundle size minimal
- Use code splitting for large features
- Optimize Rust backend for <50ms database queries

### Privacy & Local-First
- All data stored locally in SQLite
- No telemetry or tracking
- Optional features should be opt-in
- Prefer local AI models over cloud APIs when possible

## Testing Standards

### Frontend Tests (Vitest)
- Write unit tests for composables and utilities
- Write component tests for complex components
- Use `@vue/test-utils` for component testing
- Aim for >80% coverage on critical paths

### Backend Tests (Rust)
- Write unit tests for all command handlers
- Write integration tests for complex workflows
- Use `#[cfg(test)]` modules for test organization
- Test error cases explicitly

## Code Quality

### Linting & Formatting
- Run ESLint before committing (pre-commit hook)
- Use Prettier for consistent formatting
- Run Clippy on Rust code (strict mode)
- Use rustfmt for Rust code formatting

### Git Workflow
- Write clear, descriptive commit messages
- Keep commits focused on a single change
- Use conventional commits format (optional but preferred)
- Run pre-commit hooks (husky + lint-staged)

## Documentation

### Code Comments
- Document complex logic and non-obvious decisions
- Use JSDoc for public functions and composables
- Add inline comments for Rust public APIs
- Keep comments concise and up-to-date

### Component Documentation
- Document component props, emits, and slots
- Provide usage examples for complex components
- Document keyboard shortcuts in component comments

## Additional Guidelines

### Security
- Never commit secrets or API keys
- Use `.env` files for environment variables (add to `.gitignore`)
- Validate all user input on both frontend and backend
- Use Tauri's security features (CSP, allowlist)

### Accessibility
- Use semantic HTML elements
- Provide ARIA labels for interactive elements
- Ensure keyboard navigation works everywhere
- Test with screen readers when implementing new features

### State Management
- Prefer Vue Composition API for local component state
- Use Pinia only for global state that needs to be shared
- Keep state as close to where it's used as possible
- Avoid prop drilling by using provide/inject for deep hierarchies

## Resources

- **Tech Stack Document:** `agent-os/product/tech-stack.md`
- **Vue 3 Docs:** https://vuejs.org/guide/
- **Tailwind v4 Docs:** https://tailwindcss.com/docs
- **Tauri 2 Docs:** https://v2.tauri.app/
- **TanStack Virtual Docs:** https://tanstack.com/virtual/latest
