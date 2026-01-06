import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import AppShell from '@/components/AppShell.vue';

describe('AppShell', () => {
  it('renders three-pane layout correctly', () => {
    const wrapper = mount(AppShell, {
      slots: {
        sidebar: '<div class="test-sidebar">Sidebar</div>',
        list: '<div class="test-list">Email List</div>',
        content: '<div class="test-content">Content</div>',
      },
    });

    expect(wrapper.find('.test-sidebar').exists()).toBe(true);
    expect(wrapper.find('.test-list').exists()).toBe(true);
    expect(wrapper.find('.test-content').exists()).toBe(true);
  });

  it('collapses sidebar when sidebarCollapsed prop is true', () => {
    const wrapper = mount(AppShell, {
      props: {
        sidebarCollapsed: true,
      },
      slots: {
        sidebar: '<div>Sidebar</div>',
      },
    });

    const sidebar = wrapper.find('aside');
    expect(sidebar.classes()).toContain('w-0');
  });

  it('shows sidebar when sidebarCollapsed prop is false', () => {
    const wrapper = mount(AppShell, {
      props: {
        sidebarCollapsed: false,
      },
      slots: {
        sidebar: '<div>Sidebar</div>',
      },
    });

    const sidebar = wrapper.find('aside');
    expect(sidebar.classes()).toContain('w-60');
  });
});
