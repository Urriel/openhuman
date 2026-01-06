<script setup lang="ts">
import { computed, ref } from 'vue';
import EmailComposer from '@/components/EmailComposer.vue';
import EmailList from '@/components/EmailList.vue';
import EmailReader from '@/components/EmailReader.vue';
import FolderNavigation from '@/components/FolderNavigation.vue';
import type { FolderKey } from '@/components/FolderNavigation.vue';
import AppSidebar from '@/components/AppSidebar.vue';
import SearchBar from '@/components/SearchBar.vue';
import TopHeader from '@/components/TopHeader.vue';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';

const selectedFolder = ref<FolderKey>('INBOX');
const selectedLabel = ref<number | null>(null);
const selectedMessageId = ref<number | null>(null);
const isComposing = ref(false);
const searchResults = ref<any[]>([]);
const isSearching = ref(false);
const theme = ref<'light' | 'dark'>('light');

const emailListRef = ref<InstanceType<typeof EmailList> | null>(null);

function handleFolderSelect(folder: FolderKey) {
  selectedFolder.value = folder;
  selectedLabel.value = null;
  selectedMessageId.value = null;
  isSearching.value = false;
}

function handleMessageSelect(messageId: number) {
  selectedMessageId.value = messageId;
  isComposing.value = false;
}

function openComposer() {
  isComposing.value = true;
  selectedMessageId.value = null;
}

function closeComposer() {
  isComposing.value = false;
}

function handleEmailSent() {
  isComposing.value = false;
  emailListRef.value?.loadMessages();
}

function handleSearchResults(results: any[]) {
  searchResults.value = results;
  isSearching.value = true;
}

function handleSearchClear() {
  searchResults.value = [];
  isSearching.value = false;
}

function refreshList() {
  emailListRef.value?.loadMessages();
}

function toggleTheme() {
  theme.value = theme.value === 'light' ? 'dark' : 'light';
  document.documentElement.classList.toggle('dark');
}

const messageCount = computed(() => 20);
const lastUpdate = computed(() => '3 days ago');
</script>

<template>
  <SidebarProvider>
    <AppSidebar active-route="mail" />

    <!-- Full-width compose view -->
    <SidebarInset v-if="isComposing">
      <EmailComposer :account-id="1" @close="closeComposer" @sent="handleEmailSent" />
    </SidebarInset>

    <!-- Normal email client layout -->
    <SidebarInset v-else>
      <TopHeader
        :email-count="messageCount"
        :last-update-text="lastUpdate"
        :theme="theme"
        @toggle-theme="toggleTheme"
      />

      <FolderNavigation
        :active-folder="selectedFolder"
        :inbox-count="messageCount"
        @compose="openComposer"
        @select-folder="handleFolderSelect"
      />

      <div class="flex min-h-0 flex-1 overflow-hidden">
        <div class="flex w-full flex-col border-r bg-background md:w-[320px]">
          <div class="border-b px-3 py-2 md:px-4">
            <SearchBar @results="handleSearchResults" @clear="handleSearchClear" />
          </div>

          <div class="min-h-0 flex-1 overflow-hidden">
            <EmailList
              ref="emailListRef"
              :folder="selectedFolder"
              :label-id="selectedLabel ?? undefined"
              @select-message="handleMessageSelect"
            />
          </div>
        </div>

        <div class="flex min-w-0 flex-1 flex-col bg-background">
          <EmailReader :message-id="selectedMessageId ?? undefined" @refresh="refreshList" />
        </div>
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
