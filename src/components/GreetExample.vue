<script setup lang="ts">
import { ref } from 'vue';
import { invokeGreet } from '@/types/commands';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

/**
 * GreetExample Component
 *
 * Demonstrates the type-safe IPC pattern for invoking Tauri commands.
 *
 * This example shows:
 * 1. Using the type-safe wrapper function (invokeGreet)
 * 2. Proper error handling
 * 3. Loading state management
 * 4. TypeScript type safety
 */

const name = ref('');
const greeting = ref('');
const error = ref('');
const isLoading = ref(false);

/**
 * Invokes the greet command with error handling
 */
async function handleGreet() {
  if (!name.value.trim()) {
    error.value = 'Please enter a name';
    return;
  }

  try {
    isLoading.value = true;
    error.value = '';

    // Type-safe invocation - invokeGreet returns Promise<string>
    // TypeScript ensures we pass the correct parameter type
    greeting.value = await invokeGreet(name.value);
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to invoke greet command';
    greeting.value = '';
  } finally {
    isLoading.value = false;
  }
}

/**
 * Resets the form to initial state
 */
function reset() {
  name.value = '';
  greeting.value = '';
  error.value = '';
}
</script>

<template>
  <Card class="w-full max-w-md mx-auto">
    <CardHeader>
      <CardTitle>Type-Safe IPC Example</CardTitle>
      <CardDescription>
        Demonstrates invoking a Tauri command with full TypeScript type safety
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Input Section -->
      <div class="space-y-2">
        <label for="name-input" class="text-sm font-medium"> Enter your name </label>
        <Input
          id="name-input"
          v-model="name"
          placeholder="Enter a name..."
          @keyup.enter="handleGreet"
          :disabled="isLoading"
        />
      </div>

      <!-- Action Buttons -->
      <div class="flex gap-2">
        <Button @click="handleGreet" :disabled="isLoading || !name.trim()" class="flex-1">
          {{ isLoading ? 'Greeting...' : 'Greet' }}
        </Button>
        <Button variant="outline" @click="reset" :disabled="isLoading"> Reset </Button>
      </div>

      <!-- Error Display -->
      <div
        v-if="error"
        class="p-3 text-sm text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400 rounded-md"
        role="alert"
      >
        {{ error }}
      </div>

      <!-- Success Display -->
      <div
        v-if="greeting"
        class="p-3 text-sm text-green-600 bg-green-50 dark:bg-green-900/20 dark:text-green-400 rounded-md"
        role="status"
      >
        <strong>Response:</strong> {{ greeting }}
      </div>

      <!-- Code Example -->
      <details class="text-xs">
        <summary class="cursor-pointer text-sm font-medium mb-2">View Implementation Code</summary>
        <pre
          class="p-3 bg-gray-100 dark:bg-gray-800 rounded-md overflow-x-auto"
        ><code>// TypeScript (Frontend)
import { invokeGreet } from '@/types/commands';

const greeting = await invokeGreet('World');
// TypeScript knows greeting is a string

// Rust (Backend)
#[tauri::command]
pub fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}</code></pre>
      </details>
    </CardContent>
  </Card>
</template>
