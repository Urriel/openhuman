<script setup lang="ts">
import { computed } from 'vue';
import { useCommandPalette } from '@/composables/useCommandPalette';
import { useEmailActions } from '@/composables/useEmailActions';
import { onKeyStroke } from '@vueuse/core';
import {
  CommandDialog,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
} from '@/components/ui/command';
import { Loader2 } from 'lucide-vue-next';

const {
  isOpen,
  mode,
  query,
  currentPage,
  filteredCommands,
  filteredLabels,
  searchResults,
  isSearching,
  closePalette,
  executeCommand,
  popPage,
} = useCommandPalette();

const emailActions = useEmailActions();

/**
 * Group commands by category
 */
const groupedCommands = computed(() => {
  const groups = new Map<string, typeof filteredCommands.value>();
  filteredCommands.value.forEach(cmd => {
    if (!groups.has(cmd.category)) {
      groups.set(cmd.category, []);
    }
    groups.get(cmd.category)!.push(cmd);
  });
  return groups;
});

/**
 * Handle command selection
 */
async function handleSelect(commandId: string) {
  await executeCommand(commandId);
  closePalette();
}

/**
 * Handle label selection
 */
async function handleLabelSelect(labelId: number, labelName: string) {
  // TODO: Get current email ID from context
  const currentEmailId = 1; // Placeholder
  await emailActions.applyLabel(currentEmailId, labelId, labelName);
  popPage();
}

/**
 * Handle email search result selection
 */
function handleEmailSelect(emailId: number) {
  // TODO: Navigate to email
  console.log('Open email:', emailId);
  closePalette();
}

/**
 * Handle Backspace key to navigate back
 */
onKeyStroke('Backspace', e => {
  if (isOpen.value && query.value === '' && currentPage.value !== 'main') {
    e.preventDefault();
    popPage();
  }
});

/**
 * Get placeholder text based on mode and page
 */
const placeholder = computed(() => {
  if (currentPage.value === 'labels') {
    return 'Search labels...';
  }
  if (mode.value === 'search') {
    return 'Search emails...';
  }
  return 'Type a command or search...';
});
</script>

<template>
  <CommandDialog v-model:open="isOpen">
    <CommandInput v-model="query" :placeholder="placeholder" />

    <!-- Main commands page -->
    <CommandList v-if="currentPage === 'main' && mode === 'commands'">
      <CommandEmpty>No commands found.</CommandEmpty>
      <CommandGroup
        v-for="[category, commands] in groupedCommands"
        :key="category"
        :heading="category"
      >
        <CommandItem
          v-for="cmd in commands"
          :key="cmd.id"
          :value="cmd.id"
          :disabled="cmd.disabled"
          @select="handleSelect(cmd.id)"
        >
          <span>{{ cmd.label }}</span>
          <span v-if="cmd.shortcut" class="ml-auto text-xs text-muted-foreground">{{
            cmd.shortcut
          }}</span>
        </CommandItem>
      </CommandGroup>
    </CommandList>

    <!-- Search mode -->
    <CommandList v-else-if="mode === 'search'">
      <div v-if="isSearching" class="flex items-center justify-center p-4">
        <Loader2 class="h-4 w-4 animate-spin" />
      </div>
      <CommandEmpty v-else-if="searchResults.length === 0 && query">
        No emails found.
      </CommandEmpty>
      <CommandGroup v-else-if="searchResults.length > 0" heading="Search Results">
        <CommandItem
          v-for="result in searchResults"
          :key="result.id"
          :value="result.id.toString()"
          @select="handleEmailSelect(result.id)"
        >
          <div class="flex flex-col gap-1 w-full">
            <div class="flex items-center gap-2">
              <div
                class="h-2 w-2 rounded-full"
                :class="result.is_read ? 'bg-transparent' : 'bg-blue-500'"
              ></div>
              <span class="font-semibold">{{ result.subject }}</span>
              <span v-if="result.is_starred" class="ml-auto text-yellow-500">★</span>
            </div>
            <div class="text-xs text-muted-foreground">{{ result.sender }}</div>
            <div class="text-xs text-muted-foreground truncate" v-html="result.snippet"></div>
          </div>
        </CommandItem>
      </CommandGroup>
    </CommandList>

    <!-- Labels page -->
    <CommandList v-else-if="currentPage === 'labels'">
      <CommandEmpty>No labels found.</CommandEmpty>
      <CommandGroup heading="Apply Label">
        <CommandItem
          v-for="label in filteredLabels"
          :key="label.id"
          :value="label.name"
          @select="handleLabelSelect(label.id, label.name)"
        >
          <div class="flex items-center gap-2">
            <div
              class="h-3 w-3 rounded-full"
              :style="{ backgroundColor: label.color || '#6B7280' }"
            ></div>
            <span>{{ label.name }}</span>
          </div>
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>
