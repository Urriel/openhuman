import { createGlobalState, useDebounceFn } from '@vueuse/core';
import { ref, computed, watch } from 'vue';
import { toast } from 'vue-sonner';
import type {
  Command,
  CommandPaletteMode,
  ViewContext,
  SearchResultItem,
} from '@/types/command-palette';
import type { Label } from '@/types/commands';
import { invokeSearchMessages, invokeListLabels, invokeSyncEmails } from '@/types/commands';
import { useEmailActions } from './useEmailActions';

/**
 * Global command palette state and logic
 */
export const useCommandPalette = createGlobalState(() => {
  // State
  const isOpen = ref(false);
  const mode = ref<CommandPaletteMode>('commands');
  const query = ref('');
  const navigationStack = ref<string[]>(['main']);
  const currentContext = ref<ViewContext>('global');
  const currentAccountId = ref<number>(1); // TODO: Get from actual account state
  const currentEmailId = ref<number | null>(null);
  const selectedEmailIds = ref<number[]>([]);
  const showAccountManagement = ref(false);

  // Search state
  const searchResults = ref<SearchResultItem[]>([]);
  const isSearching = ref(false);

  // Labels state
  const labels = ref<Label[]>([]);
  const isLoadingLabels = ref(false);

  // Email actions
  const emailActions = useEmailActions();

  /**
   * Current page in navigation stack
   */
  const currentPage = computed(() => navigationStack.value[navigationStack.value.length - 1]);

  /**
   * Define all available commands
   */
  const commands: Command[] = [
    // Email Actions
    {
      id: 'archive',
      label: 'Archive',
      category: 'Email Actions',
      keywords: ['archive', 'done', 'e'],
      shortcut: 'e',
      action: async () => {
        const ids =
          selectedEmailIds.value.length > 0
            ? selectedEmailIds.value
            : currentEmailId.value
              ? [currentEmailId.value]
              : [];
        if (ids.length > 0) {
          await emailActions.archiveEmails(ids);
        }
      },
      context: ['inbox', 'thread'],
    },
    {
      id: 'delete',
      label: 'Delete',
      category: 'Email Actions',
      keywords: ['delete', 'trash', '#'],
      shortcut: '#',
      action: async () => {
        if (currentEmailId.value) {
          await emailActions.deleteEmail(currentEmailId.value);
        }
      },
      context: ['inbox', 'thread'],
    },
    {
      id: 'star',
      label: 'Star',
      category: 'Email Actions',
      keywords: ['star', 'favorite', 's'],
      shortcut: 's',
      action: async () => {
        if (currentEmailId.value) {
          // TODO: Get current star status from email state
          await emailActions.toggleStar(currentEmailId.value, false);
        }
      },
      context: ['inbox', 'thread'],
    },
    {
      id: 'mark-read',
      label: 'Mark as Read',
      category: 'Email Actions',
      keywords: ['read', 'mark'],
      shortcut: 'Shift+I',
      action: async () => {
        const ids =
          selectedEmailIds.value.length > 0
            ? selectedEmailIds.value
            : currentEmailId.value
              ? [currentEmailId.value]
              : [];
        if (ids.length > 0) {
          await emailActions.markRead(ids);
        }
      },
      context: ['inbox', 'thread'],
    },
    {
      id: 'mark-unread',
      label: 'Mark as Unread',
      category: 'Email Actions',
      keywords: ['unread', 'mark'],
      shortcut: 'Shift+U',
      action: async () => {
        const ids =
          selectedEmailIds.value.length > 0
            ? selectedEmailIds.value
            : currentEmailId.value
              ? [currentEmailId.value]
              : [];
        if (ids.length > 0) {
          await emailActions.markUnread(ids);
        }
      },
      context: ['inbox', 'thread'],
    },
    {
      id: 'refresh',
      label: 'Refresh',
      category: 'Email Actions',
      keywords: ['refresh', 'reload', 'sync'],
      shortcut: 'Cmd+R',
      action: async () => {
        closePalette();
        try {
          const result = await invokeSyncEmails();
          if (result.new_messages > 0) {
            toast.success(
              `Synced ${result.new_messages} new message${result.new_messages > 1 ? 's' : ''}`
            );
          } else {
            toast.success('All caught up!');
          }
        } catch (err) {
          console.error('Sync failed:', err);
          toast.error('Failed to sync emails');
        }
      },
    },
    // Composition
    {
      id: 'compose',
      label: 'Compose',
      category: 'Composition',
      keywords: ['compose', 'new', 'email', 'c'],
      shortcut: 'c',
      action: () => {
        toast.info('Compose not yet implemented');
      },
    },
    {
      id: 'reply',
      label: 'Reply',
      category: 'Composition',
      keywords: ['reply', 'respond', 'r'],
      shortcut: 'r',
      action: () => {
        toast.info('Reply not yet implemented');
      },
      context: ['thread'],
    },
    {
      id: 'reply-all',
      label: 'Reply All',
      category: 'Composition',
      keywords: ['reply', 'all', 'a'],
      shortcut: 'a',
      action: () => {
        toast.info('Reply All not yet implemented');
      },
      context: ['thread'],
    },
    {
      id: 'forward',
      label: 'Forward',
      category: 'Composition',
      keywords: ['forward', 'f'],
      shortcut: 'f',
      action: () => {
        toast.info('Forward not yet implemented');
      },
      context: ['thread'],
    },
    // Navigation
    {
      id: 'next-email',
      label: 'Next Email',
      category: 'Navigation',
      keywords: ['next', 'down', 'j'],
      shortcut: 'j',
      action: () => {
        // Handled by EmailList component
      },
      context: ['inbox'],
    },
    {
      id: 'prev-email',
      label: 'Previous Email',
      category: 'Navigation',
      keywords: ['previous', 'prev', 'up', 'k'],
      shortcut: 'k',
      action: () => {
        // Handled by EmailList component
      },
      context: ['inbox'],
    },
    {
      id: 'open-email',
      label: 'Open Email',
      category: 'Navigation',
      keywords: ['open', 'enter', 'o'],
      shortcut: 'Enter',
      action: () => {
        // Handled by EmailList component
      },
      context: ['inbox'],
    },
    {
      id: 'go-inbox',
      label: 'Go to Inbox',
      category: 'Navigation',
      keywords: ['inbox', 'go'],
      shortcut: 'g→i',
      action: () => {
        toast.info('Navigation not yet implemented');
      },
    },
    {
      id: 'go-sent',
      label: 'Go to Sent',
      category: 'Navigation',
      keywords: ['sent', 'go'],
      shortcut: 'g→s',
      action: () => {
        toast.info('Navigation not yet implemented');
      },
    },
    {
      id: 'go-drafts',
      label: 'Go to Drafts',
      category: 'Navigation',
      keywords: ['drafts', 'go'],
      shortcut: 'g→d',
      action: () => {
        toast.info('Navigation not yet implemented');
      },
    },
    {
      id: 'go-all-mail',
      label: 'Go to All Mail',
      category: 'Navigation',
      keywords: ['all', 'mail', 'go'],
      shortcut: 'g→a',
      action: () => {
        toast.info('Navigation not yet implemented');
      },
    },
    {
      id: 'go-starred',
      label: 'Go to Starred',
      category: 'Navigation',
      keywords: ['starred', 'go'],
      shortcut: 'g→*',
      action: () => {
        toast.info('Navigation not yet implemented');
      },
    },
    {
      id: 'go-archive',
      label: 'Go to Archive',
      category: 'Navigation',
      keywords: ['archive', 'go'],
      shortcut: 'g→e',
      action: () => {
        toast.info('Navigation not yet implemented');
      },
    },
    // Selection
    {
      id: 'select-all',
      label: 'Select All',
      category: 'Selection',
      keywords: ['select', 'all'],
      shortcut: 'Cmd+A',
      action: () => {
        toast.info('Select All not yet implemented');
      },
      context: ['inbox'],
    },
    {
      id: 'deselect',
      label: 'Deselect All',
      category: 'Selection',
      keywords: ['deselect', 'clear'],
      shortcut: 'Cmd+D',
      action: () => {
        toast.info('Deselect not yet implemented');
      },
      context: ['inbox'],
    },
    {
      id: 'toggle-checkbox',
      label: 'Toggle Checkbox',
      category: 'Selection',
      keywords: ['toggle', 'select', 'x'],
      shortcut: 'x',
      action: () => {
        // Handled by EmailList component
      },
      context: ['inbox'],
    },
    // Labels
    {
      id: 'apply-label',
      label: 'Apply Label',
      category: 'Labels',
      keywords: ['label', 'tag', 'l'],
      shortcut: 'l',
      action: () => {
        pushPage('labels');
      },
      context: ['inbox', 'thread'],
    },
    {
      id: 'remove-label',
      label: 'Remove Label',
      category: 'Labels',
      keywords: ['remove', 'label', 'untag'],
      shortcut: 'Shift+L',
      action: () => {
        toast.info('Remove Label not yet implemented');
      },
      context: ['inbox', 'thread'],
    },
    // System
    {
      id: 'command-palette',
      label: 'Command Palette',
      category: 'System',
      keywords: ['command', 'palette', 'search'],
      shortcut: 'Cmd+K',
      action: () => {
        openPalette();
      },
    },
    {
      id: 'help',
      label: 'Keyboard Shortcuts',
      category: 'System',
      keywords: ['help', 'shortcuts', 'keys', '?'],
      shortcut: '?',
      action: () => {
        toast.info('Help modal not yet implemented');
      },
    },
    {
      id: 'toggle-theme',
      label: 'Toggle Theme',
      category: 'System',
      keywords: ['theme', 'dark', 'light'],
      shortcut: 'Cmd+Shift+D',
      action: () => {
        toast.info('Theme toggle not yet implemented');
      },
    },
    {
      id: 'manage-accounts',
      label: 'Manage Email Accounts',
      category: 'System',
      keywords: ['accounts', 'settings', 'manage', 'email'],
      shortcut: 'Cmd+,',
      action: () => {
        showAccountManagement.value = true;
        closePalette();
      },
    },
    {
      id: 'snooze',
      label: 'Snooze (Coming Soon)',
      category: 'Email Actions',
      keywords: ['snooze', 'later', 'h'],
      shortcut: 'h',
      action: () => {
        toast.info('Snooze feature coming soon');
      },
      context: ['inbox', 'thread'],
      disabled: true,
    },
  ];

  /**
   * Filtered commands based on query and context
   */
  const filteredCommands = computed(() => {
    return commands.filter(cmd => {
      // Filter by context
      if (cmd.context && !cmd.context.includes(currentContext.value)) {
        return false;
      }
      // Filter by query
      if (query.value === '') {
        return true;
      }
      const q = query.value.toLowerCase();
      return cmd.label.toLowerCase().includes(q) || cmd.keywords?.some(k => k.includes(q)) || false;
    });
  });

  /**
   * Filtered labels based on query
   */
  const filteredLabels = computed(() => {
    if (query.value === '') {
      return labels.value;
    }
    const q = query.value.toLowerCase();
    return labels.value.filter(label => label.name.toLowerCase().includes(q));
  });

  /**
   * Fetch labels from backend
   */
  async function fetchLabels(): Promise<void> {
    if (isLoadingLabels.value) return;

    try {
      isLoadingLabels.value = true;
      const result = await invokeListLabels(currentAccountId.value, false);
      labels.value = result;
    } catch (err) {
      console.error('Failed to fetch labels:', err);
      toast.error('Failed to load labels');
    } finally {
      isLoadingLabels.value = false;
    }
  }

  /**
   * Search for emails
   */
  const searchEmails = useDebounceFn(async (searchQuery: string) => {
    if (!searchQuery || mode.value !== 'search') {
      searchResults.value = [];
      return;
    }

    try {
      isSearching.value = true;
      const results = await invokeSearchMessages(searchQuery, currentAccountId.value);
      searchResults.value = results.map(r => ({
        id: r.id,
        subject: r.subject || '(No subject)',
        sender: r.from_addr || 'Unknown',
        snippet: r.snippet || '',
        timestamp: r.date || '',
        is_read: r.is_read,
        is_starred: r.is_starred,
      }));
    } catch (err) {
      console.error('Search failed:', err);
      toast.error('Search failed');
    } finally {
      isSearching.value = false;
    }
  }, 300);

  /**
   * Watch query changes for search mode
   */
  watch(query, newQuery => {
    if (mode.value === 'search') {
      searchEmails(newQuery);
    }
  });

  /**
   * Watch isOpen to fetch labels when opening
   */
  watch(isOpen, newIsOpen => {
    if (newIsOpen && labels.value.length === 0) {
      fetchLabels();
    }
  });

  /**
   * Open the command palette
   */
  function openPalette(initialMode: CommandPaletteMode = 'commands'): void {
    isOpen.value = true;
    mode.value = initialMode;
    query.value = '';
    navigationStack.value = ['main'];
  }

  /**
   * Close the command palette
   */
  function closePalette(): void {
    isOpen.value = false;
    query.value = '';
    searchResults.value = [];
    navigationStack.value = ['main'];
  }

  /**
   * Execute a command by ID
   */
  async function executeCommand(commandId: string): Promise<void> {
    const command = commands.find(c => c.id === commandId);
    if (!command || command.disabled) {
      return;
    }

    try {
      await command.action();
    } catch (err) {
      console.error('Command execution failed:', err);
      toast.error('Command failed');
    }
  }

  /**
   * Push a page onto the navigation stack
   */
  function pushPage(page: string): void {
    navigationStack.value.push(page);
    query.value = '';
  }

  /**
   * Pop a page from the navigation stack
   */
  function popPage(): void {
    if (navigationStack.value.length > 1) {
      navigationStack.value.pop();
      query.value = '';
    } else {
      closePalette();
    }
  }

  /**
   * Set the current context
   */
  function setContext(context: ViewContext): void {
    currentContext.value = context;
  }

  /**
   * Set the current email ID
   */
  function setCurrentEmail(emailId: number | null): void {
    currentEmailId.value = emailId;
  }

  /**
   * Set selected email IDs
   */
  function setSelectedEmails(emailIds: number[]): void {
    selectedEmailIds.value = emailIds;
  }

  return {
    // State
    isOpen,
    mode,
    query,
    currentPage,
    currentContext,
    currentEmailId,
    selectedEmailIds,
    showAccountManagement,

    // Computed
    filteredCommands,
    filteredLabels,

    // Search
    searchResults,
    isSearching,

    // Labels
    labels,
    isLoadingLabels,

    // Methods
    openPalette,
    closePalette,
    executeCommand,
    pushPage,
    popPage,
    setContext,
    setCurrentEmail,
    setSelectedEmails,
    fetchLabels,
  };
});
