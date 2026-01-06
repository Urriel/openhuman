<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Loader2 } from 'lucide-vue-next';

interface SearchResult {
  id: number;
  subject?: string;
  from_addr: string;
  snippet: string;
}

interface Props {
  accountId?: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  results: [results: SearchResult[]];
  clear: [];
}>();

const query = ref('');
const isSearching = ref(false);
const debounceTimer = ref<ReturnType<typeof setTimeout> | null>(null);

async function performSearch() {
  if (!query.value.trim()) {
    emit('clear');
    return;
  }

  try {
    isSearching.value = true;
    const results = await invoke<SearchResult[]>('search_messages', {
      query: query.value,
      accountId: props.accountId,
      limit: 1000,
    });
    emit('results', results);
  } catch (error) {
    console.error('Search failed:', error);
  } finally {
    isSearching.value = false;
  }
}

function clearSearch() {
  query.value = '';
  emit('clear');
}

// Debounce search input
watch(query, () => {
  if (debounceTimer.value) {
    clearTimeout(debounceTimer.value);
  }

  if (!query.value.trim()) {
    emit('clear');
    return;
  }

  debounceTimer.value = setTimeout(() => {
    performSearch();
  }, 300);
});
</script>

<template>
  <div class="flex items-center gap-2 px-0 py-0" data-testid="search-bar">
    <div class="relative flex-1">
      <Input
        v-model="query"
        type="search"
        placeholder="Search emails..."
        class="pr-8"
        aria-label="Search emails"
      />
      <Loader2
        v-if="isSearching"
        :size="16"
        class="absolute right-2 top-2.5 animate-spin text-muted-foreground"
      />
    </div>
    <Button v-if="query" variant="ghost" size="sm" @click="clearSearch">Clear</Button>
  </div>
</template>
