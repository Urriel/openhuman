<script setup lang="ts">
import { cn } from '@/lib/utils';
import type { HTMLAttributes } from 'vue';

interface Props {
  src?: string;
  alt?: string;
  class?: HTMLAttributes['class'];
}

const props = defineProps<Props>();
</script>

<template>
  <img :src="props.src" :alt="props.alt" :class="cn('aspect-square h-full w-full', props.class)" />
</template>
