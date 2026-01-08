import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount, flushPromises } from '@vue/test-utils';
import { createRouter, createMemoryHistory } from 'vue-router';
import SettingsView from '@/views/SettingsView.vue';
import { routes } from '@/router/routes';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue([]),
}));

describe('SettingsView', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders SettingsView component', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push('/settings/accounts');
    await router.isReady();

    const wrapper = mount(SettingsView, {
      global: {
        plugins: [router],
        stubs: {
          AccountManagement: true,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
    await flushPromises();
  });

  it('includes AccountManagement component', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes,
    });

    await router.push('/settings/accounts');
    await router.isReady();

    const wrapper = mount(SettingsView, {
      global: {
        plugins: [router],
        stubs: {
          AccountManagement: true,
        },
      },
    });

    await flushPromises();
    
    // Verify AccountManagement is included
    expect(wrapper.html()).toContain('account-management');
  });
});
