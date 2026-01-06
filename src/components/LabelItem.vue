<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Tag } from 'lucide-vue-next';

interface Props {
  name: string;
  count: number;
  color?: string;
  active?: boolean;
}

withDefaults(defineProps<Props>(), {
  active: false,
  color: '#999999',
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
    <Tag :size="14" :style="{ color: color }" class="shrink-0" />
    <span class="flex-1 truncate text-left">{{ name }}</span>
    <span v-if="count > 0" class="text-xs text-muted-foreground">{{ count }}</span>
  </Button>
</template>
