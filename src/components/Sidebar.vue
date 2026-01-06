<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
  Inbox,
  Send,
  FileText,
  Star,
  Archive,
  Trash2,
  Folder as FolderIcon,
  Tag,
  ChevronDown,
} from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';

interface Folder {
  id: number;
  account_id: number;
  name: string;
  message_count: number;
}

interface Label {
  id: number;
  account_id: number;
  name: string;
  color?: string;
  message_count?: number;
}

const emit = defineEmits<{
  selectFolder: [folder: string];
  selectLabel: [labelId: number];
}>();

const folders = ref<Folder[]>([
  { id: 1, account_id: 1, name: 'INBOX', message_count: 20 },
  { id: 2, account_id: 1, name: 'Sent', message_count: 0 },
  { id: 3, account_id: 1, name: 'Drafts', message_count: 0 },
  { id: 4, account_id: 1, name: 'Favorites', message_count: 0 },
  { id: 5, account_id: 1, name: 'Archive', message_count: 0 },
  { id: 6, account_id: 1, name: 'Deleted', message_count: 0 },
]);

const labels = ref<Label[]>([]);
const activeFolder = ref('INBOX');
const activeLabel = ref<number | null>(null);
const isLoadingLabels = ref(false);
const labelError = ref<string | null>(null);
const showMoreFolders = ref(false);

function getFolderIcon(folderName: string) {
  switch (folderName) {
    case 'INBOX':
      return Inbox;
    case 'Sent':
      return Send;
    case 'Drafts':
      return FileText;
    case 'Favorites':
      return Star;
    case 'Archive':
      return Archive;
    case 'Deleted':
    case 'Trash':
      return Trash2;
    default:
      return FolderIcon;
  }
}

const visibleFolders = ref<Folder[]>([]);

function updateVisibleFolders() {
  if (showMoreFolders.value) {
    visibleFolders.value = folders.value;
  } else {
    visibleFolders.value = folders.value.slice(0, 3);
  }
}

async function loadLabels() {
  try {
    isLoadingLabels.value = true;
    labelError.value = null;
    const accountId = 1;
    const result = await invoke<Label[]>('list_labels', {
      accountId,
      includeCounts: true,
    });
    labels.value = result;
  } catch (error) {
    console.error('Failed to load labels:', error);
    labelError.value = 'Failed to load labels';
  } finally {
    isLoadingLabels.value = false;
  }
}

function selectFolder(folderName: string) {
  activeFolder.value = folderName;
  activeLabel.value = null;
  emit('selectFolder', folderName);
}

function selectLabel(labelId: number) {
  activeLabel.value = labelId;
  activeFolder.value = '';
  emit('selectLabel', labelId);
}

function toggleMoreFolders() {
  showMoreFolders.value = !showMoreFolders.value;
  updateVisibleFolders();
}

onMounted(() => {
  updateVisibleFolders();
  loadLabels();
});
</script>

<template>
  <div class="flex flex-col gap-4 py-2">
    <!-- Main Folders -->
    <div class="flex flex-col gap-1 px-2">
      <Button
        v-for="folder in visibleFolders"
        :key="folder.id"
        variant="ghost"
        :class="[
          'w-full justify-start gap-2 font-normal',
          activeFolder === folder.name && 'bg-muted',
        ]"
        @click="selectFolder(folder.name)"
      >
        <component :is="getFolderIcon(folder.name)" :size="16" class="shrink-0" />
        <span class="flex-1 text-left">{{ folder.name }}</span>
        <Badge v-if="folder.message_count > 0" variant="secondary" class="ml-auto">
          {{ folder.message_count }}
        </Badge>
      </Button>

      <Button
        v-if="folders.length > 3"
        variant="ghost"
        class="w-full justify-start gap-2 font-normal text-muted-foreground"
        @click="toggleMoreFolders"
      >
        <ChevronDown
          :size="16"
          :class="['shrink-0 transition-transform', showMoreFolders && 'rotate-180']"
        />
        <span>{{ showMoreFolders ? 'Less' : 'More' }}</span>
      </Button>
    </div>

    <Separator />

    <!-- Folders Section Header -->
    <div class="px-4">
      <h3 class="text-xs font-semibold uppercase text-muted-foreground">Folders</h3>
    </div>

    <!-- All Folders Collapsible -->
    <div class="flex flex-col gap-1 px-2">
      <Button
        variant="ghost"
        class="w-full justify-start gap-2 font-normal"
        @click="selectFolder('INBOX')"
      >
        <Inbox :size="16" class="shrink-0" />
        <span class="flex-1 text-left">Inbox</span>
      </Button>
    </div>

    <Separator />

    <!-- Labels Section -->
    <div class="px-4">
      <h3 class="text-xs font-semibold uppercase text-muted-foreground">Labels</h3>
    </div>

    <div class="flex flex-col gap-1 px-2">
      <!-- Loading State -->
      <div v-if="isLoadingLabels" class="px-2 py-4 text-sm text-muted-foreground">
        Loading labels...
      </div>

      <!-- Error State -->
      <div v-else-if="labelError" class="px-2 py-4 text-sm text-destructive">
        {{ labelError }}
      </div>

      <!-- Empty State -->
      <div v-else-if="labels.length === 0" class="px-2 py-4 text-sm text-muted-foreground">
        No labels yet
      </div>

      <!-- Labels List -->
      <Button
        v-for="label in labels"
        v-else
        :key="label.id"
        variant="ghost"
        :class="['w-full justify-start gap-2 font-normal', activeLabel === label.id && 'bg-muted']"
        @click="selectLabel(label.id)"
      >
        <Tag :size="16" class="shrink-0" :style="{ color: label.color }" />
        <span class="flex-1 text-left">{{ label.name }}</span>
        <Badge
          v-if="label.message_count && label.message_count > 0"
          variant="secondary"
          class="ml-auto"
        >
          {{ label.message_count }}
        </Badge>
      </Button>
    </div>
  </div>
</template>
