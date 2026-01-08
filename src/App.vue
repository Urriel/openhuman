<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import EmailComposer from '@/components/EmailComposer.vue';
import AppSidebar from '@/components/AppSidebar.vue';
import CommandPalette from '@/components/CommandPalette.vue';
import KeyboardShortcutsHelp from '@/components/KeyboardShortcutsHelp.vue';
import { Toaster } from '@/components/ui/sonner';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import { useKeyboardShortcuts } from '@/composables/useKeyboardShortcuts';
import { useCommandPalette } from '@/composables/useCommandPalette';
import { useUndoStack } from '@/composables/useUndoStack';
import { toast } from 'vue-sonner';

const route = useRoute();
const router = useRouter();

// Composition state (kept as overlay, not a route)
const isComposing = ref(false);

// Command palette and keyboard shortcuts
const { openPalette } = useCommandPalette();
const { register } = useKeyboardShortcuts();
const { executeUndo, hasUndo } = useUndoStack();

const helpModalRef = ref<InstanceType<typeof KeyboardShortcutsHelp> | null>(null);

function openComposer() {
  isComposing.value = true;
}

function closeComposer() {
  isComposing.value = false;
}

function handleEmailSent() {
  isComposing.value = false;
  // Refresh will be handled by the EmailList component
}

function toggleTheme() {
  document.documentElement.classList.toggle('dark');
}

// Register global keyboard shortcuts
onMounted(() => {
  // Command palette shortcuts
  register('Cmd+K', () => openPalette('commands'), { global: true });
  register('/', () => openPalette('search'), { global: true });

  // Navigation shortcuts
  register('g', () => {}, { global: true }); // Prefix key for navigation
  register('i', () => void router.push('/inbox'), { global: true });
  register('s', () => void router.push('/sent'), { global: true });
  register('d', () => void router.push('/drafts'), { global: true });
  register('a', () => void router.push('/archive'), { global: true });

  // Compose shortcut
  register('c', () => openComposer(), { global: true });

  // Undo shortcut
  register('z', async () => {
    if (hasUndo.value) {
      await executeUndo();
      toast.success('Undone');
    } else {
      toast.info('Nothing to undo');
    }
  });

  // Theme toggle shortcut
  register('Cmd+Shift+D', () => toggleTheme());

  // Account management shortcut
  register('Cmd+,', () => void router.push('/settings/accounts'), { global: true });

  // Help modal shortcut
  register('?', () => helpModalRef.value?.open(), { global: true });
});
</script>

<template>
  <SidebarProvider>
    <AppSidebar :active-route="route?.path || '/inbox'" />

    <!-- Compose overlay (not a route) -->
    <SidebarInset v-if="isComposing">
      <EmailComposer :account-id="1" @close="closeComposer" @sent="handleEmailSent" />
    </SidebarInset>

    <!-- Router-based views -->
    <SidebarInset v-else>
      <RouterView />
    </SidebarInset>

    <!-- Global components -->
    <CommandPalette />
    <KeyboardShortcutsHelp ref="helpModalRef" />
    <Toaster position="top-right" />
  </SidebarProvider>
</template>
