<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Link from '@tiptap/extension-link';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { SidebarTrigger } from '@/components/ui/sidebar';
import { stripHtml } from '@/lib/html-utils';

interface Props {
  accountId: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  close: [];
  sent: [emailId: number];
}>();

const to = ref('');
const cc = ref('');
const bcc = ref('');
const subject = ref('');
const isSending = ref(false);
const error = ref<string | null>(null);

// Initialize Tiptap editor
const editor = useEditor({
  extensions: [
    StarterKit,
    Link.configure({
      openOnClick: false,
    }),
  ],
  content: '',
  editorProps: {
    attributes: {
      class:
        'prose prose-sm max-w-none min-h-[300px] p-4 border border-border rounded-md focus:outline-none',
    },
  },
});

function parseEmails(emailStr: string): string[] {
  return emailStr
    .split(',')
    .map(e => e.trim())
    .filter(e => e.length > 0);
}

async function sendEmail() {
  if (!to.value.trim()) {
    error.value = 'Please enter at least one recipient';
    return;
  }

  if (!editor.value) {
    error.value = 'Editor not initialized';
    return;
  }

  try {
    isSending.value = true;
    error.value = null;

    const bodyHtml = editor.value.getHTML();
    const bodyPlain = stripHtml(bodyHtml);

    const emailId = await invoke<number>('send_email', {
      params: {
        accountId: props.accountId,
        to: parseEmails(to.value),
        cc: parseEmails(cc.value),
        bcc: parseEmails(bcc.value),
        subject: subject.value,
        bodyPlain,
        bodyHtml,
      },
    });

    emit('sent', emailId);
    resetForm();
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to send email';
  } finally {
    isSending.value = false;
  }
}

function resetForm() {
  to.value = '';
  cc.value = '';
  bcc.value = '';
  subject.value = '';
  editor.value?.commands.clearContent();
}

function cancel() {
  emit('close');
}
</script>

<template>
  <div class="flex h-full flex-col bg-background">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border px-4 py-3">
      <div class="flex items-center gap-2">
        <SidebarTrigger class="-ml-1" />
        <h2 class="text-lg font-semibold">New Message</h2>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" :disabled="isSending" @click="cancel">Cancel</Button>
        <Button :disabled="isSending" @click="sendEmail">
          {{ isSending ? 'Sending...' : 'Send' }}
        </Button>
      </div>
    </div>

    <!-- Email Form -->
    <div class="flex-1 overflow-y-auto p-4">
      <div class="space-y-4">
        <!-- Error Message -->
        <div v-if="error" class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
          {{ error }}
        </div>

        <!-- Recipients -->
        <div class="space-y-2">
          <div class="flex gap-2">
            <span class="w-12 pt-2 text-sm text-muted-foreground">To:</span>
            <Input
              v-model="to"
              type="email"
              placeholder="recipient@example.com, another@example.com"
              class="flex-1"
            />
          </div>
          <div class="flex gap-2">
            <span class="w-12 pt-2 text-sm text-muted-foreground">Cc:</span>
            <Input v-model="cc" type="email" placeholder="Optional" class="flex-1" />
          </div>
          <div class="flex gap-2">
            <span class="w-12 pt-2 text-sm text-muted-foreground">Bcc:</span>
            <Input v-model="bcc" type="email" placeholder="Optional" class="flex-1" />
          </div>
        </div>

        <!-- Subject -->
        <div class="flex gap-2">
          <span class="w-12 pt-2 text-sm text-muted-foreground">Subject:</span>
          <Input v-model="subject" placeholder="Email subject" class="flex-1" />
        </div>

        <!-- Toolbar -->
        <div v-if="editor" class="flex flex-wrap gap-1 border-b border-border pb-2">
          <Button
            variant="ghost"
            size="sm"
            :class="editor.isActive('bold') && 'bg-accent'"
            @click="editor.chain().focus().toggleBold().run()"
          >
            <strong>B</strong>
          </Button>
          <Button
            variant="ghost"
            size="sm"
            :class="editor.isActive('italic') && 'bg-accent'"
            @click="editor.chain().focus().toggleItalic().run()"
          >
            <em>I</em>
          </Button>
          <Button
            variant="ghost"
            size="sm"
            :class="editor.isActive('bulletList') && 'bg-accent'"
            @click="editor.chain().focus().toggleBulletList().run()"
          >
            • List
          </Button>
          <Button
            variant="ghost"
            size="sm"
            :class="editor.isActive('orderedList') && 'bg-accent'"
            @click="editor.chain().focus().toggleOrderedList().run()"
          >
            1. List
          </Button>
        </div>

        <!-- Editor -->
        <EditorContent :editor="editor" />
      </div>
    </div>
  </div>
</template>
