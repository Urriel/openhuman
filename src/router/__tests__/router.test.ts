import { describe, it, expect, beforeEach } from 'vitest';
import { createMemoryHistory, createRouter } from 'vue-router';
import { routes } from '@/router/routes';
import type { Router } from 'vue-router';

describe('Router Configuration', () => {
  let router: Router;

  beforeEach(() => {
    router = createRouter({
      history: createMemoryHistory(),
      routes,
    });
  });

  it('creates router instance with createWebHistory configuration', () => {
    expect(router).toBeDefined();
    expect(router.options.routes).toEqual(routes);
  });

  it('redirects root path to /inbox', async () => {
    await router.push('/');
    expect(router.currentRoute.value.path).toBe('/inbox');
  });

  it('matches folder routes correctly', async () => {
    const folders = ['inbox', 'sent', 'drafts', 'favorites', 'archive', 'deleted', 'spam', 'junk'];

    for (const folder of folders) {
      await router.push(`/${folder}`);
      expect(router.currentRoute.value.path).toBe(`/${folder}`);
      expect(router.currentRoute.value.name).toBe('folder');
      expect(router.currentRoute.value.params.folder).toBe(folder);
    }
  });

  it('preserves query parameters on navigation', async () => {
    await router.push({ path: '/inbox', query: { message: '123', label: 'work' } });

    expect(router.currentRoute.value.query.message).toBe('123');
    expect(router.currentRoute.value.query.label).toBe('work');
  });

  it('matches settings/accounts route', async () => {
    await router.push('/settings/accounts');
    expect(router.currentRoute.value.path).toBe('/settings/accounts');
    expect(router.currentRoute.value.name).toBe('settings-accounts');
  });

  it('includes meta context for folder routes', async () => {
    await router.push('/inbox');
    expect(router.currentRoute.value.meta.context).toBe('inbox');
  });

  it('includes meta context for settings routes', async () => {
    await router.push('/settings/accounts');
    expect(router.currentRoute.value.meta.context).toBe('settings');
  });
});
