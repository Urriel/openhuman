# Type-Safe IPC Pattern

This document describes the type-safe Inter-Process Communication (IPC) pattern used in OpenHuman to communicate between the frontend (Vue/TypeScript) and backend (Rust/Tauri).

## Overview

The IPC pattern ensures type safety across the frontend-backend boundary by:
1. Defining Rust command handlers with the `#[tauri::command]` attribute
2. Creating matching TypeScript type definitions
3. Providing type-safe wrapper functions for invoking commands
4. Implementing proper error handling on both sides

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend                             │
│  ┌──────────────────┐           ┌──────────────────┐        │
│  │  Vue Component   │  ────────▶│  Type-Safe       │        │
│  │                  │           │  Wrapper         │        │
│  │  GreetExample.vue│           │  invokeGreet()   │        │
│  └──────────────────┘           └──────────────────┘        │
│                                           │                  │
│                                           ▼                  │
│                                  ┌──────────────────┐        │
│                                  │  Tauri invoke    │        │
│                                  │  API             │        │
│                                  └──────────────────┘        │
└───────────────────────────────────────────┬─────────────────┘
                                            │ IPC
                                            ▼
┌─────────────────────────────────────────────────────────────┐
│                         Backend                              │
│                                  ┌──────────────────┐        │
│                                  │  Tauri Runtime   │        │
│                                  └──────────────────┘        │
│                                           │                  │
│                                           ▼                  │
│                                  ┌──────────────────┐        │
│                                  │  Command Handler │        │
│                                  │  greet()         │        │
│                                  └──────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Adding a New Command

Follow these steps to add a new type-safe command:

### 1. Create the Rust Command Handler

Create a new file in `src-tauri/src/commands/` (e.g., `my_command.rs`):

```rust
/// Documentation for your command
#[tauri::command]
pub fn my_command(param1: String, param2: i32) -> Result<MyResponse, String> {
    // Implementation
    Ok(MyResponse {
        field1: param1,
        field2: param2 * 2,
    })
}

// Include tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_command() {
        let result = my_command("test".to_string(), 5);
        assert!(result.is_ok());
    }
}
```

### 2. Register the Command Module

Update `src-tauri/src/commands/mod.rs`:

```rust
pub mod greet;
pub mod my_command;  // Add this line

pub use greet::greet;
pub use my_command::my_command;  // Add this line
```

### 3. Register in Tauri Builder

Update `src-tauri/src/lib.rs`:

```rust
use commands::{greet, my_command};  // Add my_command to imports

// In the run() function:
.invoke_handler(tauri::generate_handler![
    greet,
    my_command  // Add this line
])
```

### 4. Define TypeScript Types

Update `src/types/commands.ts`:

```typescript
// Request/Response types
export interface MyCommandRequest {
  param1: string;
  param2: number;
}

export interface MyCommandResponse {
  field1: string;
  field2: number;
}

// Type-safe wrapper function
export async function invokeMyCommand(
  param1: string,
  param2: number
): Promise<MyCommandResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<MyCommandResponse>('my_command', { param1, param2 });
}
```

### 5. Use in Vue Component

```vue
<script setup lang="ts">
import { ref } from 'vue';
import { invokeMyCommand } from '@/types/commands';

const result = ref<MyCommandResponse | null>(null);

async function handleInvoke() {
  try {
    result.value = await invokeMyCommand('test', 42);
    // TypeScript knows the exact shape of result.value
  } catch (err) {
    console.error('Command failed:', err);
  }
}
</script>
```

## Error Handling

### Backend (Rust)

Use `Result<T, E>` for operations that can fail:

```rust
#[tauri::command]
pub fn risky_operation(input: String) -> Result<String, String> {
    if input.is_empty() {
        return Err("Input cannot be empty".to_string());
    }
    Ok(format!("Processed: {}", input))
}
```

### Frontend (TypeScript)

Always wrap invocations in try-catch:

```typescript
try {
  const result = await invokeRiskyOperation(input);
  // Handle success
} catch (err) {
  // Handle error - Tauri will reject the promise if Rust returns Err
  const errorMessage = err instanceof Error ? err.message : 'Unknown error';
  console.error(errorMessage);
}
```

## Best Practices

1. **Keep commands focused**: Each command should do one thing well
2. **Use strong types**: Avoid `any` in TypeScript and generic types in Rust
3. **Document parameters**: Add JSDoc/rustdoc comments explaining what each parameter does
4. **Handle errors gracefully**: Always return `Result<T, E>` for fallible operations
5. **Write tests**: Test both the Rust handler and the TypeScript wrapper
6. **Use wrapper functions**: Always provide a typed wrapper instead of using `invoke` directly
7. **Validate input**: Check parameters on both frontend and backend
8. **Keep it async**: All commands are async from the frontend's perspective

## Example: Complete Command Implementation

See the `greet` command for a complete reference implementation:

- **Rust**: `src-tauri/src/commands/greet.rs`
- **TypeScript**: `src/types/commands.ts`
- **Vue Example**: `src/components/GreetExample.vue`

## Common Patterns

### Command with Complex Return Type

```rust
#[derive(serde::Serialize)]
pub struct ComplexResponse {
    items: Vec<String>,
    count: usize,
    metadata: HashMap<String, String>,
}

#[tauri::command]
pub fn get_complex_data() -> Result<ComplexResponse, String> {
    // Implementation
}
```

```typescript
export interface ComplexResponse {
  items: string[];
  count: number;
  metadata: Record<string, string>;
}
```

### Command with Optional Parameters

```rust
#[tauri::command]
pub fn search(query: String, limit: Option<i32>) -> Vec<String> {
    let max = limit.unwrap_or(10);
    // Implementation
}
```

```typescript
export async function invokeSearch(
  query: string,
  limit?: number
): Promise<string[]> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<string[]>('search', { query, limit });
}
```

## Testing

### Backend Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_success() {
        let result = my_command("test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_error_handling() {
        let result = my_command("".to_string());
        assert!(result.is_err());
    }
}
```

Run with: `cargo test`

### Frontend Tests (Vitest)

```typescript
import { describe, it, expect, vi } from 'vitest';
import { invokeMyCommand } from '@/types/commands';

// Mock the Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('invokeMyCommand', () => {
  it('should invoke command with correct parameters', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue({ field1: 'test', field2: 84 });

    const result = await invokeMyCommand('test', 42);
    
    expect(invoke).toHaveBeenCalledWith('my_command', {
      param1: 'test',
      param2: 42,
    });
    expect(result).toEqual({ field1: 'test', field2: 84 });
  });
});
```

Run with: `npm run test`

## Troubleshooting

### Command Not Found

If you get "command not found" errors:

1. Check that the command is registered in `commands/mod.rs`
2. Verify it's included in `tauri::generate_handler![]` in `lib.rs`
3. Ensure the command name matches exactly (case-sensitive)

### Type Mismatches

If TypeScript types don't match Rust:

1. Verify parameter names match exactly
2. Check type conversions (e.g., `i32` → `number`, `String` → `string`)
3. Use `#[derive(serde::Serialize)]` for return types
4. Use `#[derive(serde::Deserialize)]` for parameter types

### Performance Issues

For commands that take a long time:

1. Consider using Tauri events for progress updates
2. Move heavy work to a separate thread
3. Use streaming responses for large datasets

## Resources

- [Tauri Commands Documentation](https://tauri.app/develop/calling-rust/)
- [Serde Serialization](https://serde.rs/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/)
