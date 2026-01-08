import { createGlobalState } from '@vueuse/core';
import { ref, computed } from 'vue';

export interface UndoAction {
  description: string;
  callback: () => Promise<void>;
}

/**
 * Global undo stack manager
 * Stores a single undo action (not an array) that can be executed with the z key
 * New actions replace the previous undo
 */
export const useUndoStack = createGlobalState(() => {
  const undoAction = ref<UndoAction | null>(null);

  /**
   * Push a new undo action (replaces existing undo)
   */
  function pushUndo(description: string, callback: () => Promise<void>): void {
    undoAction.value = {
      description,
      callback,
    };
  }

  /**
   * Execute the undo action and clear it
   */
  async function executeUndo(): Promise<void> {
    if (!undoAction.value) {
      return;
    }

    const action = undoAction.value;
    undoAction.value = null; // Clear before executing

    try {
      await action.callback();
    } catch (error) {
      console.error('Undo failed:', error);
      throw error;
    }
  }

  /**
   * Check if there's an undo action available
   */
  const hasUndo = computed(() => undoAction.value !== null);

  /**
   * Get the description of the current undo action
   */
  const undoDescription = computed(() => undoAction.value?.description || null);

  /**
   * Clear the undo stack
   */
  function clearUndo(): void {
    undoAction.value = null;
  }

  return {
    pushUndo,
    executeUndo,
    hasUndo,
    undoDescription,
    clearUndo,
  };
});
