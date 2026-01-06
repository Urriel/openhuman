<script setup lang="ts">
import { ref } from 'vue';

interface Props {
  sidebarCollapsed?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  sidebarCollapsed: false,
});

const collapsed = ref(props.sidebarCollapsed);
</script>

<template>
  <div class="flex h-screen w-screen overflow-hidden bg-background">
    <!-- Left Sidebar -->
    <aside
      :class="[
        'flex flex-col border-r border-border bg-muted/10 transition-all duration-300',
        collapsed ? 'w-0' : 'w-60',
      ]"
      style="min-width: 0"
    >
      <div v-if="!collapsed" class="flex-1 overflow-y-auto p-4">
        <slot name="sidebar" />
      </div>
    </aside>

    <!-- Main Content Area -->
    <div class="flex min-w-0 flex-1 overflow-hidden">
      <!-- Email List Pane -->
      <div class="flex w-96 min-w-[20rem] max-w-md flex-col border-r border-border bg-background">
        <div class="flex-1 overflow-hidden">
          <slot name="list" />
        </div>
      </div>

      <!-- Reader/Composer Pane -->
      <div class="flex min-w-0 flex-1 flex-col bg-background">
        <div class="flex-1 overflow-hidden">
          <slot name="content" />
        </div>
      </div>
    </div>
  </div>
</template>
