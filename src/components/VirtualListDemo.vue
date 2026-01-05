<script setup lang="ts">
import { ref, computed } from 'vue';
import VirtualList from './VirtualList.vue';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

/**
 * VirtualListDemo Component
 *
 * Demonstrates the VirtualList component with 10,000+ items to verify
 * smooth scrolling performance at 60 FPS.
 *
 * Features:
 * - Generates 10,000 demo items
 * - Scroll to specific indices for testing
 * - Different item styles for visual variety
 */

interface DemoItem {
  id: number;
  title: string;
  description: string;
}

// Generate 10,000 demo items
const ITEM_COUNT = 10000;
const items = ref<DemoItem[]>(
  Array.from({ length: ITEM_COUNT }, (_, i) => ({
    id: i,
    title: `Item ${i + 1}`,
    description: `This is item number ${i + 1} out of ${ITEM_COUNT} items`,
  }))
);

// Reference to the VirtualList component
const virtualListRef = ref<{
  scrollToIndex: (index: number) => void;
  scrollToOffset: (offset: number) => void;
} | null>(null);

// Scroll to specific indices
const scrollToTop = () => {
  virtualListRef.value?.scrollToIndex(0);
};

const scrollToMiddle = () => {
  virtualListRef.value?.scrollToIndex(Math.floor(ITEM_COUNT / 2));
};

const scrollToBottom = () => {
  virtualListRef.value?.scrollToIndex(ITEM_COUNT - 1);
};

// Performance info
const itemCount = computed(() => items.value.length.toLocaleString());
</script>

<template>
  <Card class="w-full max-w-3xl mx-auto">
    <CardHeader>
      <CardTitle>TanStack Virtual Demo</CardTitle>
      <CardDescription>
        Virtualized list with {{ itemCount }} items for smooth 60 FPS scrolling
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Control buttons -->
      <div class="flex gap-2 flex-wrap">
        <Button @click="scrollToTop" size="sm"> Scroll to Top </Button>
        <Button @click="scrollToMiddle" size="sm" variant="secondary">
          Scroll to Middle (Item {{ Math.floor(ITEM_COUNT / 2) + 1 }})
        </Button>
        <Button @click="scrollToBottom" size="sm" variant="outline"> Scroll to Bottom </Button>
      </div>

      <!-- Virtual List -->
      <VirtualList
        ref="virtualListRef"
        :items="items"
        :item-height="72"
        container-height="500px"
        :overscan="10"
      >
        <template #default="{ item, index }">
          <div
            class="p-4 border-b transition-colors hover:bg-accent"
            :class="{
              'bg-blue-50 dark:bg-blue-950': index % 10 === 0,
              'bg-muted/50': index % 2 === 0 && index % 10 !== 0,
            }"
          >
            <div class="font-semibold text-sm">{{ item.title }}</div>
            <div class="text-xs text-muted-foreground">
              {{ item.description }}
            </div>
          </div>
        </template>
      </VirtualList>

      <!-- Performance note -->
      <div class="text-xs text-muted-foreground text-center pt-2">
        Only visible items are rendered - scroll to see smooth performance!
      </div>
    </CardContent>
  </Card>
</template>
