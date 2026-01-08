import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import AttachmentUpload from '@/components/AttachmentUpload.vue';

describe('AttachmentUpload', () => {
  it('renders drag-and-drop zone', () => {
    const wrapper = mount(AttachmentUpload);

    expect(wrapper.text()).toContain('Click to upload');
    expect(wrapper.text()).toContain('or drag and drop');
  });

  it('shows empty state initially', () => {
    const wrapper = mount(AttachmentUpload);

    expect(wrapper.text()).not.toContain('attached');
    expect(wrapper.findAll('.flex.items-center.justify-between.rounded-md').length).toBe(0);
  });

  it('emits update:attachments when files are added', async () => {
    const wrapper = mount(AttachmentUpload);

    // Create a mock file
    const file = new File(['test content'], 'test.txt', { type: 'text/plain' });

    // Mock FileReader
    const mockFileReader = {
      readAsDataURL: vi.fn(function (this: any) {
        this.onload({ target: { result: 'data:text/plain;base64,dGVzdCBjb250ZW50' } });
      }),
      result: 'data:text/plain;base64,dGVzdCBjb250ZW50',
    };

    vi.spyOn(window, 'FileReader').mockImplementation(() => mockFileReader as any);

    // Trigger file input change
    const input = wrapper.find('input[type="file"]');
    Object.defineProperty(input.element, 'files', {
      value: [file],
      writable: false,
    });

    await input.trigger('change');
    await wrapper.vm.$nextTick();

    // Wait for async file reading
    await new Promise(resolve => setTimeout(resolve, 100));

    const emitted = wrapper.emitted('update:attachments');
    expect(emitted).toBeTruthy();
  });

  it('displays file information after upload', async () => {
    const wrapper = mount(AttachmentUpload);

    // Simulate adding an attachment directly to component state
    const mockAttachment = {
      id: '123',
      name: 'document.pdf',
      size: 1024,
      type: 'application/pdf',
      data: 'base64data',
    };

    await wrapper.vm.$nextTick();

    // Manually set attachments for testing display
    wrapper.vm.attachments = [mockAttachment];
    await wrapper.vm.$nextTick();

    expect(wrapper.text()).toContain('1 file attached');
    expect(wrapper.text()).toContain('document.pdf');
  });

  it('formats file sizes correctly', () => {
    const wrapper = mount(AttachmentUpload);
    const vm = wrapper.vm as any;

    expect(vm.formatFileSize(0)).toBe('0 Bytes');
    expect(vm.formatFileSize(1024)).toBe('1 KB');
    expect(vm.formatFileSize(1048576)).toBe('1 MB');
    expect(vm.formatFileSize(1073741824)).toBe('1 GB');
  });

  it('removes attachment when remove button is clicked', async () => {
    const wrapper = mount(AttachmentUpload);

    // Add mock attachment
    const mockAttachment = {
      id: '123',
      name: 'test.txt',
      size: 500,
      type: 'text/plain',
      data: 'base64data',
    };

    wrapper.vm.attachments = [mockAttachment];
    await wrapper.vm.$nextTick();

    // Find and click remove button
    const removeButton = wrapper.find('button');
    await removeButton.trigger('click');

    // Check that update:attachments was emitted with empty array
    const emitted = wrapper.emitted('update:attachments');
    expect(emitted).toBeTruthy();
    if (emitted) {
      const lastEmit = emitted[emitted.length - 1];
      expect(lastEmit[0]).toEqual([]);
    }
  });

  it('displays multiple files correctly', async () => {
    const wrapper = mount(AttachmentUpload);

    const mockAttachments = [
      { id: '1', name: 'file1.txt', size: 100, type: 'text/plain', data: 'data1' },
      { id: '2', name: 'file2.pdf', size: 200, type: 'application/pdf', data: 'data2' },
      { id: '3', name: 'file3.jpg', size: 300, type: 'image/jpeg', data: 'data3' },
    ];

    wrapper.vm.attachments = mockAttachments;
    await wrapper.vm.$nextTick();

    expect(wrapper.text()).toContain('3 files attached');
    expect(wrapper.text()).toContain('file1.txt');
    expect(wrapper.text()).toContain('file2.pdf');
    expect(wrapper.text()).toContain('file3.jpg');
  });

  it('shows total size of all attachments', async () => {
    const wrapper = mount(AttachmentUpload);

    const mockAttachments = [
      { id: '1', name: 'file1.txt', size: 512, type: 'text/plain', data: 'data1' },
      { id: '2', name: 'file2.txt', size: 512, type: 'text/plain', data: 'data2' },
    ];

    wrapper.vm.attachments = mockAttachments;
    await wrapper.vm.$nextTick();

    expect(wrapper.text()).toContain('Total:');
    expect(wrapper.text()).toContain('1 KB'); // 512 + 512 = 1024 bytes = 1 KB
  });
});
