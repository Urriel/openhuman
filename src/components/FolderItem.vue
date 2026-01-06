<script setup lang="ts">
import { computed } from 'vue';
import type { Component } from 'vue';
import { Button } from '@/components/ui/button';
import { Inbox, Send, Archive, Trash2, Folder } from 'lucide-vue-next';

interface Props {
  name: string;
  count: number;
  active?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  active: false,
});

const icon = computed<Component>(() => {
  switch (props.name) {
    case 'INBOX':
      return Inbox;
    case 'Sent':
      return Send;
    case 'Archive':
      return Archive;
    case 'Trash':
      return Trash2;
    default:
      return Folder;
  }
});
</script>

<template>
  <Button
    variant="ghost"
    :class="[
      'w-full justify-start gap-2 px-2 font-normal',
      active && 'bg-accent text-accent-foreground',
    ]"
  >
    <component :is="icon" :size="16" class="shrink-0" />
    <span class="flex-1 truncate text-left">{{ name }}</span>
    <span v-if="count > 0" class="text-xs text-muted-foreground">{{ count }}</span>
  </Button>
</template>
