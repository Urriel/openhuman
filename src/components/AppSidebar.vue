<script setup lang="ts">
import {
  AudioLines,
  Files,
  FolderClosed,
  GalleryVerticalEnd,
  GitFork,
  LifeBuoy,
  Mail,
  Shield,
  Sparkles,
  Unplug,
} from 'lucide-vue-next';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';

const props = withDefaults(
  defineProps<{
    activeRoute?: 'mail' | 'sparkles' | 'gallery' | 'folder' | 'audio' | 'git' | 'files';
  }>(),
  {
    activeRoute: 'mail',
  }
);

const navItems = [
  { key: 'sparkles', label: 'Sparkles', icon: Sparkles },
  { key: 'gallery', label: 'Gallery', icon: GalleryVerticalEnd },
  { key: 'mail', label: 'Mail', icon: Mail },
  { key: 'folder', label: 'Folder', icon: FolderClosed },
  { key: 'audio', label: 'Audio', icon: AudioLines },
  { key: 'git', label: 'Git', icon: GitFork },
  { key: 'files', label: 'Files', icon: Files },
];

const footerItems = [
  { key: 'security', label: 'Security', icon: Shield },
  { key: 'unplug', label: 'Unplug', icon: Unplug },
  { key: 'help', label: 'Help', icon: LifeBuoy },
];
</script>

<template>
  <Sidebar collapsible="icon" data-testid="app-sidebar">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton
            size="lg"
            class="bg-linear-to-br from-white via-purple-200 to-blue-200 data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <div
              class="flex aspect-square size-8 items-center justify-center rounded-lg text-sidebar-primary-foreground"
            >
              <span class="text-sm font-semibold text-foreground">O</span>
            </div>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-semibold">OpenHuman</span>
              <span class="truncate text-xs">Email Client</span>
            </div>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>

    <SidebarContent>
      <SidebarGroup>
        <SidebarMenu>
          <SidebarMenuItem v-for="item in navItems" :key="item.key">
            <SidebarMenuButton :is-active="props.activeRoute === item.key" :aria-label="item.label">
              <component :is="item.icon" class="size-4" />
              <span>{{ item.label }}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem v-for="item in footerItems" :key="item.key">
          <SidebarMenuButton :aria-label="item.label">
            <component :is="item.icon" class="size-4" />
            <span>{{ item.label }}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
        <SidebarMenuItem>
          <SidebarMenuButton
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <Avatar class="h-8 w-8">
              <AvatarFallback>U</AvatarFallback>
            </Avatar>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-semibold">User</span>
              <span class="truncate text-xs">user@example.com</span>
            </div>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>
  </Sidebar>
</template>
