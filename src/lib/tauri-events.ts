/**
 * Tauri Event Listeners
 *
 * This module provides type-safe utilities for listening to Tauri events
 * emitted by the backend (Rust) for real-time updates.
 *
 * ## Usage
 *
 * ```typescript
 * import { onSyncStarted, onSyncCompleted } from '@/lib/tauri-events';
 *
 * // Listen for sync events
 * const unsubscribe = await onSyncStarted((event) => {
 *   console.log(`Sync started for account ${event.payload.account_id}`);
 * });
 *
 * // Clean up listener when component unmounts
 * unsubscribe();
 * ```
 */

import type {
  SyncStartedEvent,
  SyncProgressEvent,
  SyncCompletedEvent,
  SyncFailedEvent,
  SendStatusEvent,
  AuthRequiredEvent,
} from '@/types/commands';

type UnlistenFn = () => void;

/**
 * Listen for sync_started events
 *
 * Emitted when email sync begins for an account
 *
 * @param handler - Callback function to handle the event
 * @returns Promise resolving to unlisten function
 */
export async function onSyncStarted(
  handler: (event: { payload: SyncStartedEvent }) => void
): Promise<UnlistenFn> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<SyncStartedEvent>('sync_started', handler);
}

/**
 * Listen for sync_progress events
 *
 * Emitted periodically during sync to report progress
 *
 * @param handler - Callback function to handle the event
 * @returns Promise resolving to unlisten function
 */
export async function onSyncProgress(
  handler: (event: { payload: SyncProgressEvent }) => void
): Promise<UnlistenFn> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<SyncProgressEvent>('sync_progress', handler);
}

/**
 * Listen for sync_completed events
 *
 * Emitted when email sync completes successfully for an account
 *
 * @param handler - Callback function to handle the event
 * @returns Promise resolving to unlisten function
 */
export async function onSyncCompleted(
  handler: (event: { payload: SyncCompletedEvent }) => void
): Promise<UnlistenFn> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<SyncCompletedEvent>('sync_completed', handler);
}

/**
 * Listen for sync_failed events
 *
 * Emitted when email sync fails for an account after max retries
 *
 * @param handler - Callback function to handle the event
 * @returns Promise resolving to unlisten function
 */
export async function onSyncFailed(
  handler: (event: { payload: SyncFailedEvent }) => void
): Promise<UnlistenFn> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<SyncFailedEvent>('sync_failed', handler);
}

/**
 * Listen for send_status events
 *
 * Emitted when email send status changes (sending, sent, failed)
 *
 * @param handler - Callback function to handle the event
 * @returns Promise resolving to unlisten function
 */
export async function onSendStatus(
  handler: (event: { payload: SendStatusEvent }) => void
): Promise<UnlistenFn> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<SendStatusEvent>('send_status', handler);
}

/**
 * Listen for auth_required events
 *
 * Emitted when authentication fails and user needs to re-authenticate
 *
 * @param handler - Callback function to handle the event
 * @returns Promise resolving to unlisten function
 */
export async function onAuthRequired(
  handler: (event: { payload: AuthRequiredEvent }) => void
): Promise<UnlistenFn> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<AuthRequiredEvent>('auth_required', handler);
}

/**
 * Setup all event listeners at once
 *
 * Convenience function to set up all Tauri event listeners.
 * Returns a cleanup function that unsubscribes from all events.
 *
 * @param handlers - Object with handler functions for each event
 * @returns Promise resolving to cleanup function
 *
 * @example
 * ```typescript
 * const cleanup = await setupTauriEventListeners({
 *   onSyncStarted: (event) => console.log('Sync started', event.payload),
 *   onSyncCompleted: (event) => console.log('Sync completed', event.payload),
 *   onSyncFailed: (event) => console.error('Sync failed', event.payload),
 * });
 *
 * // Later, clean up all listeners
 * cleanup();
 * ```
 */
export async function setupTauriEventListeners(handlers: {
  onSyncStarted?: (event: { payload: SyncStartedEvent }) => void;
  onSyncProgress?: (event: { payload: SyncProgressEvent }) => void;
  onSyncCompleted?: (event: { payload: SyncCompletedEvent }) => void;
  onSyncFailed?: (event: { payload: SyncFailedEvent }) => void;
  onSendStatus?: (event: { payload: SendStatusEvent }) => void;
  onAuthRequired?: (event: { payload: AuthRequiredEvent }) => void;
}): Promise<UnlistenFn> {
  const unlisteners: UnlistenFn[] = [];

  if (handlers.onSyncStarted) {
    unlisteners.push(await onSyncStarted(handlers.onSyncStarted));
  }

  if (handlers.onSyncProgress) {
    unlisteners.push(await onSyncProgress(handlers.onSyncProgress));
  }

  if (handlers.onSyncCompleted) {
    unlisteners.push(await onSyncCompleted(handlers.onSyncCompleted));
  }

  if (handlers.onSyncFailed) {
    unlisteners.push(await onSyncFailed(handlers.onSyncFailed));
  }

  if (handlers.onSendStatus) {
    unlisteners.push(await onSendStatus(handlers.onSendStatus));
  }

  if (handlers.onAuthRequired) {
    unlisteners.push(await onAuthRequired(handlers.onAuthRequired));
  }

  // Return cleanup function that calls all unlisteners
  return () => {
    unlisteners.forEach(unlisten => unlisten());
  };
}
