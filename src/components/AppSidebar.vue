<script setup lang="ts">
import type { SidebarProps } from '@/components/ui/sidebar';

import { Inbox, Mail, Tag, LifeBuoy, Settings2, Shield, Unplug } from 'lucide-vue-next';
import NavMain from '@/components/NavMain.vue';
import NavProjects from '@/components/NavProjects.vue';
import NavUser from '@/components/NavUser.vue';
import TeamSwitcher from '@/components/TeamSwitcher.vue';

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from '@/components/ui/sidebar';

const props = withDefaults(defineProps<SidebarProps>(), {
  collapsible: 'icon',
});

// OpenHuman email client data
const data = {
  user: {
    name: 'User',
    email: 'user@example.com',
    avatar: '/avatars/user.jpg',
  },
  teams: [
    {
      name: 'OpenHuman',
      logo: Mail,
      plan: 'Email Client',
    },
  ],
  navMain: [
    {
      title: 'Mail',
      url: '#',
      icon: Inbox,
      isActive: true,
      items: [
        {
          title: 'Inbox',
          url: '#',
        },
        {
          title: 'Sent',
          url: '#',
        },
        {
          title: 'Drafts',
          url: '#',
        },
        {
          title: 'Archive',
          url: '#',
        },
        {
          title: 'Trash',
          url: '#',
        },
      ],
    },
    {
      title: 'Labels',
      url: '#',
      icon: Tag,
      items: [
        {
          title: 'Work',
          url: '#',
        },
        {
          title: 'Personal',
          url: '#',
        },
        {
          title: 'Important',
          url: '#',
        },
      ],
    },
    {
      title: 'Settings',
      url: '#',
      icon: Settings2,
      items: [
        {
          title: 'General',
          url: '#',
        },
        {
          title: 'Accounts',
          url: '#',
        },
        {
          title: 'Appearance',
          url: '#',
        },
      ],
    },
  ],
  projects: [
    {
      name: 'Security',
      url: '#',
      icon: Shield,
    },
    {
      name: 'Offline Mode',
      url: '#',
      icon: Unplug,
    },
    {
      name: 'Help',
      url: '#',
      icon: LifeBuoy,
    },
  ],
};
</script>

<template>
  <Sidebar v-bind="props" data-testid="app-sidebar">
    <SidebarHeader>
      <TeamSwitcher :teams="data.teams" />
    </SidebarHeader>
    <SidebarContent>
      <NavMain :items="data.navMain" />
      <NavProjects :projects="data.projects" />
    </SidebarContent>
    <SidebarFooter>
      <NavUser :user="data.user" />
    </SidebarFooter>
    <SidebarRail />
  </Sidebar>
</template>
