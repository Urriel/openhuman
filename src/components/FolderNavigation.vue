<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import {
  Archive,
  AlertOctagon,
  FileText,
  Inbox,
  MailPlus,
  Send,
  Star,
  Trash2,
} from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { useEmailNavigation } from '@/composables/useEmailNavigation';

export type FolderKey =
  | 'INBOX'
  | 'Sent'
  | 'Drafts'
  | 'Favorites'
  | 'Archive'
  | 'Deleted'
  | 'Spam'
  | 'Junk';

interface FolderItem {
  key: FolderKey;
  label: string;
  icon: typeof Inbox;
  count?: number;
}

const props = defineProps<{
  inboxCount: number;
}>();

const emit = defineEmits<{
  compose: [];
}>();

const route = useRoute();
const { navigateToFolder } = useEmailNavigation();

// Determine active folder from route
const activeFolder = computed<FolderKey>(() => {
  const folder = route.params.folder as string;
  if (!folder) return 'INBOX';
  
  const folderMap: Record<string, FolderKey> = {
    inbox: 'INBOX',
    sent: 'Sent',
    drafts: 'Drafts',
    favorites: 'Favorites',
    archive: 'Archive',
    deleted: 'Deleted',
    spam: 'Spam',
    junk: 'Junk',
  };
  
  return folderMap[folder.toLowerCase()] || 'INBOX';
});

const folders: FolderItem[] = [
  { key: 'INBOX', label: 'Inbox', icon: Inbox, count: props.inboxCount },
  { key: 'Sent', label: 'Sent items', icon: Send },
  { key: 'Drafts', label: 'Drafts', icon: FileText },
  { key: 'Favorites', label: 'Favorites', icon: Star },
  { key: 'Archive', label: 'Archive', icon: Archive },
  { key: 'Deleted', label: 'Deleted', icon: Trash2 },
  { key: 'Spam', label: 'Spam', icon: AlertOctagon },
  { key: 'Junk', label: 'Junk', icon: AlertOctagon },
];

function tabClass(isActive: boolean) {
  return cn(
    'h-8 justify-start gap-2 rounded-md px-3 text-xs font-normal tracking-tight',
    isActive
      ? 'bg-muted text-foreground'
      : 'text-muted-foreground hover:bg-muted/70 hover:text-foreground'
  );
}

function handleFolderClick(folder: FolderKey) {
  navigateToFolder(folder);
}
</script>

<template>
  <div
    class="hidden h-[54px] items-center gap-2 border-b bg-background px-3 md:flex"
    data-testid="folder-nav"
  >
    <Button
      class="h-9 bg-linear-to-br from-white via-purple-200 to-blue-200 text-foreground hover:opacity-90"
      @click="emit('compose')"
    >
      <MailPlus class="mr-2 size-4" />
      Compose
    </Button>

    <Separator orientation="vertical" class="mx-1 h-7" />

    <nav class="flex items-center gap-1 overflow-x-auto" aria-label="Folders">
      <Button
        v-for="folder in folders"
        :key="folder.key"
        variant="ghost"
        :class="tabClass(activeFolder === folder.key)"
        @click="handleFolderClick(folder.key)"
      >
        <component :is="folder.icon" class="size-4" />
        <span>{{ folder.label }}</span>
        <span v-if="folder.key === 'INBOX'" class="ml-1 text-muted-foreground">{{
          props.inboxCount
        }}</span>
      </Button>
    </nav>
  </div>
</template>
