<script setup lang="ts">
import { Archive, Trash2, MailOpen, MailCheck } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';

interface Props {
  selectedCount: number;
  hasUnread?: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
  archive: [];
  delete: [];
  markRead: [];
  markUnread: [];
}>();
</script>

<template>
  <div
    v-if="selectedCount > 0"
    class="flex h-12 items-center gap-2 border-b bg-accent/30 px-4 transition-all"
    data-testid="bulk-actions-toolbar"
  >
    <span class="text-sm font-medium">{{ selectedCount }} selected</span>
    <div class="ml-4 flex gap-1">
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              variant="ghost"
              size="icon"
              @click="emit('archive')"
              aria-label="Archive selected emails"
              data-testid="bulk-archive-btn"
            >
              <Archive class="h-4 w-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Archive</p>
          </TooltipContent>
        </Tooltip>

        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              variant="ghost"
              size="icon"
              @click="emit('delete')"
              aria-label="Delete selected emails"
              data-testid="bulk-delete-btn"
            >
              <Trash2 class="h-4 w-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Delete</p>
          </TooltipContent>
        </Tooltip>

        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              variant="ghost"
              size="icon"
              @click="emit('markRead')"
              aria-label="Mark selected emails as read"
              data-testid="bulk-mark-read-btn"
            >
              <MailOpen class="h-4 w-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Mark as read</p>
          </TooltipContent>
        </Tooltip>

        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              variant="ghost"
              size="icon"
              @click="emit('markUnread')"
              aria-label="Mark selected emails as unread"
              data-testid="bulk-mark-unread-btn"
            >
              <MailCheck class="h-4 w-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Mark as unread</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  </div>
</template>
