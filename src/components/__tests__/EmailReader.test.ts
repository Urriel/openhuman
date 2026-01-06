import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import EmailReader from '@/components/EmailReader.vue';
import EmailReaderToolbar from '@/components/EmailReaderToolbar.vue';
import { invoke } from '@tauri-apps/api/core';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('EmailReader', () => {
  const mockMessage = {
    id: 1,
    subject: 'Test Email',
    from_addr: 'sender@example.com',
    to_addr: 'recipient@example.com',
    cc_addr: null,
    bcc_addr: null,
    date: '2024-01-01T12:00:00Z',
    body_plain: 'This is the plain text body',
    body_html: '<p>This is the HTML body</p>',
    is_read: false,
    is_starred: false,
    attachments: [
      {
        id: 1,
        filename: 'test.pdf',
        size: 1024,
        mime_type: 'application/pdf',
      },
    ],
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders message content', async () => {
    vi.mocked(invoke).mockResolvedValue(mockMessage);

    const wrapper = mount(EmailReader, {
      props: {
        messageId: 1,
      },
      global: {
        components: {
          EmailReaderToolbar,
        },
      },
    });

    await vi.waitFor(() => {
      expect(wrapper.text()).toContain('Test Email');
    });
  });

  it('shows empty state when no message selected', () => {
    const wrapper = mount(EmailReader, {
      global: {
        components: {
          EmailReaderToolbar,
        },
      },
    });

    expect(wrapper.text()).toContain('Select an email to read');
  });

  it('renders toolbar buttons', async () => {
    vi.mocked(invoke).mockResolvedValue(mockMessage);

    const wrapper = mount(EmailReader, {
      props: {
        messageId: 1,
      },
      global: {
        components: {
          EmailReaderToolbar,
        },
      },
    });

    await vi.waitFor(() => {
      // Check that toolbar buttons are present
      const buttons = wrapper.findAll('button');
      expect(buttons.length).toBeGreaterThan(0);
      // Check for action buttons (Reply, Archive, Delete, Star)
      expect(wrapper.html()).toContain('title="Reply"');
      expect(wrapper.html()).toContain('title="Archive"');
      expect(wrapper.html()).toContain('title="Delete"');
    });
  });

  it('displays sender information', async () => {
    vi.mocked(invoke).mockResolvedValue(mockMessage);

    const wrapper = mount(EmailReader, {
      props: {
        messageId: 1,
      },
      global: {
        components: {
          EmailReaderToolbar,
        },
      },
    });

    await vi.waitFor(() => {
      expect(wrapper.text()).toContain('sender@example.com');
      expect(wrapper.text()).toContain('To: recipient@example.com');
    });
  });
});
