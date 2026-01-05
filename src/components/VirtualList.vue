<script setup lang="ts" generic="T">
import { computed, ref } from 'vue';
import { useVirtualizer } from '@tanstack/vue-virtual';

/**
 * VirtualList Component - A reusable virtualized list wrapper
 *
 * This component provides efficient rendering of large lists by only rendering
 * visible items in the viewport. Built with TanStack Virtual and Vue 3 Composition API.
 *
 * @template T - The type of items in the list
 *
 * Props:
 * - items: Array<T> - The array of items to render
 * - itemHeight: number - The estimated height of each item in pixels (default: 50)
 * - containerHeight: string - The height of the scrollable container (default: '400px')
 * - overscan: number - Number of items to render outside viewport for smoother scrolling (default: 5)
 *
 * Slots:
 * - default: Receives { item, index } for rendering each item
 *
 * Usage Example:
 * ```vue
 * <VirtualList :items="myItems" :item-height="60">
 *   <template #default="{ item, index }">
 *     <div>{{ item.name }}</div>
 *   </template>
 * </VirtualList>
 * ```
 */

interface Props {
  items: T[];
  itemHeight?: number;
  containerHeight?: string;
  overscan?: number;
}

const props = withDefaults(defineProps<Props>(), {
  itemHeight: 50,
  containerHeight: '400px',
  overscan: 5,
});

// Reference to the scrollable parent element
const parentRef = ref<HTMLElement | null>(null);

// Configure the virtualizer with computed options
const rowVirtualizerOptions = computed(() => ({
  count: props.items.length,
  getScrollElement: () => parentRef.value,
  estimateSize: () => props.itemHeight,
  overscan: props.overscan,
}));

// Create the virtualizer instance
const rowVirtualizer = useVirtualizer(rowVirtualizerOptions);

// Computed properties for virtual items and total size
const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems());
const totalSize = computed(() => rowVirtualizer.value.getTotalSize());

// Expose virtualizer methods for parent components
defineExpose({
  scrollToIndex: (index: number) => rowVirtualizer.value.scrollToIndex(index),
  scrollToOffset: (offset: number) => rowVirtualizer.value.scrollToOffset(offset),
});
</script>

<template>
  <div
    ref="parentRef"
    class="virtual-list-container"
    :style="{
      height: containerHeight,
      overflow: 'auto',
      position: 'relative',
    }"
  >
    <div
      class="virtual-list-wrapper"
      :style="{
        height: `${totalSize}px`,
        width: '100%',
        position: 'relative',
      }"
    >
      <div
        v-for="virtualRow in virtualRows"
        :key="String(virtualRow.key)"
        class="virtual-list-item"
        :style="{
          position: 'absolute',
          top: 0,
          left: 0,
          width: '100%',
          height: `${virtualRow.size}px`,
          transform: `translateY(${virtualRow.start}px)`,
        }"
      >
        <slot :item="items[virtualRow.index]" :index="virtualRow.index" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.virtual-list-container {
  /* Improve scrolling performance */
  will-change: scroll-position;
  -webkit-overflow-scrolling: touch;
}

.virtual-list-item {
  /* Optimize rendering performance */
  contain: layout style paint;
}
</style>
