<script setup lang="ts">
import { ref, watch, onMounted, computed, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import VirtualList from './VirtualList.vue';
import EmailListItem from './EmailListItem.vue';
import BulkActionsToolbar from './BulkActionsToolbar.vue';
import { Checkbox } from '@/components/ui/checkbox';
import { useKeyboardShortcuts } from '@/composables/useKeyboardShortcuts';

interface MessageListItem {
  id: number;
  subject?: string;
  from_addr: string;
  preview: string;
  date?: string;
  is_read: boolean;
  is_starred: boolean;
  has_attachments: boolean;
}

interface Props {
  folder?: string;
  labelId?: number;
  accountId?: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  selectMessage: [messageId: number];
}>();

const messages = ref<MessageListItem[]>([]);
const selectedIds = ref<Set<number>>(new Set());
const checkedIds = ref<Set<number>>(new Set());
const isLoading = ref(false);
const selectedIndex = ref(0);

const allChecked = computed(() => {
  return messages.value.length > 0 && checkedIds.value.size === messages.value.length;
});

const someChecked = computed(() => {
  return checkedIds.value.size > 0 && checkedIds.value.size < messages.value.length;
});

const selectedEmailId = computed(() => messages.value[selectedIndex.value]?.id);

async function loadMessages() {
  try {
    isLoading.value = true;
    const result = await invoke<MessageListItem[]>('list_messages', {
      accountId: props.accountId,
      folder: props.folder,
      labelId: props.labelId,
      isRead: null,
      isStarred: null,
      limit: 10000,
    });
    messages.value = result;
  } catch (error) {
    console.error('Failed to load messages:', error);
  } finally {
    isLoading.value = false;
  }
}

function selectMessage(messageId: number) {
  selectedIds.value.clear();
  selectedIds.value.add(messageId);
  emit('selectMessage', messageId);
}

function handleCheck(messageId: number, checked: boolean) {
  const newChecked = new Set(checkedIds.value);
  if (checked) {
    newChecked.add(messageId);
  } else {
    newChecked.delete(messageId);
  }
  checkedIds.value = newChecked;
}

function handleCheckAll(checked: boolean | 'indeterminate') {
  if (checked === 'indeterminate') return;
  if (checked) {
    checkedIds.value = new Set(messages.value.map(msg => msg.id));
  } else {
    checkedIds.value = new Set();
  }
}

async function handleBulkArchive() {
  try {
    const ids = Array.from(checkedIds.value);
    await invoke('bulk_archive_messages', { messageIds: ids });
    checkedIds.value.clear();
    await loadMessages();
  } catch (error) {
    console.error('Failed to archive messages:', error);
  }
}

async function handleBulkDelete() {
  try {
    const ids = Array.from(checkedIds.value);
    await invoke('bulk_delete_messages', { messageIds: ids });
    checkedIds.value.clear();
    await loadMessages();
  } catch (error) {
    console.error('Failed to delete messages:', error);
  }
}

async function handleBulkMarkRead() {
  try {
    const ids = Array.from(checkedIds.value);
    await invoke('bulk_mark_read', { messageIds: ids, isRead: true });
    checkedIds.value.clear();
    await loadMessages();
  } catch (error) {
    console.error('Failed to mark messages as read:', error);
  }
}

async function handleBulkMarkUnread() {
  try {
    const ids = Array.from(checkedIds.value);
    await invoke('bulk_mark_read', { messageIds: ids, isRead: false });
    checkedIds.value.clear();
    await loadMessages();
  } catch (error) {
    console.error('Failed to mark messages as unread:', error);
  }
}

// Watch for filter changes
watch([() => props.folder, () => props.labelId, () => props.accountId], () => {
  loadMessages();
});

// Keyboard navigation
const { register } = useKeyboardShortcuts();

function moveSelectionDown() {
  if (selectedIndex.value < messages.value.length - 1) {
    selectedIndex.value++;
    scrollToSelected();
  }
}

function moveSelectionUp() {
  if (selectedIndex.value > 0) {
    selectedIndex.value--;
    scrollToSelected();
  }
}

function openSelectedEmail() {
  const emailId = selectedEmailId.value;
  if (emailId) {
    selectMessage(emailId);
  }
}

function toggleSelectedCheckbox() {
  const emailId = selectedEmailId.value;
  if (emailId) {
    const isChecked = checkedIds.value.has(emailId);
    handleCheck(emailId, !isChecked);
  }
}

function scrollToSelected() {
  nextTick(() => {
    const element = document.querySelector(`[data-email-index="${selectedIndex.value}"]`);
    element?.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
  });
}

onMounted(() => {
  loadMessages();

  // Register keyboard shortcuts for navigation
  register('j', moveSelectionDown, { view: 'inbox' });
  register('k', moveSelectionUp, { view: 'inbox' });
  register('Enter', openSelectedEmail, { view: 'inbox' });
  register('o', openSelectedEmail, { view: 'inbox' });
  register('x', toggleSelectedCheckbox, { view: 'inbox' });
});

defineExpose({
  loadMessages,
  getSelectedIds: () => Array.from(selectedIds.value),
  getCheckedIds: () => Array.from(checkedIds.value),
});
</script>

<template>
  <div class="flex h-full flex-col" data-testid="email-list">
    <!-- Bulk Actions Toolbar (shows when items are checked) -->
    <BulkActionsToolbar
      :selected-count="checkedIds.size"
      @archive="handleBulkArchive"
      @delete="handleBulkDelete"
      @mark-read="handleBulkMarkRead"
      @mark-unread="handleBulkMarkUnread"
    />

    <!-- Checkbox header (shows when no items checked) -->
    <div
      v-if="messages.length > 0 && checkedIds.size === 0"
      class="flex h-12 items-center gap-3 border-b bg-background px-4"
    >
      <Checkbox
        :model-value="allChecked"
        :indeterminate="someChecked"
        @update:model-value="handleCheckAll"
        aria-label="Select all emails"
      />
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="flex flex-1 items-center justify-center">
      <p class="text-muted-foreground">Loading messages...</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="messages.length === 0" class="flex flex-1 items-center justify-center">
      <p class="text-muted-foreground">No messages found</p>
    </div>

    <!-- Virtual List -->
    <VirtualList v-else :items="messages" :item-height="100" container-height="100%" :overscan="10">
      <template #default="{ item, index }">
        <div :data-email-index="index" :class="{ 'bg-accent': selectedIndex === index }">
          <EmailListItem
            :id="item.id"
            :subject="item.subject"
            :from="item.from_addr"
            :from-email="item.from_addr"
            :preview="item.preview"
            :date="item.date"
            :is-read="item.is_read"
            :is-starred="item.is_starred"
            :has-attachments="item.has_attachments"
            :selected="selectedIds.has(item.id)"
            :checked="checkedIds.has(item.id)"
            :is-verified="item.id % 3 === 0"
            @click="selectMessage"
            @check="handleCheck"
          />
        </div>
      </template>
    </VirtualList>
  </div>
</template>
