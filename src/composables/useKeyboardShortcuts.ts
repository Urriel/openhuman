import { ref, type Ref } from 'vue';
import { useEventListener } from '@vueuse/core';

export type ViewContext = 'inbox' | 'thread' | 'compose' | 'global';

export interface ShortcutContext {
  view?: ViewContext | ViewContext[];
}

export interface ShortcutHandler {
  callback: () => void | Promise<void>;
  context?: ShortcutContext;
  global?: boolean;
}

const SEQUENCE_TIMEOUT = 1000; // 1 second timeout for sequential keys

/**
 * Global keyboard shortcuts manager
 * Handles single keys, modifier combos, and sequential key combinations
 */
export function useKeyboardShortcuts(currentContext: Ref<ViewContext> = ref('global')) {
  const shortcuts = new Map<string, ShortcutHandler>();
  const sequenceBuffer = ref<string>('');
  const sequenceTimeout = ref<number | null>(null);

  /**
   * Check if an input field is currently focused
   */
  function isInputFocused(): boolean {
    const activeElement = document.activeElement;
    if (!activeElement) return false;

    const tagName = activeElement.tagName.toLowerCase();
    return (
      tagName === 'input' ||
      tagName === 'textarea' ||
      tagName === 'select' ||
      (activeElement as HTMLElement).isContentEditable
    );
  }

  /**
   * Check if shortcut matches the current context
   */
  function matchesContext(handler: ShortcutHandler): boolean {
    if (!handler.context?.view) return true;

    const views = Array.isArray(handler.context.view)
      ? handler.context.view
      : [handler.context.view];

    return views.includes(currentContext.value);
  }

  /**
   * Clear the sequence buffer
   */
  function clearSequence() {
    sequenceBuffer.value = '';
    if (sequenceTimeout.value !== null) {
      clearTimeout(sequenceTimeout.value);
      sequenceTimeout.value = null;
    }
  }

  /**
   * Handle keyboard events
   */
  useEventListener('keydown', async (event: KeyboardEvent) => {
    const key = event.key;

    // Build shortcut key string
    let shortcutKey = '';

    // Check for modifier combos
    if (event.metaKey || event.ctrlKey) {
      const modifier = event.metaKey ? 'Cmd' : 'Ctrl';
      // Normalize letter keys to uppercase for consistency
      const normalizedKey = key.length === 1 ? key.toUpperCase() : key;
      if (event.shiftKey) {
        shortcutKey = `${modifier}+Shift+${normalizedKey}`;
      } else {
        shortcutKey = `${modifier}+${normalizedKey}`;
      }
    } else if (event.shiftKey && key.length === 1 && key !== key.toLowerCase()) {
      // Shift + letter (e.g., Shift+I)
      shortcutKey = `Shift+${key.toUpperCase()}`;
    } else {
      shortcutKey = key;
    }

    // Check if this is a global shortcut (Cmd+K, Escape)
    const handler = shortcuts.get(shortcutKey);
    const isGlobalShortcut = handler?.global || shortcutKey === 'Escape';

    // Prevent shortcuts when input is focused (except global shortcuts)
    if (isInputFocused() && !isGlobalShortcut) {
      return;
    }

    // Handle sequential keys (e.g., g→i)
    if (sequenceBuffer.value) {
      const sequenceKey = `${sequenceBuffer.value}→${key}`;
      const sequenceHandler = shortcuts.get(sequenceKey);

      if (sequenceHandler && matchesContext(sequenceHandler)) {
        event.preventDefault();
        clearSequence();
        await sequenceHandler.callback();
        return;
      }

      // If no match, clear the sequence and continue
      clearSequence();
    }

    // Start new sequence if key is 'g'
    if (key === 'g' && !event.metaKey && !event.ctrlKey && !event.altKey) {
      sequenceBuffer.value = 'g';
      sequenceTimeout.value = window.setTimeout(() => {
        clearSequence();
      }, SEQUENCE_TIMEOUT);
      return;
    }

    // Handle regular shortcuts
    if (handler && matchesContext(handler)) {
      event.preventDefault();
      await handler.callback();
    }
  });

  /**
   * Register a keyboard shortcut
   */
  function register(
    key: string,
    callback: () => void | Promise<void>,
    options?: ShortcutContext & { global?: boolean }
  ): void {
    shortcuts.set(key, {
      callback,
      context: options ? { view: options.view } : undefined,
      global: options?.global,
    });
  }

  /**
   * Unregister a keyboard shortcut
   */
  function unregister(key: string): void {
    shortcuts.delete(key);
  }

  /**
   * Clear all shortcuts
   */
  function clear(): void {
    shortcuts.clear();
    clearSequence();
  }

  return {
    register,
    unregister,
    clear,
  };
}
