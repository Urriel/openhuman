<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { Archive, Trash2, Star, Mail } from 'lucide-vue-next';

interface Props {
  messageId: number;
  isStarred: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  refresh: [];
}>();

const isProcessing = ref(false);

async function archiveMessage() {
  try {
    isProcessing.value = true;
    await invoke('archive_messages', { messageIds: [props.messageId] });
    emit('refresh');
  } catch (error) {
    console.error('Failed to archive message:', error);
  } finally {
    isProcessing.value = false;
  }
}

async function deleteMessage() {
  try {
    isProcessing.value = true;
    await invoke('delete_message', { messageId: props.messageId });
    emit('refresh');
  } catch (error) {
    console.error('Failed to delete message:', error);
  } finally {
    isProcessing.value = false;
  }
}

async function toggleStar() {
  try {
    isProcessing.value = true;
    if (props.isStarred) {
      await invoke('unstar_message', { messageId: props.messageId });
    } else {
      await invoke('star_message', { messageId: props.messageId });
    }
    emit('refresh');
  } catch (error) {
    console.error('Failed to toggle star:', error);
  } finally {
    isProcessing.value = false;
  }
}

async function markUnread() {
  try {
    isProcessing.value = true;
    await invoke('mark_unread', { messageIds: [props.messageId] });
    emit('refresh');
  } catch (error) {
    console.error('Failed to mark unread:', error);
  } finally {
    isProcessing.value = false;
  }
}
</script>

<template>
  <div class="flex items-center gap-2 border-b border-border px-4 py-2">
    <Button variant="ghost" size="sm" :disabled="isProcessing" @click="archiveMessage">
      <Archive :size="16" class="mr-1" />
      Archive
    </Button>
    <Button variant="ghost" size="sm" :disabled="isProcessing" @click="deleteMessage">
      <Trash2 :size="16" class="mr-1" />
      Delete
    </Button>
    <Button variant="ghost" size="sm" :disabled="isProcessing" @click="toggleStar">
      <Star :size="16" :fill="isStarred ? 'currentColor' : 'none'" class="mr-1" />
      {{ isStarred ? 'Unstar' : 'Star' }}
    </Button>
    <Button variant="ghost" size="sm" :disabled="isProcessing" @click="markUnread">
      <Mail :size="16" class="mr-1" />
      Mark Unread
    </Button>
  </div>
</template>
