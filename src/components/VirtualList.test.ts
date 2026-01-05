import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import VirtualList from './VirtualList.vue';

describe('VirtualList', () => {
  it('renders with default props', () => {
    const items = [{ id: 1 }, { id: 2 }, { id: 3 }];

    const wrapper = mount(VirtualList, {
      props: {
        items,
      },
      slots: {
        default: '{{ item }}',
      },
    });

    expect(wrapper.find('.virtual-list-container').exists()).toBe(true);
    expect(wrapper.find('.virtual-list-wrapper').exists()).toBe(true);
  });

  it('accepts custom item height', () => {
    const items = [{ id: 1 }];
    const customHeight = 100;

    const wrapper = mount(VirtualList, {
      props: {
        items,
        itemHeight: customHeight,
      },
      slots: {
        default: '{{ item }}',
      },
    });

    expect(wrapper.props('itemHeight')).toBe(customHeight);
  });

  it('accepts custom container height', () => {
    const items = [{ id: 1 }];
    const customContainerHeight = '600px';

    const wrapper = mount(VirtualList, {
      props: {
        items,
        containerHeight: customContainerHeight,
      },
      slots: {
        default: '{{ item }}',
      },
    });

    const container = wrapper.find('.virtual-list-container');
    expect(container.attributes('style')).toContain('height: 600px');
  });

  it('handles large datasets', () => {
    const items = Array.from({ length: 10000 }, (_, i) => ({ id: i, name: `Item ${i}` }));

    const wrapper = mount(VirtualList, {
      props: {
        items,
      },
      slots: {
        default: '{{ item.name }}',
      },
    });

    // Virtualizer should only render visible items, not all 10000
    const renderedItems = wrapper.findAll('.virtual-list-item');
    expect(renderedItems.length).toBeLessThan(100);
    expect(wrapper.find('.virtual-list-container').exists()).toBe(true);
  });
});
