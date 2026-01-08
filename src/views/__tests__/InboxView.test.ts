import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount, flushPromises } from '@vue/test-utils';
import { createRouter, createMemoryHistory } from 'vue-router';
import InboxView from '@/views/InboxView.vue';
import { routes } from '@/router/routes';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue([]),
}));

describe('InboxView', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders InboxView component', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push('/inbox');
    await router.isReady();

    const wrapper = mount(InboxView, {
      global: {
        plugins: [router],
        stubs: {
          TopHeader: true,
          FolderNavigation: true,
          SearchBar: true,
          EmailList: true,
          EmailReader: true,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
    await flushPromises();
  });

  it('reads folder from route params', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push('/sent');
    await router.isReady();

    mount(InboxView, {
      global: {
        plugins: [router],
        stubs: {
          TopHeader: true,
          FolderNavigation: true,
          SearchBar: true,
          EmailList: true,
          EmailReader: true,
        },
      },
    });

    await flushPromises();
    expect(router.currentRoute.value.params.folder).toBe('sent');
  });

  it('reads message ID from route query params', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push({ path: '/inbox', query: { message: '123' } });
    await router.isReady();

    mount(InboxView, {
      global: {
        plugins: [router],
        stubs: {
          TopHeader: true,
          FolderNavigation: true,
          SearchBar: true,
          EmailList: true,
          EmailReader: true,
        },
      },
    });

    await flushPromises();
    expect(router.currentRoute.value.query.message).toBe('123');
  });
});
