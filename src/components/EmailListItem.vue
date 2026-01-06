<script setup lang="ts">
import { computed } from 'vue';
import { BadgeCheck } from 'lucide-vue-next';
import { cn } from '@/lib/utils';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Checkbox } from '@/components/ui/checkbox';

interface Props {
  id: number;
  subject?: string;
  from: string;
  fromEmail?: string;
  preview: string;
  date?: string;
  isRead: boolean;
  isStarred: boolean;
  hasAttachments: boolean;
  selected?: boolean;
  checked?: boolean;
  isVerified?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  isVerified: false,
  checked: false,
});

const emit = defineEmits<{
  click: [id: number, event?: MouseEvent];
  star: [id: number];
  check: [id: number, checked: boolean];
}>();

function handleClick(event: MouseEvent) {
  emit('click', props.id, event);
}

function handleCheck(checked: boolean | 'indeterminate') {
  if (checked === 'indeterminate') return;
  emit('check', props.id, checked);
}

const initials = computed(() => {
  if (!props.from) return '?';
  return props.from
    .split(' ')
    .map(n => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);
});

const formattedDate = computed(() => {
  if (!props.date) return '';
  const date = new Date(props.date);
  const now = new Date();
  const diffDays = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24));

  if (diffDays === 0) {
    return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
  } else if (diffDays === 1) {
    return 'Yesterday';
  } else if (diffDays < 7) {
    return date.toLocaleDateString('en-US', { weekday: 'short' });
  } else {
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  }
});

const avatarColors = [
  'bg-red-100 text-red-600',
  'bg-blue-100 text-blue-600',
  'bg-green-100 text-green-600',
  'bg-yellow-100 text-yellow-600',
  'bg-purple-100 text-purple-600',
  'bg-pink-100 text-pink-600',
  'bg-indigo-100 text-indigo-600',
  'bg-orange-100 text-orange-600',
];

const avatarColor = computed(() => {
  const charCode = props.from.charCodeAt(0) || 0;
  return avatarColors[charCode % avatarColors.length];
});
</script>

<template>
  <div
    data-testid="email-list-item"
    :data-message-id="id"
    :class="
      cn(
        'group flex cursor-pointer items-start gap-3 border-b px-4 py-3 transition-colors',
        'hover:bg-accent/50',
        selected && 'bg-accent',
        !isRead && 'bg-accent/30'
      )
    "
    @click="handleClick"
  >
    <div class="flex items-center pt-0.5" @click.stop>
      <Checkbox
        :model-value="checked"
        @update:model-value="handleCheck"
        class="data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground"
      />
    </div>

    <Avatar :class="cn('h-9 w-9 shrink-0', avatarColor)">
      <AvatarImage :src="``" :alt="from" />
      <AvatarFallback :class="avatarColor">{{ initials }}</AvatarFallback>
    </Avatar>

    <div class="flex min-w-0 flex-1 flex-col gap-1">
      <div class="flex items-baseline justify-between gap-2">
        <div class="flex min-w-0 items-center gap-1.5">
          <span :class="cn('truncate text-sm', !isRead ? 'font-semibold' : 'font-medium')">
            {{ from }}
          </span>
          <BadgeCheck
            v-if="isVerified"
            :size="14"
            class="shrink-0 fill-cyan-500 text-white"
            aria-label="Verified"
          />
        </div>
        <span class="shrink-0 text-xs text-muted-foreground">{{ formattedDate }}</span>
      </div>

      <p v-if="fromEmail" class="truncate text-xs text-cyan-500">
        {{ fromEmail }}
      </p>

      <p :class="cn('truncate text-sm', !isRead ? 'font-medium' : 'font-normal')">
        {{ subject || '(no subject)' }}
      </p>

      <p class="line-clamp-2 text-xs leading-relaxed text-muted-foreground">
        {{ preview }}
      </p>
    </div>
  </div>
</template>
