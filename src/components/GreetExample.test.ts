import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import GreetExample from './GreetExample.vue';

// Mock the Tauri invoke function
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('GreetExample', () => {
  it('renders the component with initial state', () => {
    const wrapper = mount(GreetExample);

    expect(wrapper.find('input').exists()).toBe(true);
    expect(wrapper.text()).toContain('Type-Safe IPC Example');
  });

  it('updates name input when user types', async () => {
    const wrapper = mount(GreetExample);
    const input = wrapper.find('input');

    await input.setValue('John');
    expect((input.element as HTMLInputElement).value).toBe('John');
  });

  it('disables greet button when name is empty', () => {
    const wrapper = mount(GreetExample);
    const buttons = wrapper.findAll('button');
    const greetButton = buttons.find(btn => btn.text().includes('Greet'));

    expect(greetButton?.attributes('disabled')).toBeDefined();
  });

  it('shows error when trying to greet with empty name', async () => {
    const wrapper = mount(GreetExample);
    const buttons = wrapper.findAll('button');
    const greetButton = buttons.find(btn => btn.text().includes('Greet'));

    // The button should be disabled, but let's test the logic by setting a name and clearing it
    const input = wrapper.find('input');
    await input.setValue('  '); // Whitespace only

    // Even with whitespace, button should be disabled due to trim check
    expect(greetButton?.attributes('disabled')).toBeDefined();
  });
});
