import type { RouteRecordRaw } from 'vue-router';

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/inbox',
  },
  {
    path: '/:folder(inbox|sent|drafts|trash)',
    name: 'folder',
    component: () => import('@/App.vue'),
  },
  {
    path: '/compose',
    name: 'compose',
    component: () => import('@/App.vue'),
  },
  {
    path: '/settings/accounts',
    name: 'settings-accounts',
    component: () => import('@/App.vue'),
  },
  {
    path: '/archive',
    name: 'archive',
    component: () => import('@/App.vue'),
  },
  {
    path: '/favorites',
    name: 'favorites',
    component: () => import('@/App.vue'),
  },
];
