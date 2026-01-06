import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import BulkActionsToolbar from '../BulkActionsToolbar.vue';

describe('BulkActionsToolbar', () => {
  it('does not render when selectedCount is 0', () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 0,
      },
    });

    expect(wrapper.find('[data-testid="bulk-actions-toolbar"]').exists()).toBe(false);
  });

  it('renders when selectedCount is greater than 0', () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 3,
      },
    });

    expect(wrapper.find('[data-testid="bulk-actions-toolbar"]').exists()).toBe(true);
  });

  it('displays correct selected count', () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 5,
      },
    });

    expect(wrapper.text()).toContain('5 selected');
  });

  it('emits archive event when archive button is clicked', async () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 2,
      },
    });

    await wrapper.find('[data-testid="bulk-archive-btn"]').trigger('click');
    expect(wrapper.emitted('archive')).toBeTruthy();
  });

  it('emits delete event when delete button is clicked', async () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 2,
      },
    });

    await wrapper.find('[data-testid="bulk-delete-btn"]').trigger('click');
    expect(wrapper.emitted('delete')).toBeTruthy();
  });

  it('emits markRead event when mark read button is clicked', async () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 2,
      },
    });

    await wrapper.find('[data-testid="bulk-mark-read-btn"]').trigger('click');
    expect(wrapper.emitted('markRead')).toBeTruthy();
  });

  it('emits markUnread event when mark unread button is clicked', async () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 2,
      },
    });

    await wrapper.find('[data-testid="bulk-mark-unread-btn"]').trigger('click');
    expect(wrapper.emitted('markUnread')).toBeTruthy();
  });

  it('renders all action buttons', () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 1,
      },
    });

    expect(wrapper.find('[data-testid="bulk-archive-btn"]').exists()).toBe(true);
    expect(wrapper.find('[data-testid="bulk-delete-btn"]').exists()).toBe(true);
    expect(wrapper.find('[data-testid="bulk-mark-read-btn"]').exists()).toBe(true);
    expect(wrapper.find('[data-testid="bulk-mark-unread-btn"]').exists()).toBe(true);
  });

  it('has proper ARIA labels on buttons', () => {
    const wrapper = mount(BulkActionsToolbar, {
      props: {
        selectedCount: 1,
      },
    });

    expect(wrapper.find('[data-testid="bulk-archive-btn"]').attributes('aria-label')).toBe(
      'Archive selected emails'
    );
    expect(wrapper.find('[data-testid="bulk-delete-btn"]').attributes('aria-label')).toBe(
      'Delete selected emails'
    );
    expect(wrapper.find('[data-testid="bulk-mark-read-btn"]').attributes('aria-label')).toBe(
      'Mark selected emails as read'
    );
    expect(wrapper.find('[data-testid="bulk-mark-unread-btn"]').attributes('aria-label')).toBe(
      'Mark selected emails as unread'
    );
  });
});
