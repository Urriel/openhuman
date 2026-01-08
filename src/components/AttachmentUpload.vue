<script setup lang="ts">
import { ref, computed } from 'vue';
import { Button } from '@/components/ui/button';
import { Paperclip, X } from 'lucide-vue-next';

interface AttachmentFile {
  id: string;
  name: string;
  size: number;
  type: string;
  data: string; // base64 encoded data
}

const emit = defineEmits<{
  'update:attachments': [attachments: AttachmentFile[]];
}>();

const attachments = ref<AttachmentFile[]>([]);
const isDragging = ref(false);
const fileInputRef = ref<HTMLInputElement | null>(null);

// Format file size for display
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

// Convert File to base64
async function fileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const result = reader.result as string;
      // Extract base64 data (remove data URL prefix)
      const base64 = result.split(',')[1];
      resolve(base64);
    };
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
}

// Add files to attachment list
async function addFiles(files: FileList | File[]) {
  const fileArray = Array.from(files);

  for (const file of fileArray) {
    // Check if file already exists
    const exists = attachments.value.some(a => a.name === file.name && a.size === file.size);
    if (exists) continue;

    try {
      const base64Data = await fileToBase64(file);

      const attachment: AttachmentFile = {
        id: crypto.randomUUID(),
        name: file.name,
        size: file.size,
        type: file.type || 'application/octet-stream',
        data: base64Data,
      };

      attachments.value.push(attachment);
    } catch (error) {
      console.error('Failed to read file:', file.name, error);
    }
  }

  // Emit updated attachments
  emit('update:attachments', attachments.value);
}

// Handle file input change
function handleFileInput(event: Event) {
  const input = event.target as HTMLInputElement;
  if (input.files && input.files.length > 0) {
    addFiles(input.files);
    // Reset input to allow selecting the same file again
    input.value = '';
  }
}

// Handle drag events
function handleDragEnter(event: DragEvent) {
  event.preventDefault();
  isDragging.value = true;
}

function handleDragOver(event: DragEvent) {
  event.preventDefault();
  isDragging.value = true;
}

function handleDragLeave(event: DragEvent) {
  event.preventDefault();
  // Only set to false if we're leaving the drop zone itself
  const target = event.currentTarget as HTMLElement;
  const related = event.relatedTarget as HTMLElement;
  if (!target.contains(related)) {
    isDragging.value = false;
  }
}

function handleDrop(event: DragEvent) {
  event.preventDefault();
  isDragging.value = false;

  const files = event.dataTransfer?.files;
  if (files && files.length > 0) {
    addFiles(files);
  }
}

// Remove attachment
function removeAttachment(id: string) {
  attachments.value = attachments.value.filter(a => a.id !== id);
  emit('update:attachments', attachments.value);
}

// Trigger file input click
function triggerFileInput() {
  fileInputRef.value?.click();
}

// Total size of all attachments
const totalSize = computed(() => {
  return attachments.value.reduce((sum, file) => sum + file.size, 0);
});
</script>

<template>
  <div class="space-y-3">
    <!-- Hidden file input -->
    <input ref="fileInputRef" type="file" multiple class="hidden" @change="handleFileInput" />

    <!-- Drag and drop zone -->
    <div
      :class="[
        'border-2 border-dashed rounded-lg p-6 text-center transition-colors cursor-pointer',
        isDragging
          ? 'border-primary bg-primary/5'
          : 'border-border hover:border-primary/50 hover:bg-muted/30',
      ]"
      @dragenter="handleDragEnter"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
      @click="triggerFileInput"
    >
      <div class="flex flex-col items-center gap-2">
        <Paperclip :size="24" class="text-muted-foreground" />
        <div class="text-sm">
          <span class="font-medium text-foreground">Click to upload</span>
          <span class="text-muted-foreground"> or drag and drop</span>
        </div>
        <div class="text-xs text-muted-foreground">Any file type supported</div>
      </div>
    </div>

    <!-- Attachments list -->
    <div v-if="attachments.length > 0" class="space-y-2">
      <div class="flex items-center justify-between text-sm">
        <span class="font-medium text-foreground">
          {{ attachments.length }} file{{ attachments.length !== 1 ? 's' : '' }} attached
        </span>
        <span class="text-xs text-muted-foreground"> Total: {{ formatFileSize(totalSize) }} </span>
      </div>

      <div class="space-y-1">
        <div
          v-for="attachment in attachments"
          :key="attachment.id"
          class="flex items-center justify-between rounded-md border border-border bg-muted/30 px-3 py-2 text-sm"
        >
          <div class="flex items-center gap-2 min-w-0 flex-1">
            <Paperclip :size="14" class="text-muted-foreground flex-shrink-0" />
            <span class="truncate font-medium text-foreground">{{ attachment.name }}</span>
            <span class="text-xs text-muted-foreground flex-shrink-0">
              {{ formatFileSize(attachment.size) }}
            </span>
          </div>
          <Button
            variant="ghost"
            size="icon"
            class="h-6 w-6 flex-shrink-0"
            @click.stop="removeAttachment(attachment.id)"
          >
            <X :size="14" />
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
