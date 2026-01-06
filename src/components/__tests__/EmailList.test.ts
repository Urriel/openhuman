import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import EmailList from '@/components/EmailList.vue';

// Mock Tauri invoke at the module level
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

describe('EmailList', () => {
  const mockMessages = [
    {
      id: 1,
      subject: 'Test Email 1',
      from_addr: 'sender1@example.com',
      preview: 'This is a preview',
      date: '2024-01-01T12:00:00Z',
      is_read: false,
      is_starred: false,
      has_attachments: false,
    },
    {
      id: 2,
      subject: 'Test Email 2',
      from_addr: 'sender2@example.com',
      preview: 'Another preview',
      date: '2024-01-02T12:00:00Z',
      is_read: true,
      is_starred: true,
      has_attachments: true,
    },
  ];

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders email list component', async () => {
    vi.mocked(invoke).mockResolvedValue(mockMessages);

    mount(EmailList, {
      props: {
        folder: 'INBOX',
      },
    });

    await vi.waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('list_messages', expect.any(Object));
    });
  });

  it('shows empty state when no messages', async () => {
    vi.mocked(invoke).mockResolvedValue([]);

    const wrapper = mount(EmailList);

    await vi.waitFor(() => {
      expect(wrapper.text()).toContain('No messages found');
    });
  });

  it('shows loading state initially', async () => {
    vi.mocked(invoke).mockImplementation(() => new Promise(() => {}));

    const wrapper = mount(EmailList);

    // Wait for component to mount and start loading
    await vi.waitFor(() => {
      expect(wrapper.text()).toContain('Loading messages...');
    });
  });
});
