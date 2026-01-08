# AGENTS.md - OpenHuman Development Guide

**For AI coding agents working in this repository**

## Project Overview

OpenHuman is a local-first email client built with Tauri 2 (Rust backend) and Vue 3 (TypeScript frontend). The application prioritizes keyboard-first navigation, performance, and privacy.

**Design Philosophy:** OpenHuman follows a Superhuman-inspired design approach optimized for power users. See `agent-os/product/design-principles.md` for comprehensive guidelines.

**Core Principles:**

1. **Keyboard First, Mouse Optional** - Every action has a keyboard shortcut
2. **Density Over Whitespace** - High information density with clear visual hierarchy
3. **AI as Collaborator, Not Replacement** - AI suggests, human decides
4. **Speed Through Reduction** - Remove friction, keep acceleration

## Quick Reference

### Essential Commands

**Development:**

```bash
npm run dev                  # Frontend dev server (Vite)
npm run tauri dev            # Full desktop app with hot reload
```

**Testing:**

```bash
npm run test                 # Run all frontend tests (Vitest)
npm run test:ui              # Test UI with file filtering
vitest path/to/file.test.ts  # Run single test file
cargo test --manifest-path src-tauri/Cargo.toml              # All Rust tests
cargo test --manifest-path src-tauri/Cargo.toml test_name    # Single Rust test
```

**Code Quality:**

```bash
npm run lint                 # ESLint check (.vue, .ts files)
npm run lint:fix             # ESLint auto-fix
npm run format               # Prettier format
npm run format:check         # Prettier check only
npm run clippy               # Rust linter (strict mode)
npm run rustfmt              # Rust formatter
vue-tsc --noEmit             # TypeScript type checking
```

**Building:**

```bash
npm run build                # Frontend production build
npm run tauri build          # Full app with installer
```

**Pre-commit:** Runs automatically via husky + lint-staged (ESLint, Prettier, rustfmt on staged files)

---

## COVE Method (Chain of Verification)

**All AI agents must follow this verification methodology for code changes:**

### 1. Draft Initial Response

- Analyze the request and plan your implementation
- Write the initial code solution
- Document assumptions made

### 2. Plan Verification Questions

Generate specific questions to verify your implementation:

- **Type Safety:** "Are all TypeScript types correctly defined? Do Rust types match IPC signatures?"
- **Error Handling:** "Are all error cases handled? Do error messages follow our user-friendly guidelines?"
- **Testing:** "Do existing tests still pass? Are new tests needed for critical paths?"
- **Style Compliance:** "Does the code follow naming conventions? Is formatting correct?"
- **Architecture:** "Does this follow our IPC pattern? Are components properly structured?"
- **Performance:** "Will this work with 1000+ items? Are queries optimized?"
- **Accessibility:** "Is keyboard navigation working? Are ARIA labels present?"
- **Design Principles:** "Does this follow our keyboard-first, high-density, speed-focused design principles?"

### 3. Answer Verification Questions

Answer each question independently by:

- Reading the actual code you wrote
- Checking against guidelines in this file
- Running relevant commands (`npm run lint`, `cargo clippy`, tests)
- Verifying file locations match directory structure

### 4. Generate Final Verified Response

- Fix any issues found during verification
- Run all relevant quality checks
- Provide summary of changes and verification results
- Note any remaining concerns or follow-up needed

**Example Verification Checklist:**

```markdown
✓ TypeScript strict mode compliance
✓ Path aliases used (@/ not ../)
✓ Error handling with try-catch
✓ Component follows <script setup> pattern
✓ Tailwind classes (no custom CSS)
✓ Tests pass (npm run test)
✓ Linting passes (npm run lint)
✓ Types verified (vue-tsc --noEmit)
```

---

## Tech Stack

**Frontend:** Vue 3 (Composition API + `<script setup>`), TypeScript (strict), Vite, Tailwind CSS v4, shadcn-vue, TanStack Virtual  
**Backend:** Tauri 2, Rust (edition 2021), SQLite (sqlx), tokio, thiserror  
**Testing:** Vitest + @vue/test-utils (frontend), cargo test (backend)  
**Linting:** ESLint 9 (flat config), Clippy, Prettier, rustfmt

---

## Code Style Guidelines

### TypeScript/Vue

**Imports:**

- Use path aliases: `@/` for `src/`, `@/components/*`, `@/types/*`
- Type-only imports: `import type { SomeType } from '@/types'`
- Organize: external deps → internal modules → types → relative
- Single quotes, semicolons required (Prettier enforced)

**Formatting:**

- 2-space indentation, 100-char line length
- Arrow functions: omit parens for single param (`x => x * 2`)
- Trailing commas: ES5 style

**Types:**

- Strict mode enabled: no implicit `any`, unused locals/params are errors
- Use `interface` for object shapes, `type` for unions/primitives
- Avoid `any`; use `unknown` when truly dynamic
- Explicit return types on exported functions
- Props: `defineProps<PropsInterface>()` with TypeScript interface

**Naming:**

- Components: PascalCase (`EmailList.vue`, `ComposeDialog.vue`)
- Composables: camelCase with `use` prefix (`useEmailSync.ts`)
- Variables/functions: camelCase (`fetchEmails`, `emailCount`)
- Constants: UPPER_SNAKE_CASE (`MAX_RESULTS`)
- Files: Match component name or kebab-case for non-components

**Vue Components:**

```vue
<script setup lang="ts">
import { ref } from 'vue';
import type { Email } from '@/types';

interface Props {
  emails: Email[];
  pageSize?: number;
}

const props = withDefaults(defineProps<Props>(), {
  pageSize: 50,
});

const isLoading = ref(false);

async function handleAction() {
  try {
    isLoading.value = true;
    // logic here
  } catch (err) {
    // User-friendly error handling
    error.value = err instanceof Error ? err.message : 'Unknown error';
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <Button @click="handleAction" :disabled="isLoading">
      {{ isLoading ? 'Loading...' : 'Submit' }}
    </Button>
  </div>
</template>
```

**Tailwind CSS:**

- Use utility classes directly in templates
- Conditional classes: use `cn()` utility (`clsx` + `tailwind-merge`)
- Prefer shadcn-vue components over custom UI elements
- Dark mode: `dark:` prefix, responsive: `md:`, `lg:` prefixes

### Rust

**Naming:**

- Functions: snake_case (`fetch_emails`, `send_email`)
- Structs/Enums: PascalCase (`EmailMessage`, `DatabaseError`)
- Constants: UPPER_SNAKE_CASE (`MAX_RETRIES`)

**Error Handling:**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Failed to connect: {0}")]
    ConnectionError(String),
    #[error("Invalid credentials")]
    AuthError,
}

pub type EmailResult<T> = Result<T, EmailError>;

// Use Result for all fallible operations
#[tauri::command]
pub fn send_email(to: &str) -> EmailResult<String> {
    validate_email(to)?;
    Ok("Email sent".to_string())
}
```

**Documentation:**

````rust
/// Fetches emails from the server via IMAP.
///
/// # Example (TypeScript)
/// ```typescript
/// const emails = await invoke<Email[]>('fetch_emails', { folder: 'INBOX' });
/// ```
#[tauri::command]
pub fn fetch_emails(folder: &str) -> EmailResult<Vec<Email>> {
    // implementation
}
````

---

## IPC Pattern (Frontend ↔ Backend)

**Type-safe command invocation:**

1. **Backend** - Define command handler:

```rust
// src-tauri/src/commands/example.rs
#[tauri::command]
pub fn example_command(param: String) -> Result<String, String> {
    Ok(format!("Processed: {}", param))
}
```

2. **Backend** - Register in `lib.rs`:

```rust
.invoke_handler(tauri::generate_handler![example_command])
```

3. **Frontend** - Define types:

```typescript
// src/types/commands.ts
export interface ExampleRequest {
  param: string;
}
export type ExampleResponse = string;
```

4. **Frontend** - Invoke with type safety:

```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke<ExampleResponse>('example_command', { param: 'test' });
```

**See:** `docs/IPC_PATTERN.md` for comprehensive guide

---

## Error Handling

**Frontend:**

- Try-catch around async Tauri commands
- User-friendly messages (not technical details)
- Display errors in UI with proper ARIA roles
- Type-safe error checking: `err instanceof Error`

**Backend:**

- Custom error types with `thiserror`
- Return `Result<T, E>` for all fallible operations
- Descriptive error messages (security-conscious)
- Convert external errors using `From` trait

**Best Practices:**

- Fail fast with clear messages
- Graceful degradation for non-critical failures
- Clean up resources (use RAII in Rust, finally in TypeScript)

---

## Testing Standards

**Strategy:**

- Write minimal tests during development
- Focus on core user flows and critical paths
- Defer edge cases unless business-critical
- Test behavior, not implementation

**Frontend (Vitest):**

```typescript
import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';

describe('EmailList', () => {
  it('renders email items', () => {
    const wrapper = mount(EmailList, { props: { emails: mockEmails } });
    expect(wrapper.findAll('.email-item')).toHaveLength(mockEmails.length);
  });
});
```

**Backend (Rust):**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
    }
}
```

---

## Key Architectural Patterns

**Virtual Scrolling:**

- Use `VirtualList.vue` for lists with 100+ items
- Required for: email inbox, search results, folder views
- Provide consistent item heights for best performance

**State Management:**

- Vue Composition API for local component state
- Pinia only when global state needs sharing across components
- Keep state close to where it's used
- Use provide/inject for deep hierarchies

**Component Architecture:**

- shadcn-vue components in `src/components/ui/`
- Uses CVA (class-variance-authority) for variants
- Reka UI primitives for accessibility

---

## Design Principles

**Keyboard-First:**

- Every UI action accessible via keyboard
- Proper focus management and tab order
- Visual focus indicators required

**Performance:**

- Target 60 FPS scrolling
- Optimize for <50ms database queries (Rust)
- Use virtual scrolling for large lists

**Privacy & Local-First:**

- All data in local SQLite
- No telemetry/tracking
- Optional features are opt-in

**Accessibility:**

- Semantic HTML, ARIA labels
- Keyboard navigation everywhere
- Test with screen readers for new features

---

## Directory Structure

```
src/                    # Frontend (Vue 3 + TypeScript)
├── components/         # Vue components
│   └── ui/            # shadcn-vue components
├── types/             # TypeScript definitions
├── lib/               # Utilities (cn, etc.)
├── App.vue            # Root component
└── main.ts            # Entry point

src-tauri/             # Backend (Rust)
├── src/
│   ├── commands/      # Tauri command handlers
│   ├── email/         # Email protocol implementations
│   ├── error.rs       # Custom error types
│   ├── db.rs          # Database layer
│   └── lib.rs         # Library entry

agent-os/              # Standards & specifications
├── standards/         # Coding standards (global, frontend, backend, testing)
├── product/           # Product docs (tech stack, roadmap, mission)
└── specs/             # Feature specifications

docs/                  # Documentation
└── IPC_PATTERN.md     # Type-safe IPC guide
```

---

## Additional Resources

- **Copilot Instructions:** `.github/copilot-instructions.md` (comprehensive coding guidelines)
- **Tech Stack Details:** `agent-os/product/tech-stack.md`
- **Coding Standards:** `agent-os/standards/` (global, frontend, backend, testing)
- **IPC Guide:** `docs/IPC_PATTERN.md`
- **README:** Project setup, commands, architecture overview
