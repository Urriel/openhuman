import { describe, it, expect, beforeEach } from 'vitest';
import { createRouter, createMemoryHistory } from 'vue-router';
import { routes } from '@/router/routes';

describe('Sidebar Navigation Integration', () => {
  let router: ReturnType<typeof createRouter>;

  beforeEach(async () => {
    router = createRouter({
      history: createMemoryHistory(),
      routes,
    });
    await router.push('/inbox');
    await router.isReady();
  });

  it('navigates to /inbox via router', async () => {
    await router.push('/inbox');
    expect(router.currentRoute.value.path).toBe('/inbox');
    expect(router.currentRoute.value.name).toBe('folder');
  });

  it('navigates to /trash via router', async () => {
    await router.push('/trash');
    expect(router.currentRoute.value.path).toBe('/trash');
    expect(router.currentRoute.value.name).toBe('folder');
    expect(router.currentRoute.value.params.folder).toBe('trash');
  });

  it('navigates to /sent via router', async () => {
    await router.push('/sent');
    expect(router.currentRoute.value.path).toBe('/sent');
    expect(router.currentRoute.value.name).toBe('folder');
  });

  it('navigates to /settings/accounts via router', async () => {
    await router.push('/settings/accounts');
    expect(router.currentRoute.value.path).toBe('/settings/accounts');
    expect(router.currentRoute.value.name).toBe('settings-accounts');
  });
});
