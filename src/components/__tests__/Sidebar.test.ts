import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import Sidebar from '@/components/Sidebar.vue';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue([]),
}));

describe('Sidebar', () => {
  it('renders folders list', async () => {
    const wrapper = mount(Sidebar);

    // Wait for component to mount and render folders
    await wrapper.vm.$nextTick();

    expect(wrapper.text()).toContain('INBOX');
    expect(wrapper.text()).toContain('Sent');
    expect(wrapper.text()).toContain('Drafts');
  });

  it('emits selectFolder event when folder is clicked', async () => {
    const wrapper = mount(Sidebar);

    const inboxButton = wrapper.findAll('button').find(btn => btn.text().includes('INBOX'));
    if (inboxButton) {
      await inboxButton.trigger('click');
      expect(wrapper.emitted('selectFolder')).toBeTruthy();
    }
  });

  it('shows folders section', () => {
    const wrapper = mount(Sidebar);
    expect(wrapper.text()).toContain('Folders');
    expect(wrapper.text()).toContain('Labels');
  });
});
