import { useRouter, useRoute } from 'vue-router';
import type { FolderKey } from '@/components/FolderNavigation.vue';

/**
 * Composable for centralized email navigation logic using vue-router
 */
export function useEmailNavigation() {
  const router = useRouter();
  const route = useRoute();

  /**
   * Navigate to a specific folder
   */
  function navigateToFolder(folder: FolderKey) {
    const path = `/${folder.toLowerCase()}`;
    void router.push(path);
  }

  /**
   * Navigate to inbox
   */
  function navigateToInbox() {
    void router.push('/inbox');
  }

  /**
   * Select an email by updating the query param
   * Preserves other query params (like label filters)
   */
  function selectEmail(messageId: number) {
    void router.push({
      query: {
        ...route.query,
        message: messageId.toString(),
      },
    });
  }

  /**
   * Clear email selection (remove message query param)
   */
  function clearEmailSelection() {
    const { message: _message, ...otherQuery } = route.query;
    void router.push({ query: otherQuery });
  }

  /**
   * Navigate to settings/accounts
   */
  function navigateToSettings() {
    void router.push('/settings/accounts');
  }

  /**
   * Navigate to a specific folder with a label filter
   */
  function navigateToFolderWithLabel(folder: FolderKey, labelId: number) {
    void router.push({
      path: `/${folder.toLowerCase()}`,
      query: { label: labelId.toString() },
    });
  }

  return {
    navigateToFolder,
    navigateToInbox,
    selectEmail,
    clearEmailSelection,
    navigateToSettings,
    navigateToFolderWithLabel,
  };
}
