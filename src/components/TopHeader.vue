<script setup lang="ts">
import { Moon, Sun } from 'lucide-vue-next';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { SidebarTrigger } from '@/components/ui/sidebar';

const props = defineProps<{
  emailCount: number;
  lastUpdateText: string;
  theme: 'light' | 'dark';
}>();

const emit = defineEmits<{
  toggleTheme: [];
}>();
</script>

<template>
  <header
    class="flex h-14 items-center gap-3 border-b bg-background px-3 md:px-4"
    data-testid="top-header"
  >
    <SidebarTrigger class="-ml-1" />

    <div class="flex items-center gap-2">
      <div class="flex items-center gap-2">
        <span
          class="inline-flex h-8 w-8 items-center justify-center rounded-md bg-muted text-foreground"
        >
          <span class="text-sm font-semibold">✉</span>
        </span>
        <div class="flex items-center gap-2">
          <span class="text-sm font-normal tracking-tight md:text-base">Emails</span>
          <Badge variant="secondary" class="h-6 rounded-md px-2">
            {{ props.emailCount }} emails
          </Badge>
        </div>
      </div>
    </div>

    <div class="ml-auto flex items-center gap-2">
      <span class="hidden text-xs text-muted-foreground md:inline"
        >Last update {{ props.lastUpdateText }}</span
      >

      <div class="hidden items-center -space-x-2 sm:flex">
        <Avatar class="h-7 w-7 border border-background"><AvatarFallback>A</AvatarFallback></Avatar>
        <Avatar class="h-7 w-7 border border-background"><AvatarFallback>B</AvatarFallback></Avatar>
        <Avatar class="h-7 w-7 border border-background"><AvatarFallback>C</AvatarFallback></Avatar>
        <Avatar class="h-7 w-7 border border-background"><AvatarFallback>D</AvatarFallback></Avatar>
      </div>

      <Button variant="ghost" size="icon" aria-label="Toggle theme" @click="emit('toggleTheme')">
        <Sun v-if="props.theme === 'dark'" class="size-4" />
        <Moon v-else class="size-4" />
      </Button>

      <Button variant="outline" class="hidden h-8 px-3 md:inline-flex">GitHub</Button>

      <Button
        class="hidden h-8 bg-linear-to-r from-purple-500 to-blue-500 px-3 text-white hover:opacity-90 md:inline-flex"
      >
        Upgrade to Pro
      </Button>
    </div>
  </header>
</template>
