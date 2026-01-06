import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import EmailComposer from '@/components/EmailComposer.vue';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('EmailComposer', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  const mountWithStubs = (props: any) => {
    return mount(EmailComposer, {
      props,
      global: {
        stubs: {
          SidebarTrigger: {
            template: '<button data-sidebar-trigger>Toggle</button>',
          },
        },
      },
    });
  };

  it('renders composer form', () => {
    const wrapper = mountWithStubs({ accountId: 1 });

    expect(wrapper.text()).toContain('New Message');
    expect(wrapper.find('input[type="email"]').exists()).toBe(true);
  });

  it('shows Send and Cancel buttons', () => {
    const wrapper = mountWithStubs({ accountId: 1 });

    const buttons = wrapper.findAllComponents({ name: 'Button' });
    const buttonTexts = buttons.map(b => b.text());

    expect(buttonTexts).toContain('Send');
    expect(buttonTexts).toContain('Cancel');
  });

  it('emits close event when cancel is clicked', async () => {
    const wrapper = mountWithStubs({ accountId: 1 });

    const cancelButton = wrapper.findAll('button').find(b => b.text() === 'Cancel');
    await cancelButton?.trigger('click');

    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('shows error when sending without recipient', async () => {
    const wrapper = mountWithStubs({ accountId: 1 });

    const sendButton = wrapper.findAll('button').find(b => b.text() === 'Send');
    await sendButton?.trigger('click');

    expect(wrapper.text()).toContain('Please enter at least one recipient');
  });
});
