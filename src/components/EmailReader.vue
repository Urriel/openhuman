<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Reply, ReplyAll, Forward, Archive, Trash2, Star, MoreHorizontal } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Separator } from '@/components/ui/separator';

interface Message {
  id: number;
  subject?: string;
  from_addr: string;
  to_addr?: string;
  cc_addr?: string;
  body_html?: string;
  body_plain?: string;
  date?: string;
  is_starred: boolean;
}

interface Props {
  messageId?: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  refresh: [];
}>();

const message = ref<Message | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function loadMessage() {
  if (!props.messageId) {
    message.value = null;
    return;
  }

  try {
    isLoading.value = true;
    error.value = null;
    const result = await invoke<Message>('get_message', {
      messageId: props.messageId,
    });
    message.value = result;
  } catch (err) {
    console.error('Failed to load message:', err);
    error.value = err instanceof Error ? err.message : 'Failed to load message';
  } finally {
    isLoading.value = false;
  }
}

async function toggleStar() {
  if (!message.value) return;

  try {
    if (message.value.is_starred) {
      await invoke('unstar_message', { messageId: message.value.id });
    } else {
      await invoke('star_message', { messageId: message.value.id });
    }
    message.value.is_starred = !message.value.is_starred;
    emit('refresh');
  } catch (err) {
    console.error('Failed to toggle star:', err);
  }
}

async function archiveMessage() {
  if (!message.value) return;

  try {
    await invoke('archive_messages', { messageIds: [message.value.id] });
    emit('refresh');
  } catch (err) {
    console.error('Failed to archive message:', err);
  }
}

async function deleteMessage() {
  if (!message.value) return;

  try {
    await invoke('delete_message', { messageId: message.value.id });
    emit('refresh');
  } catch (err) {
    console.error('Failed to delete message:', err);
  }
}

const senderInitials = computed(() => {
  if (!message.value?.from_addr) return '?';
  const name = message.value.from_addr.split('@')[0];
  return name
    .split('.')
    .map(n => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);
});

const formattedDate = computed(() => {
  if (!message.value?.date) return '';
  return new Date(message.value.date).toLocaleString('en-US', {
    weekday: 'short',
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  });
});

// Email content rendering with fallback
const emailContent = computed(() => {
  if (!message.value) return { html: null, plain: null, error: false };

  try {
    // Prefer HTML content if available
    if (message.value.body_html) {
      // Sanitize HTML by creating a DOM element and checking if it's valid
      const div = document.createElement('div');
      div.innerHTML = message.value.body_html;
      return {
        html: message.value.body_html,
        plain: message.value.body_plain,
        error: false,
      };
    }
    return {
      html: null,
      plain: message.value.body_plain,
      error: false,
    };
  } catch (err) {
    console.error('Error rendering HTML content:', err);
    // Fallback to plain text if HTML rendering fails
    return {
      html: null,
      plain: message.value.body_plain,
      error: true,
    };
  }
});

watch(() => props.messageId, loadMessage, { immediate: true });
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden">
    <!-- Empty State -->
    <div
      v-if="!messageId && !isLoading"
      class="flex flex-1 items-center justify-center text-muted-foreground"
    >
      <p>Select an email to read</p>
    </div>

    <!-- Loading State -->
    <div v-else-if="isLoading" class="flex flex-1 items-center justify-center">
      <p class="text-muted-foreground">Loading message...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="flex flex-1 items-center justify-center">
      <p class="text-destructive">{{ error }}</p>
    </div>

    <!-- Message Content -->
    <div v-else-if="message" class="flex flex-1 flex-col overflow-hidden">
      <!-- Toolbar -->
      <div class="flex items-center gap-2 border-b px-4 py-2">
        <Button variant="ghost" size="icon" title="Reply">
          <Reply :size="16" />
        </Button>
        <Button variant="ghost" size="icon" title="Reply All">
          <ReplyAll :size="16" />
        </Button>
        <Button variant="ghost" size="icon" title="Forward">
          <Forward :size="16" />
        </Button>

        <Separator orientation="vertical" class="mx-2 h-6" />

        <Button variant="ghost" size="icon" title="Archive" @click="archiveMessage">
          <Archive :size="16" />
        </Button>
        <Button variant="ghost" size="icon" title="Delete" @click="deleteMessage">
          <Trash2 :size="16" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          :title="message.is_starred ? 'Unstar' : 'Star'"
          @click="toggleStar"
        >
          <Star
            :size="16"
            :fill="message.is_starred ? 'currentColor' : 'none'"
            :class="message.is_starred ? 'text-yellow-500' : ''"
          />
        </Button>

        <div class="ml-auto">
          <Button variant="ghost" size="icon">
            <MoreHorizontal :size="16" />
          </Button>
        </div>
      </div>

      <!-- Message Header -->
      <div class="border-b p-6">
        <h1 class="mb-4 text-2xl font-bold">{{ message.subject || '(no subject)' }}</h1>

        <div class="flex items-start gap-4">
          <Avatar class="h-10 w-10">
            <AvatarImage :src="``" :alt="message.from_addr" />
            <AvatarFallback>{{ senderInitials }}</AvatarFallback>
          </Avatar>

          <div class="flex-1">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-semibold">{{ message.from_addr.split('@')[0] }}</p>
                <p class="text-sm text-muted-foreground">{{ message.from_addr }}</p>
              </div>
              <p class="text-sm text-muted-foreground">{{ formattedDate }}</p>
            </div>

            <div v-if="message.to_addr" class="mt-2 text-sm text-muted-foreground">
              <span>To: {{ message.to_addr }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Message Body -->
      <div class="flex-1 overflow-y-auto p-6">
        <!-- Show warning if HTML failed to render and fell back to plain text -->
        <div
          v-if="emailContent.error"
          class="mb-4 rounded-md bg-yellow-50 dark:bg-yellow-900/20 px-4 py-2 text-sm text-yellow-800 dark:text-yellow-200"
        >
          ⚠ HTML rendering failed - showing plain text version
        </div>

        <!-- Render HTML content if available and no error -->
        <div
          v-if="emailContent.html && !emailContent.error"
          class="prose prose-sm max-w-none dark:prose-invert"
          v-html="emailContent.html"
        />

        <!-- Render plain text content as fallback -->
        <div v-else-if="emailContent.plain" class="whitespace-pre-wrap text-sm font-mono">
          {{ emailContent.plain }}
        </div>

        <!-- No content available -->
        <div v-else class="text-sm text-muted-foreground italic">No message content</div>
      </div>
    </div>
  </div>
</template>
