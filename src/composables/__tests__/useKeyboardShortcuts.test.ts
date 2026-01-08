import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { ref, nextTick } from 'vue';
import { useKeyboardShortcuts } from '../useKeyboardShortcuts';

describe('useKeyboardShortcuts', () => {
  let cleanupFns: (() => void)[] = [];

  beforeEach(() => {
    cleanupFns = [];
  });

  afterEach(() => {
    cleanupFns.forEach(fn => fn());
    cleanupFns = [];
  });

  function dispatchKeydown(key: string, modifiers: Partial<KeyboardEvent> = {}) {
    const event = new KeyboardEvent('keydown', {
      key,
      bubbles: true,
      cancelable: true,
      ...modifiers,
    });
    document.dispatchEvent(event);
    return event;
  }

  describe('suspension behavior', () => {
    it('should execute shortcuts when not suspended', async () => {
      const isSuspended = ref(false);
      const callback = vi.fn();

      const { register, clear } = useKeyboardShortcuts(ref('global'), isSuspended);
      cleanupFns.push(clear);

      register('g→i', callback, { global: true });

      // Dispatch 'g' then 'i'
      dispatchKeydown('g');
      await nextTick();
      dispatchKeydown('i');
      await nextTick();

      expect(callback).toHaveBeenCalledTimes(1);
    });

    it('should block all shortcuts except Escape and Cmd+K when suspended', async () => {
      const isSuspended = ref(true);
      const navigationCallback = vi.fn();
      const composeCallback = vi.fn();
      const cmdKCallback = vi.fn();
      const escapeCallback = vi.fn();

      const { register, clear } = useKeyboardShortcuts(ref('global'), isSuspended);
      cleanupFns.push(clear);

      register('g→i', navigationCallback, { global: true });
      register('c', composeCallback, { global: true });
      register('Cmd+K', cmdKCallback, { global: true });
      register('Escape', escapeCallback, { global: true });

      // Test blocked shortcuts
      dispatchKeydown('g');
      await nextTick();
      dispatchKeydown('i');
      await nextTick();
      expect(navigationCallback).not.toHaveBeenCalled();

      dispatchKeydown('c');
      await nextTick();
      expect(composeCallback).not.toHaveBeenCalled();

      // Test allowed shortcuts
      dispatchKeydown('k', { metaKey: true });
      await nextTick();
      expect(cmdKCallback).toHaveBeenCalledTimes(1);

      dispatchKeydown('Escape');
      await nextTick();
      expect(escapeCallback).toHaveBeenCalledTimes(1);
    });

    it('should clear sequence buffer when suspended', async () => {
      const isSuspended = ref(false);
      const callback = vi.fn();

      const { register, clear } = useKeyboardShortcuts(ref('global'), isSuspended);
      cleanupFns.push(clear);

      register('g→i', callback, { global: true });

      // Start sequence
      dispatchKeydown('g');
      await nextTick();

      // Suspend while in sequence
      isSuspended.value = true;
      await nextTick();

      // Try to complete sequence
      dispatchKeydown('i');
      await nextTick();

      expect(callback).not.toHaveBeenCalled();

      // Unsuspend and verify sequence works again
      isSuspended.value = false;
      await nextTick();

      dispatchKeydown('g');
      await nextTick();
      dispatchKeydown('i');
      await nextTick();

      expect(callback).toHaveBeenCalledTimes(1);
    });

    it('should allow toggling suspension state dynamically', async () => {
      const isSuspended = ref(false);
      const callback = vi.fn();

      const { register, clear } = useKeyboardShortcuts(ref('global'), isSuspended);
      cleanupFns.push(clear);

      register('c', callback, { global: true });

      // Works when not suspended
      dispatchKeydown('c');
      await nextTick();
      expect(callback).toHaveBeenCalledTimes(1);

      // Suspend
      isSuspended.value = true;
      await nextTick();

      // Blocked when suspended
      dispatchKeydown('c');
      await nextTick();
      expect(callback).toHaveBeenCalledTimes(1); // Still 1

      // Unsuspend
      isSuspended.value = false;
      await nextTick();

      // Works again
      dispatchKeydown('c');
      await nextTick();
      expect(callback).toHaveBeenCalledTimes(2);
    });
  });
});
