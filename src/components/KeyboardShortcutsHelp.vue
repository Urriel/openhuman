<script setup lang="ts">
import { ref, computed } from 'vue';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';

const isOpen = ref(false);
const searchQuery = ref('');

interface Shortcut {
  category: string;
  key: string;
  description: string;
  context?: string;
}

const shortcuts: Shortcut[] = [
  // Email Actions
  { category: 'Email Actions', key: 'e', description: 'Archive email', context: 'inbox/thread' },
  { category: 'Email Actions', key: '#', description: 'Delete email', context: 'inbox/thread' },
  {
    category: 'Email Actions',
    key: 's',
    description: 'Star/unstar email',
    context: 'inbox/thread',
  },
  {
    category: 'Email Actions',
    key: 'Shift+I',
    description: 'Mark as read',
    context: 'inbox/thread',
  },
  {
    category: 'Email Actions',
    key: 'Shift+U',
    description: 'Mark as unread',
    context: 'inbox/thread',
  },
  { category: 'Email Actions', key: 'Cmd+R', description: 'Refresh' },

  // Composition
  { category: 'Composition', key: 'c', description: 'Compose new email' },
  { category: 'Composition', key: 'r', description: 'Reply', context: 'thread' },
  { category: 'Composition', key: 'a', description: 'Reply all', context: 'thread' },
  { category: 'Composition', key: 'f', description: 'Forward', context: 'thread' },
  { category: 'Composition', key: 'Cmd+Enter', description: 'Send email', context: 'compose' },

  // Navigation
  { category: 'Navigation', key: 'j', description: 'Next email', context: 'inbox' },
  { category: 'Navigation', key: 'k', description: 'Previous email', context: 'inbox' },
  { category: 'Navigation', key: 'Enter / o', description: 'Open email', context: 'inbox' },
  { category: 'Navigation', key: 'g→i', description: 'Go to Inbox' },
  { category: 'Navigation', key: 'g→s', description: 'Go to Sent' },
  { category: 'Navigation', key: 'g→d', description: 'Go to Drafts' },
  { category: 'Navigation', key: 'g→a', description: 'Go to All Mail' },
  { category: 'Navigation', key: 'g→*', description: 'Go to Starred' },
  { category: 'Navigation', key: 'g→e', description: 'Go to Archive' },

  // Selection
  { category: 'Selection', key: 'Cmd+A', description: 'Select all', context: 'inbox' },
  { category: 'Selection', key: 'Cmd+D', description: 'Deselect all', context: 'inbox' },
  { category: 'Selection', key: 'x', description: 'Toggle checkbox', context: 'inbox' },

  // Labels
  { category: 'Labels', key: 'l', description: 'Apply label', context: 'inbox/thread' },
  { category: 'Labels', key: 'Shift+L', description: 'Remove label', context: 'inbox/thread' },

  // System
  { category: 'System', key: 'Cmd+K', description: 'Open command palette' },
  { category: 'System', key: '/', description: 'Search emails' },
  { category: 'System', key: '?', description: 'Show keyboard shortcuts' },
  { category: 'System', key: 'Cmd+Shift+D', description: 'Toggle theme' },
  { category: 'System', key: 'z', description: 'Undo last action' },
  { category: 'System', key: 'Escape', description: 'Close dialog/palette' },
];

const filteredShortcuts = computed(() => {
  if (!searchQuery.value) return shortcuts;

  const q = searchQuery.value.toLowerCase();
  return shortcuts.filter(
    s =>
      s.key.toLowerCase().includes(q) ||
      s.description.toLowerCase().includes(q) ||
      s.category.toLowerCase().includes(q)
  );
});

const groupedShortcuts = computed(() => {
  const groups = new Map<string, Shortcut[]>();
  filteredShortcuts.value.forEach(s => {
    if (!groups.has(s.category)) {
      groups.set(s.category, []);
    }
    groups.get(s.category)!.push(s);
  });
  return groups;
});

function open() {
  isOpen.value = true;
}

defineExpose({ open });
</script>

<template>
  <Dialog v-model:open="isOpen">
    <DialogContent class="max-w-2xl max-h-[80vh] overflow-y-auto">
      <DialogHeader>
        <DialogTitle>Keyboard Shortcuts</DialogTitle>
      </DialogHeader>

      <Input v-model="searchQuery" placeholder="Search shortcuts..." class="mb-4" />

      <div v-if="filteredShortcuts.length === 0" class="text-center py-8 text-muted-foreground">
        No shortcuts found
      </div>

      <div v-for="[category, shortcuts] in groupedShortcuts" :key="category" class="mb-6">
        <h3 class="text-sm font-semibold mb-2 text-foreground">{{ category }}</h3>
        <div class="space-y-2">
          <div
            v-for="shortcut in shortcuts"
            :key="shortcut.key"
            class="flex items-center justify-between py-2 px-3 rounded hover:bg-accent"
          >
            <div class="flex flex-col">
              <span class="text-sm">{{ shortcut.description }}</span>
              <span v-if="shortcut.context" class="text-xs text-muted-foreground">
                {{ shortcut.context }}
              </span>
            </div>
            <kbd class="px-2 py-1 text-xs bg-muted rounded border border-border font-mono">
              {{ shortcut.key }}
            </kbd>
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
