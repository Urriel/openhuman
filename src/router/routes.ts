import type { RouteRecordRaw } from 'vue-router';

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/inbox',
  },
  {
    path: '/:folder(inbox|sent|drafts|favorites|archive|deleted|spam|junk)',
    name: 'folder',
    component: () => import('@/views/InboxView.vue'),
    meta: {
      context: 'inbox',
    },
  },
  {
    path: '/settings/accounts',
    name: 'settings-accounts',
    component: () => import('@/views/SettingsView.vue'),
    meta: {
      context: 'settings',
    },
  },
];
