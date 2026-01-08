<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import EmailList from '@/components/EmailList.vue';
import EmailReader from '@/components/EmailReader.vue';
import FolderNavigation from '@/components/FolderNavigation.vue';
import type { FolderKey } from '@/components/FolderNavigation.vue';
import SearchBar from '@/components/SearchBar.vue';
import TopHeader from '@/components/TopHeader.vue';

const route = useRoute();

// Extract folder from route params (default to INBOX)
const currentFolder = computed<FolderKey>(() => {
  const folder = route.params.folder as string;
  if (!folder) return 'INBOX';
  
  // Convert lowercase folder name to FolderKey format
  const folderMap: Record<string, FolderKey> = {
    inbox: 'INBOX',
    sent: 'Sent',
    drafts: 'Drafts',
    favorites: 'Favorites',
    archive: 'Archive',
    deleted: 'Deleted',
    spam: 'Spam',
    junk: 'Junk',
  };
  
  return folderMap[folder.toLowerCase()] || 'INBOX';
});

// Extract message ID from query params
const selectedMessageId = computed(() => {
  const messageId = route.query.message;
  return messageId ? Number(messageId) : undefined;
});

// Extract label ID from query params
const selectedLabel = computed(() => {
  const labelId = route.query.label;
  return labelId ? Number(labelId) : undefined;
});

// Computed properties for TopHeader
const messageCount = computed(() => 20); // TODO: Get actual count
const lastUpdate = computed(() => '3 days ago'); // TODO: Get actual last update

// Theme state (TODO: move to global store or composable)
const theme = computed<'light' | 'dark'>(() => 'light');

function handleToggleTheme() {
  document.documentElement.classList.toggle('dark');
}

function handleSearchResults(results: any[]) {
  // TODO: Handle search results
  console.log('Search results:', results);
}

function handleSearchClear() {
  // TODO: Clear search
  console.log('Search cleared');
}

function handleRefresh() {
  // TODO: Refresh email list
  console.log('Refresh requested');
}
</script>

<template>
  <div class="flex h-full flex-col">
    <TopHeader
      :email-count="messageCount"
      :last-update-text="lastUpdate"
      :theme="theme"
      @toggle-theme="handleToggleTheme"
    />

    <FolderNavigation :inbox-count="messageCount" />

    <div class="flex min-h-0 flex-1 overflow-hidden">
      <div class="flex w-full flex-col border-r bg-background md:w-[320px]">
        <div class="border-b px-3 py-2 md:px-4">
          <SearchBar @results="handleSearchResults" @clear="handleSearchClear" />
        </div>

        <div class="min-h-0 flex-1 overflow-hidden">
          <EmailList :folder="currentFolder" :label-id="selectedLabel" />
        </div>
      </div>

      <div class="flex min-w-0 flex-1 flex-col bg-background">
        <EmailReader :message-id="selectedMessageId" @refresh="handleRefresh" />
      </div>
    </div>
  </div>
</template>
