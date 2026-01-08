import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount, VueWrapper } from '@vue/test-utils';
import AccountManagement from '@/components/AccountManagement.vue';
import type { Account, TestConnectionResult } from '@/types/commands';

// Mock Tauri invoke
const mockInvoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

// Mock vue-sonner - IMPORTANT: Define inline to avoid hoisting issues
vi.mock('vue-sonner', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
    info: vi.fn(),
  },
}));

// Get reference to mock after it's hoisted
const { toast: mockToast } = await import('vue-sonner');

// Stub for SidebarTrigger to avoid Sidebar context requirement
const SidebarTriggerStub = {
  template: '<button data-testid="sidebar-trigger">Toggle Sidebar</button>',
};

// Mount options with stubs for sidebar components
const mountOptions = {
  global: {
    stubs: {
      SidebarTrigger: SidebarTriggerStub,
    },
  },
};

describe('AccountManagement', () => {
  let wrapper: VueWrapper<any>;

  const mockAccounts: Account[] = [
    {
      id: 1,
      email: 'test@gmail.com',
      provider: 'Gmail',
      imap_host: 'imap.gmail.com',
      imap_port: 993,
      smtp_host: 'smtp.gmail.com',
      smtp_port: 587,
      sync_enabled: true,
    },
    {
      id: 2,
      email: 'work@outlook.com',
      provider: 'Outlook',
      imap_host: 'imap.outlook.com',
      imap_port: 993,
      smtp_host: 'smtp.outlook.com',
      smtp_port: 587,
      sync_enabled: false,
    },
  ];

  beforeEach(() => {
    vi.clearAllMocks();
    // Default: all invokes return mockAccounts
    // Individual tests can override this with mockResolvedValueOnce/mockRejectedValueOnce
    mockInvoke.mockImplementation((cmd: string) => {
      if (cmd === 'list_accounts') {
        return Promise.resolve(mockAccounts);
      }
      return Promise.resolve([]);
    });
  });

  afterEach(() => {
    if (wrapper) {
      wrapper.unmount();
    }
  });

  describe('Rendering', () => {
    it('should render the component', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      expect(wrapper.find('h1').text()).toBe('Email Accounts');
    });

    it('should load and display accounts on mount', async () => {
      wrapper = mount(AccountManagement, mountOptions);

      // Wait for the component to mount and load accounts
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 200));

      expect(mockInvoke).toHaveBeenCalledWith('list_accounts');

      // Force another update to ensure rendering is complete
      await wrapper.vm.$nextTick();

      const text = wrapper.text();
      expect(text).toContain('test@gmail.com');
      expect(text).toContain('work@outlook.com');
    });

    it('should display account count correctly', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      expect(wrapper.text()).toContain('2 accounts');
    });

    it('should show empty state when no accounts exist', async () => {
      mockInvoke.mockResolvedValue([]);
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      expect(wrapper.text()).toContain('No accounts configured');
      expect(wrapper.text()).toContain('Add Your First Account');
    });

    it('should display sync status badges correctly', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 200));
      await wrapper.vm.$nextTick();

      const text = wrapper.text();
      // Check for either "Syncing" or "Paused" badge text
      expect(text.includes('Syncing') || text.includes('Paused')).toBe(true);
    });
  });

  describe('Add Account Dialog', () => {
    it('should open add account dialog when Add Account button is clicked', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      const buttons = wrapper.findAll('button');
      const addButton = buttons.find(b => b.text().includes('Add Account'));
      expect(addButton).toBeTruthy();

      await addButton!.trigger('click');
      await wrapper.vm.$nextTick();

      expect(wrapper.vm.currentView).toBe('add');
    });

    it('should validate required fields', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      // Navigate to add view
      wrapper.vm.navigateToAdd();
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      // Try to call addAccount directly with empty form
      await wrapper.vm.addAccount();
      await wrapper.vm.$nextTick();

      // Check that toast error was called (validation failed)
      expect(mockToast.error).toHaveBeenCalled();
      expect(mockInvoke).not.toHaveBeenCalledWith('add_account', expect.anything());
    });

    it('should validate email format', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      await wrapper.vm.$nextTick();

      // Set invalid email
      wrapper.vm.form.email = 'invalid-email';
      const isValid = wrapper.vm.validateForm();

      expect(isValid).toBe(false);
      expect(wrapper.vm.errors.email).toContain('email');
    });

    it('should validate port ranges', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      await wrapper.vm.$nextTick();

      // Set invalid port
      wrapper.vm.form.imap_port = 99999;
      const isValid = wrapper.vm.validateForm();

      expect(isValid).toBe(false);
      expect(wrapper.vm.errors.imap_port).toBeTruthy();
    });

    it('successfully add an account with valid data', async () => {
      const newAccount: Account = {
        id: 3,
        email: 'new@example.com',
        provider: 'Custom',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        sync_enabled: true,
      };

      mockInvoke
        .mockResolvedValueOnce(mockAccounts) // list_accounts on mount
        .mockResolvedValueOnce(newAccount) // add_account
        .mockResolvedValueOnce([...mockAccounts, newAccount]); // list_accounts after add

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToAdd();
      await wrapper.vm.$nextTick();

      wrapper.vm.form = {
        email: 'new@example.com',
        provider: 'Custom',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        password: 'password123',
      };

      await wrapper.vm.addAccount();
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      // Check that add_account was called with correct params (camelCase for IPC)
      const addAccountCall = mockInvoke.mock.calls.find(call => call[0] === 'add_account');
      expect(addAccountCall).toBeTruthy();
      expect(addAccountCall![1]).toEqual({
        email: 'new@example.com',
        provider: 'Custom',
        imapHost: 'imap.example.com',
        imapPort: 993,
        smtpHost: 'smtp.example.com',
        smtpPort: 587,
        password: 'password123',
      });

      expect(mockToast.success).toHaveBeenCalledWith(expect.stringContaining('new@example.com'));
    });
  });

  describe('Edit Account Dialog', () => {
    it('should open edit dialog with pre-filled data', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      const account = mockAccounts[0];
      wrapper.vm.navigateToEdit(account);
      await wrapper.vm.$nextTick();

      expect(wrapper.vm.form.email).toBe(account.email);
      expect(wrapper.vm.form.provider).toBe(account.provider);
      expect(wrapper.vm.form.imap_host).toBe(account.imap_host);
      expect(wrapper.vm.form.imap_port).toBe(account.imap_port);
      expect(wrapper.vm.form.smtp_host).toBe(account.smtp_host);
      expect(wrapper.vm.form.smtp_port).toBe(account.smtp_port);
      expect(wrapper.vm.form.password).toBe(''); // Should be empty for security
    });

    it('should successfully update an account', async () => {
      const updatedAccount: Account = {
        ...mockAccounts[0],
        email: 'updated@gmail.com',
        provider: 'Updated Gmail',
      };

      mockInvoke
        .mockResolvedValueOnce(mockAccounts) // list_accounts
        .mockResolvedValueOnce(updatedAccount) // update_account
        .mockResolvedValueOnce([updatedAccount, mockAccounts[1]]); // list_accounts after update

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToEdit(mockAccounts[0]);
      wrapper.vm.form.email = 'updated@gmail.com';
      wrapper.vm.form.provider = 'Updated Gmail';

      await wrapper.vm.updateAccount();
      await wrapper.vm.$nextTick();

      expect(mockInvoke).toHaveBeenCalledWith('update_account', {
        accountId: 1,
        email: 'updated@gmail.com',
        provider: 'Updated Gmail',
        imapHost: mockAccounts[0].imap_host,
        imapPort: mockAccounts[0].imap_port,
        smtpHost: mockAccounts[0].smtp_host,
        smtpPort: mockAccounts[0].smtp_port,
        password: undefined,
      });
      expect(mockToast.success).toHaveBeenCalled();
    });

    it('should update password only if provided', async () => {
      const updatedAccount: Account = { ...mockAccounts[0] };

      mockInvoke
        .mockResolvedValueOnce(mockAccounts)
        .mockResolvedValueOnce(updatedAccount)
        .mockResolvedValueOnce(mockAccounts);

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToEdit(mockAccounts[0]);
      wrapper.vm.form.password = 'newpassword123';

      await wrapper.vm.updateAccount();

      expect(mockInvoke).toHaveBeenCalledWith(
        'update_account',
        expect.objectContaining({
          password: 'newpassword123',
        })
      );
    });
  });

  describe('Delete Account', () => {
    it('should show confirmation dialog before deleting', async () => {
      const confirmSpy = vi.spyOn(window, 'confirm').mockReturnValue(false);

      mockInvoke.mockResolvedValue(mockAccounts);

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      await wrapper.vm.deleteAccount(mockAccounts[0]);

      expect(confirmSpy).toHaveBeenCalledWith(expect.stringContaining('test@gmail.com'));
      expect(mockInvoke).not.toHaveBeenCalledWith('delete_account');

      confirmSpy.mockRestore();
    });

    it('should delete account when confirmed', async () => {
      const confirmSpy = vi.spyOn(window, 'confirm').mockReturnValue(true);

      mockInvoke
        .mockResolvedValueOnce(mockAccounts) // list_accounts
        .mockResolvedValueOnce(undefined) // delete_account
        .mockResolvedValueOnce([mockAccounts[1]]); // list_accounts after delete

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      await wrapper.vm.deleteAccount(mockAccounts[0]);
      await wrapper.vm.$nextTick();

      expect(mockInvoke).toHaveBeenCalledWith('delete_account', { accountId: 1 });
      expect(mockToast.success).toHaveBeenCalledWith(expect.stringContaining('test@gmail.com'));

      confirmSpy.mockRestore();
    });
  });

  describe('Connection Test', () => {
    it('should test connection with provided credentials', async () => {
      const testResult: TestConnectionResult = {
        imap_success: true,
        smtp_success: true,
        imap_error: null,
        smtp_error: null,
      };

      mockInvoke
        .mockResolvedValueOnce(mockAccounts) // list_accounts
        .mockResolvedValueOnce(testResult); // test_account_connection

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100)); // Wait for list_accounts

      wrapper.vm.navigateToAdd();
      wrapper.vm.form = {
        email: 'test@example.com',
        provider: 'Test',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        password: 'password123',
      };

      await wrapper.vm.testConnection();
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100)); // Wait for test_account_connection

      expect(mockInvoke).toHaveBeenCalledWith('test_account_connection', {
        imapHost: 'imap.example.com',
        imapPort: 993,
        smtpHost: 'smtp.example.com',
        smtpPort: 587,
        email: 'test@example.com',
        password: 'password123',
      });
      expect(mockToast.success).toHaveBeenCalledWith('Connection test successful!');
    });

    it('should display error when connection test fails', async () => {
      const testResult: TestConnectionResult = {
        imap_success: false,
        smtp_success: true,
        imap_error: 'Connection timeout',
        smtp_error: null,
      };

      mockInvoke.mockResolvedValueOnce(mockAccounts).mockResolvedValueOnce(testResult);

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100)); // Wait for list_accounts

      wrapper.vm.navigateToAdd();
      wrapper.vm.form = {
        email: 'test@example.com',
        provider: 'Test',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        password: 'password123',
      };

      await wrapper.vm.testConnection();
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100)); // Wait for test_account_connection

      expect(wrapper.vm.testResult).toEqual(testResult);
      expect(mockToast.error).toHaveBeenCalledWith('Connection test failed - see details below');
    });

    it('should not test connection with invalid form data', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      wrapper.vm.form.email = 'invalid'; // Invalid email

      await wrapper.vm.testConnection();

      expect(mockInvoke).not.toHaveBeenCalledWith('test_account_connection');
      expect(mockToast.error).toHaveBeenCalledWith('Please fix validation errors before testing');
    });
  });

  describe('Keyboard Navigation', () => {
    it('should navigate back to list on Escape key when in add view', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      expect(wrapper.vm.currentView).toBe('add');

      const escapeEvent = new KeyboardEvent('keydown', { key: 'Escape' });
      window.dispatchEvent(escapeEvent);
      await wrapper.vm.$nextTick();

      expect(wrapper.vm.currentView).toBe('list');
    });

    it('should emit close event on Escape when in list view', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      expect(wrapper.vm.currentView).toBe('list');

      const escapeEvent = new KeyboardEvent('keydown', { key: 'Escape' });
      window.dispatchEvent(escapeEvent);
      await wrapper.vm.$nextTick();

      expect(wrapper.emitted('close')).toBeTruthy();
    });
  });

  describe('Error Handling', () => {
    it('should handle list_accounts error gracefully', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('Failed to load accounts'));

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 150));

      expect(mockToast.error).toHaveBeenCalledWith('Failed to load accounts');

      // Ensure wrapper accounts is empty after error
      expect(wrapper.vm.accounts).toEqual([]);
    });

    it('should handle add_account error gracefully', async () => {
      mockInvoke
        .mockResolvedValueOnce(mockAccounts) // list_accounts on mount
        .mockRejectedValueOnce(new Error('Database error')); // add_account fails

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToAdd();
      wrapper.vm.form = {
        email: 'test@example.com',
        provider: 'Test',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        password: 'password123',
      };

      await wrapper.vm.addAccount();
      await wrapper.vm.$nextTick();

      // Should show error toast
      expect(mockToast.error).toHaveBeenCalled();
    });

    it('should handle connection test error gracefully', async () => {
      mockInvoke
        .mockResolvedValueOnce(mockAccounts) // list_accounts on mount
        .mockRejectedValueOnce(new Error('Network error')); // test_account_connection fails

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToAdd();
      wrapper.vm.form = {
        email: 'test@example.com',
        provider: 'Test',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        password: 'password123',
      };

      await wrapper.vm.testConnection();
      await wrapper.vm.$nextTick();

      // Should show error toast
      expect(mockToast.error).toHaveBeenCalled();
    });
  });

  describe('Form Reset', () => {
    it('should reset form when navigating to list', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      wrapper.vm.form.email = 'test@example.com';
      wrapper.vm.form.provider = 'Test Provider';

      wrapper.vm.navigateToList();
      await wrapper.vm.$nextTick();

      expect(wrapper.vm.form.email).toBe('');
      expect(wrapper.vm.form.provider).toBe('');
      expect(wrapper.vm.errors).toEqual({});
    });

    it('should clear test results when navigating to add', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.testResult = {
        imap_success: true,
        smtp_success: true,
        imap_error: null,
        smtp_error: null,
      };

      wrapper.vm.navigateToAdd();
      await wrapper.vm.$nextTick();

      expect(wrapper.vm.testResult).toBeNull();
    });
  });
});
