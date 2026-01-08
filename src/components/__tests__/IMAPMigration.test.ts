/**
 * IMAP Migration Frontend Tests (Task Group 5.1)
 *
 * Focused tests for IMAP migration frontend updates:
 * - AccountManagement form with imap_host/imap_port
 * - Form validation for IMAP fields
 * - Connection test with IMAP credentials
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount, VueWrapper } from '@vue/test-utils';
import AccountManagement from '@/components/AccountManagement.vue';
import type { Account, TestConnectionResult } from '@/types/commands';

// Mock Tauri invoke
const mockInvoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

// Mock vue-sonner
vi.mock('vue-sonner', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
    info: vi.fn(),
  },
}));

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

describe('IMAP Migration - Frontend Updates', () => {
  describe('AccountManagement - IMAP Fields', () => {
    let wrapper: VueWrapper<any>;

    beforeEach(() => {
      vi.clearAllMocks();
      mockInvoke.mockResolvedValue([]);
    });

    afterEach(() => {
      if (wrapper) {
        wrapper.unmount();
      }
    });

    it('should have imap_host and imap_port fields in form', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      await wrapper.vm.$nextTick();

      // Check that form has imap fields
      expect(wrapper.vm.form).toHaveProperty('imap_host');
      expect(wrapper.vm.form).toHaveProperty('imap_port');

      // Ensure it does NOT have old pop3 fields
      expect(wrapper.vm.form).not.toHaveProperty('pop3_host');
      expect(wrapper.vm.form).not.toHaveProperty('pop3_port');
    });

    it('should default IMAP port to 993', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      await wrapper.vm.$nextTick();

      expect(wrapper.vm.form.imap_port).toBe(993);
    });

    it('should validate IMAP host as required', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();
      wrapper.vm.form.imap_host = '';

      const isValid = wrapper.vm.validateForm();

      expect(isValid).toBe(false);
      expect(wrapper.vm.errors.imap_host).toContain('IMAP');
    });

    it('should validate IMAP port range (1-65535)', async () => {
      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();

      wrapper.vm.navigateToAdd();

      // Test invalid port (too high)
      wrapper.vm.form.imap_port = 99999;
      let isValid = wrapper.vm.validateForm();
      expect(isValid).toBe(false);
      expect(wrapper.vm.errors.imap_port).toBeTruthy();

      // Test invalid port (too low)
      wrapper.vm.form.imap_port = 0;
      isValid = wrapper.vm.validateForm();
      expect(isValid).toBe(false);
      expect(wrapper.vm.errors.imap_port).toBeTruthy();
    });

    it('should send IMAP credentials to backend on connection test', async () => {
      const testResult: TestConnectionResult = {
        imap_success: true,
        smtp_success: true,
        imap_error: null,
        smtp_error: null,
      };

      mockInvoke
        .mockResolvedValueOnce([]) // list_accounts
        .mockResolvedValueOnce(testResult); // test_account_connection

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToAdd();
      wrapper.vm.form = {
        email: 'test@imap.com',
        provider: 'IMAP',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        password: 'password',
      };

      await wrapper.vm.testConnection();
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      // Verify IMAP fields were sent (not pop3)
      expect(mockInvoke).toHaveBeenCalledWith('test_account_connection', {
        imapHost: 'imap.example.com',
        imapPort: 993,
        smtpHost: 'smtp.example.com',
        smtpPort: 587,
        email: 'test@imap.com',
        password: 'password',
      });
    });

    it('should display IMAP success/error in test results', async () => {
      const testResult: TestConnectionResult = {
        imap_success: true,
        smtp_success: false,
        imap_error: null,
        smtp_error: 'SMTP connection failed',
      };

      mockInvoke.mockResolvedValueOnce([]).mockResolvedValueOnce(testResult);

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToAdd();
      wrapper.vm.form = {
        email: 'test@imap.com',
        provider: 'IMAP',
        imap_host: 'imap.example.com',
        imap_port: 993,
        smtp_host: 'smtp.example.com',
        smtp_port: 587,
        password: 'password',
      };

      await wrapper.vm.testConnection();
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      // Verify result has imap_success/imap_error (not pop3)
      expect(wrapper.vm.testResult).toBeTruthy();
      expect(wrapper.vm.testResult.imap_success).toBe(true);
      expect(wrapper.vm.testResult.imap_error).toBeNull();
      expect(wrapper.vm.testResult.smtp_success).toBe(false);
      expect(wrapper.vm.testResult.smtp_error).toBe('SMTP connection failed');
    });

    it('should call add_account with imap_host and imap_port', async () => {
      const newAccount: Account = {
        id: 1,
        email: 'new@example.com',
        provider: 'Gmail',
        imap_host: 'imap.gmail.com',
        imap_port: 993,
        smtp_host: 'smtp.gmail.com',
        smtp_port: 587,
        sync_enabled: true,
      };

      mockInvoke
        .mockResolvedValueOnce([]) // list_accounts
        .mockResolvedValueOnce(newAccount) // add_account
        .mockResolvedValueOnce([newAccount]); // list_accounts after add

      wrapper = mount(AccountManagement, mountOptions);
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      wrapper.vm.navigateToAdd();
      wrapper.vm.form = {
        email: 'new@example.com',
        provider: 'Gmail',
        imap_host: 'imap.gmail.com',
        imap_port: 993,
        smtp_host: 'smtp.gmail.com',
        smtp_port: 587,
        password: 'password123',
      };

      await wrapper.vm.addAccount();
      await wrapper.vm.$nextTick();
      await new Promise(resolve => setTimeout(resolve, 100));

      const addCall = mockInvoke.mock.calls.find(call => call[0] === 'add_account');
      expect(addCall).toBeTruthy();
      expect(addCall![1]).toEqual({
        email: 'new@example.com',
        provider: 'Gmail',
        imapHost: 'imap.gmail.com',
        imapPort: 993,
        smtpHost: 'smtp.gmail.com',
        smtpPort: 587,
        password: 'password123',
      });
    });
  });
});
