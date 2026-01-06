import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import SearchBar from '@/components/SearchBar.vue';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

describe('SearchBar', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders search input', () => {
    const wrapper = mount(SearchBar);

    const input = wrapper.find('input[type="search"]');
    expect(input.exists()).toBe(true);
  });

  it('accepts user input', async () => {
    const wrapper = mount(SearchBar);

    const input = wrapper.find('input[type="search"]');
    await input.setValue('test query');

    expect((input.element as HTMLInputElement).value).toBe('test query');
  });

  it('emits clear event when clear button clicked', async () => {
    const wrapper = mount(SearchBar);

    const input = wrapper.find('input[type="search"]');
    await input.setValue('test query');
    await wrapper.vm.$nextTick();

    const clearButton = wrapper
      .findAll('button')
      .find(b => b.attributes('title')?.includes('Clear'));
    if (clearButton) {
      await clearButton.trigger('click');
      expect(wrapper.emitted('clear')).toBeTruthy();
    }
  });

  it('can perform search', async () => {
    vi.useFakeTimers();
    vi.mocked(invoke).mockResolvedValue([]);

    const wrapper = mount(SearchBar);

    const input = wrapper.find('input[type="search"]');
    await input.setValue('test');

    // Fast-forward debounce time
    vi.advanceTimersByTime(300);
    await wrapper.vm.$nextTick();

    await vi.waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('search_messages', expect.any(Object));
    });

    vi.useRealTimers();
  });
});
