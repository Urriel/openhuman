import { toast } from 'vue-sonner';
import {
  invokeArchiveMessages,
  invokeDeleteMessage,
  invokeMarkRead,
  invokeMarkUnread,
  invokeApplyLabel,
  invokeRemoveLabel,
} from '@/types/commands';
import { useUndoStack } from './useUndoStack';

export interface EmailActionCallbacks {
  onSuccess?: () => void | Promise<void>;
  onError?: (error: Error) => void;
}

/**
 * Email actions composable with toast notifications and undo support
 */
export function useEmailActions() {
  const { pushUndo, executeUndo } = useUndoStack();

  /**
   * Archive one or more emails
   */
  async function archiveEmails(
    messageIds: number[],
    callbacks?: EmailActionCallbacks
  ): Promise<void> {
    try {
      // Execute archive
      await invokeArchiveMessages(messageIds);

      // Show success toast with undo button
      const count = messageIds.length;
      toast.success(`Archived ${count} email${count > 1 ? 's' : ''}`, {
        action: {
          label: 'Undo',
          onClick: async () => {
            await executeUndo();
          },
        },
      });

      // Push undo action
      pushUndo('Archive', async () => {
        // Undo by moving back to inbox (unarchive)
        // Note: We need an unarchive command in the backend for this to work properly
        // For now, we'll just show a message
        toast.info('Undo archive not yet implemented in backend');
      });

      // Call success callback if provided
      await callbacks?.onSuccess?.();
    } catch (err) {
      console.error('Failed to archive emails:', err);
      const error = err instanceof Error ? err : new Error('Unknown error');
      toast.error('Failed to archive emails');
      callbacks?.onError?.(error);
      throw error;
    }
  }

  /**
   * Delete an email
   */
  async function deleteEmail(messageId: number, callbacks?: EmailActionCallbacks): Promise<void> {
    try {
      await invokeDeleteMessage(messageId);

      // Show success toast with undo button
      toast.success('Email deleted', {
        action: {
          label: 'Undo',
          onClick: async () => {
            await executeUndo();
          },
        },
      });

      // Push undo action
      pushUndo('Delete', async () => {
        // Undo delete - would need to restore from trash
        toast.info('Undo delete not yet implemented in backend');
      });

      await callbacks?.onSuccess?.();
    } catch (err) {
      console.error('Failed to delete email:', err);
      const error = err instanceof Error ? err : new Error('Unknown error');
      toast.error('Failed to delete email');
      callbacks?.onError?.(error);
      throw error;
    }
  }

  /**
   * Star/unstar an email
   */
  async function toggleStar(
    messageId: number,
    isStarred: boolean,
    callbacks?: EmailActionCallbacks
  ): Promise<void> {
    try {
      const { invoke } = await import('@tauri-apps/api/core');

      if (isStarred) {
        await invoke('unstar_message', { messageId });
        toast.success('Removed star');
      } else {
        await invoke('star_message', { messageId });
        toast.success('Starred email');
      }

      // Push undo action
      pushUndo(isStarred ? 'Unstar' : 'Star', async () => {
        // Toggle back
        if (isStarred) {
          await invoke('star_message', { messageId });
        } else {
          await invoke('unstar_message', { messageId });
        }
        toast.success('Undone');
      });

      await callbacks?.onSuccess?.();
    } catch (err) {
      console.error('Failed to toggle star:', err);
      const error = err instanceof Error ? err : new Error('Unknown error');
      toast.error('Failed to update star');
      callbacks?.onError?.(error);
      throw error;
    }
  }

  /**
   * Mark emails as read
   */
  async function markRead(messageIds: number[], callbacks?: EmailActionCallbacks): Promise<void> {
    try {
      await invokeMarkRead(messageIds);

      const count = messageIds.length;
      toast.success(`Marked ${count} email${count > 1 ? 's' : ''} as read`, {
        action: {
          label: 'Undo',
          onClick: async () => {
            await executeUndo();
          },
        },
      });

      // Push undo action
      pushUndo('Mark read', async () => {
        await invokeMarkUnread(messageIds);
        toast.success('Undone');
      });

      await callbacks?.onSuccess?.();
    } catch (err) {
      console.error('Failed to mark as read:', err);
      const error = err instanceof Error ? err : new Error('Unknown error');
      toast.error('Failed to mark as read');
      callbacks?.onError?.(error);
      throw error;
    }
  }

  /**
   * Mark emails as unread
   */
  async function markUnread(messageIds: number[], callbacks?: EmailActionCallbacks): Promise<void> {
    try {
      await invokeMarkUnread(messageIds);

      const count = messageIds.length;
      toast.success(`Marked ${count} email${count > 1 ? 's' : ''} as unread`, {
        action: {
          label: 'Undo',
          onClick: async () => {
            await executeUndo();
          },
        },
      });

      // Push undo action
      pushUndo('Mark unread', async () => {
        await invokeMarkRead(messageIds);
        toast.success('Undone');
      });

      await callbacks?.onSuccess?.();
    } catch (err) {
      console.error('Failed to mark as unread:', err);
      const error = err instanceof Error ? err : new Error('Unknown error');
      toast.error('Failed to mark as unread');
      callbacks?.onError?.(error);
      throw error;
    }
  }

  /**
   * Apply a label to an email
   */
  async function applyLabel(
    messageId: number,
    labelId: number,
    labelName: string,
    callbacks?: EmailActionCallbacks
  ): Promise<void> {
    try {
      await invokeApplyLabel(messageId, labelId);

      toast.success(`Applied label: ${labelName}`, {
        action: {
          label: 'Undo',
          onClick: async () => {
            await executeUndo();
          },
        },
      });

      // Push undo action
      pushUndo('Apply label', async () => {
        await invokeRemoveLabel(messageId, labelId);
        toast.success('Undone');
      });

      await callbacks?.onSuccess?.();
    } catch (err) {
      console.error('Failed to apply label:', err);
      const error = err instanceof Error ? err : new Error('Unknown error');
      toast.error('Failed to apply label');
      callbacks?.onError?.(error);
      throw error;
    }
  }

  /**
   * Remove a label from an email
   */
  async function removeLabel(
    messageId: number,
    labelId: number,
    labelName: string,
    callbacks?: EmailActionCallbacks
  ): Promise<void> {
    try {
      await invokeRemoveLabel(messageId, labelId);

      toast.success(`Removed label: ${labelName}`, {
        action: {
          label: 'Undo',
          onClick: async () => {
            await executeUndo();
          },
        },
      });

      // Push undo action
      pushUndo('Remove label', async () => {
        await invokeApplyLabel(messageId, labelId);
        toast.success('Undone');
      });

      await callbacks?.onSuccess?.();
    } catch (err) {
      console.error('Failed to remove label:', err);
      const error = err instanceof Error ? err : new Error('Unknown error');
      toast.error('Failed to remove label');
      callbacks?.onError?.(error);
      throw error;
    }
  }

  return {
    archiveEmails,
    deleteEmail,
    toggleStar,
    markRead,
    markUnread,
    applyLabel,
    removeLabel,
  };
}
