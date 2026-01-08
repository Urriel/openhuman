import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { createRouter, createMemoryHistory } from 'vue-router';
import { useEmailNavigation } from '@/composables/useEmailNavigation';
import { routes } from '@/router/routes';
import { defineComponent } from 'vue';

describe('useEmailNavigation', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('navigates to a specific folder', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push('/inbox');
    await router.isReady();

    const TestComponent = defineComponent({
      template: '<div></div>',
      setup() {
        const nav = useEmailNavigation();
        nav.navigateToFolder('Sent');
        return {};
      },
    });

    mount(TestComponent, {
      global: {
        plugins: [router],
      },
    });

    await vi.waitFor(() => {
      expect(router.currentRoute.value.path).toBe('/sent');
    });
  });

  it('selects an email by updating query param', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push('/inbox');
    await router.isReady();

    const TestComponent = defineComponent({
      template: '<div></div>',
      setup() {
        const nav = useEmailNavigation();
        nav.selectEmail(123);
        return {};
      },
    });

    mount(TestComponent, {
      global: {
        plugins: [router],
      },
    });

    await vi.waitFor(() => {
      expect(router.currentRoute.value.query.message).toBe('123');
    });
  });

  it('preserves other query params when selecting email', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push({ path: '/inbox', query: { label: 'work' } });
    await router.isReady();

    const TestComponent = defineComponent({
      template: '<div></div>',
      setup() {
        const nav = useEmailNavigation();
        nav.selectEmail(456);
        return {};
      },
    });

    mount(TestComponent, {
      global: {
        plugins: [router],
      },
    });

    await vi.waitFor(() => {
      expect(router.currentRoute.value.query.message).toBe('456');
      expect(router.currentRoute.value.query.label).toBe('work');
    });
  });

  it('navigates to settings', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push('/inbox');
    await router.isReady();

    const TestComponent = defineComponent({
      template: '<div></div>',
      setup() {
        const nav = useEmailNavigation();
        nav.navigateToSettings();
        return {};
      },
    });

    mount(TestComponent, {
      global: {
        plugins: [router],
      },
    });

    await vi.waitFor(() => {
      expect(router.currentRoute.value.path).toBe('/settings/accounts');
    });
  });
});
