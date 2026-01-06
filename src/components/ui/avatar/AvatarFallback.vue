<script setup lang="ts">
import { cn } from '@/lib/utils';
import type { HTMLAttributes } from 'vue';

interface Props {
  class?: HTMLAttributes['class'];
}

const props = defineProps<Props>();
</script>

<template>
  <div
    :class="
      cn('flex h-full w-full items-center justify-center rounded-full bg-muted', props.class)
    "
  >
    <slot />
  </div>
</template>
