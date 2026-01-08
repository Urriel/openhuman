<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { toast } from 'vue-sonner';
import {
  invokeAddAccount,
  invokeListAccounts,
  invokeUpdateAccount,
  invokeRemoveAccount,
  invokeTestAccountConnection,
  invokeSyncEmails,
  type Account,
  type AddAccountRequest,
  type UpdateAccountRequest,
  type TestConnectionRequest,
  type TestConnectionResult,
} from '@/types/commands';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { Badge } from '@/components/ui/badge';
import { SidebarTrigger } from '@/components/ui/sidebar';

// View state: 'list' | 'add' | 'edit'
type ViewMode = 'list' | 'add' | 'edit';
const currentView = ref<ViewMode>('list');

// Component state
const accounts = ref<Account[]>([]);
const isLoading = ref(false);
const selectedAccount = ref<Account | null>(null);

// Form state
const form = ref({
  email: '',
  provider: '',
  imap_host: '',
  imap_port: 993,
  smtp_host: '',
  smtp_port: 587,
  password: '',
});

// Validation errors
const errors = ref<Record<string, string>>({});

// Connection test state
const testingConnection = ref(false);
const testResult = ref<TestConnectionResult | null>(null);

// Emit close event
const emit = defineEmits<{
  close: [];
}>();

// Load accounts on mount
onMounted(async () => {
  await loadAccounts();
});

// Load accounts from backend
async function loadAccounts() {
  try {
    isLoading.value = true;
    accounts.value = await invokeListAccounts();
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Failed to load accounts';
    toast.error(message);
  } finally {
    isLoading.value = false;
  }
}

// Validate form
function validateForm(): boolean {
  errors.value = {};

  if (!form.value.email || !form.value.email.includes('@')) {
    errors.value.email = 'Valid email address is required';
  }

  if (!form.value.provider.trim()) {
    errors.value.provider = 'Provider is required';
  }

  if (!form.value.imap_host.trim()) {
    errors.value.imap_host = 'IMAP host is required';
  }

  if (form.value.imap_port < 1 || form.value.imap_port > 65535) {
    errors.value.imap_port = 'Valid port (1-65535) is required';
  }

  if (!form.value.smtp_host.trim()) {
    errors.value.smtp_host = 'SMTP host is required';
  }

  if (form.value.smtp_port < 1 || form.value.smtp_port > 65535) {
    errors.value.smtp_port = 'Valid port (1-65535) is required';
  }

  if (!form.value.password && currentView.value === 'add') {
    errors.value.password = 'Password is required';
  }

  return Object.keys(errors.value).length === 0;
}

// Test connection
async function testConnection() {
  if (!validateForm()) {
    toast.error('Please fix validation errors before testing');
    return;
  }

  try {
    testingConnection.value = true;
    testResult.value = null;

    const params: TestConnectionRequest = {
      imap_host: form.value.imap_host,
      imap_port: form.value.imap_port,
      smtp_host: form.value.smtp_host,
      smtp_port: form.value.smtp_port,
      email: form.value.email,
      password: form.value.password,
    };

    testResult.value = await invokeTestAccountConnection(params);

    if (testResult.value.imap_success && testResult.value.smtp_success) {
      toast.success('Connection test successful!');
    } else {
      toast.error('Connection test failed - see details below');
    }
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Connection test failed';
    toast.error(message);
  } finally {
    testingConnection.value = false;
  }
}

// Add new account
async function addAccount() {
  if (!validateForm()) {
    toast.error('Please fix validation errors');
    return;
  }

  try {
    isLoading.value = true;

    const params: AddAccountRequest = {
      email: form.value.email,
      provider: form.value.provider,
      imap_host: form.value.imap_host,
      imap_port: form.value.imap_port,
      smtp_host: form.value.smtp_host,
      smtp_port: form.value.smtp_port,
      password: form.value.password,
    };

    await invokeAddAccount(params);
    toast.success(`Account ${form.value.email} added successfully`);

    // Trigger initial sync for the new account
    try {
      await invokeSyncEmails();
      toast.success('Email sync started');
    } catch (syncErr) {
      console.warn('Failed to start sync:', syncErr);
      // Don't fail the account creation if sync fails
    }

    resetForm();
    await loadAccounts();
    currentView.value = 'list';
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Failed to add account';
    toast.error(message);
  } finally {
    isLoading.value = false;
  }
}

// Update existing account
async function updateAccount() {
  if (!selectedAccount.value || !validateForm()) {
    toast.error('Please fix validation errors');
    return;
  }

  try {
    isLoading.value = true;

    const params: UpdateAccountRequest = {
      accountId: selectedAccount.value.id,
      email: form.value.email,
      provider: form.value.provider,
      imap_host: form.value.imap_host,
      imap_port: form.value.imap_port,
      smtp_host: form.value.smtp_host,
      smtp_port: form.value.smtp_port,
      password: form.value.password || undefined,
    };

    await invokeUpdateAccount(params);
    toast.success(`Account ${form.value.email} updated successfully`);

    // Trigger sync for the updated account
    try {
      await invokeSyncEmails();
      toast.success('Email sync started');
    } catch (syncErr) {
      console.warn('Failed to start sync:', syncErr);
      // Don't fail the account update if sync fails
    }

    resetForm();
    await loadAccounts();
    currentView.value = 'list';
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Failed to update account';
    toast.error(message);
  } finally {
    isLoading.value = false;
  }
}

// Delete account
async function deleteAccount(account: Account) {
  if (!confirm(`Are you sure you want to delete ${account.email}?`)) {
    return;
  }

  try {
    isLoading.value = true;
    await invokeRemoveAccount(account.id);
    toast.success(`Account ${account.email} deleted`);
    await loadAccounts();
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Failed to delete account';
    toast.error(message);
  } finally {
    isLoading.value = false;
  }
}

// Navigate to add view
function navigateToAdd() {
  resetForm();
  testResult.value = null;
  currentView.value = 'add';
}

// Navigate to edit view
function navigateToEdit(account: Account) {
  selectedAccount.value = account;
  form.value = {
    email: account.email,
    provider: account.provider,
    imap_host: account.imap_host,
    imap_port: account.imap_port,
    smtp_host: account.smtp_host,
    smtp_port: account.smtp_port,
    password: '', // Don't pre-fill password
  };
  testResult.value = null;
  currentView.value = 'edit';
}

// Navigate back to list
function navigateToList() {
  currentView.value = 'list';
  resetForm();
}

// Reset form
function resetForm() {
  form.value = {
    email: '',
    provider: '',
    imap_host: '',
    imap_port: 993,
    smtp_host: '',
    smtp_port: 587,
    password: '',
  };
  errors.value = {};
  selectedAccount.value = null;
  testResult.value = null;
}

// Handle Escape key
function handleEscape(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    if (currentView.value === 'list') {
      emit('close');
    } else {
      navigateToList();
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleEscape);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleEscape);
});

// Account count
const accountCount = computed(() => accounts.value.length);

// Page title
const pageTitle = computed(() => {
  if (currentView.value === 'add') return 'Add Email Account';
  if (currentView.value === 'edit') return 'Edit Account';
  return 'Email Accounts';
});
</script>

<template>
  <div class="flex h-full flex-col bg-background">
    <!-- Header with SidebarTrigger -->
    <header class="flex h-14 items-center gap-4 border-b px-4 lg:h-[60px] lg:px-6">
      <SidebarTrigger class="-ml-1" />
      <div class="flex-1">
        <h1 class="text-lg font-semibold md:text-xl">{{ pageTitle }}</h1>
        <p v-if="currentView === 'list'" class="text-sm text-muted-foreground">
          Manage your email accounts - {{ accountCount }} account{{ accountCount !== 1 ? 's' : '' }}
        </p>
        <p v-else-if="currentView === 'add'" class="text-sm text-muted-foreground">
          Configure your new email account
        </p>
        <p v-else class="text-sm text-muted-foreground">
          Update account settings for {{ selectedAccount?.email }}
        </p>
      </div>
      <div class="flex items-center gap-2">
        <Button v-if="currentView === 'list'" @click="navigateToAdd" size="sm">
          Add Account
        </Button>
        <Button v-if="currentView === 'list'" @click="emit('close')" variant="outline" size="sm">
          Close
        </Button>
        <Button v-if="currentView !== 'list'" @click="navigateToList" variant="outline" size="sm">
          Back to List
        </Button>
      </div>
    </header>

    <!-- Main Content Area -->
    <main class="flex-1 overflow-y-auto p-4 md:p-6">
      <!-- Account List View -->
      <div v-if="currentView === 'list'">
        <div
          v-if="isLoading && accounts.length === 0"
          class="flex items-center justify-center py-12"
        >
          <p class="text-muted-foreground">Loading accounts...</p>
        </div>

        <div
          v-else-if="accounts.length === 0"
          class="flex flex-col items-center justify-center py-12"
        >
          <p class="text-muted-foreground mb-4">No accounts configured</p>
          <Button @click="navigateToAdd" size="sm">Add Your First Account</Button>
        </div>

        <div v-else class="space-y-4 max-w-4xl">
          <Card
            v-for="account in accounts"
            :key="account.id"
            class="p-4 hover:bg-accent/50 transition-colors"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-2">
                  <h3 class="font-semibold">{{ account.email }}</h3>
                  <Badge v-if="account.sync_enabled" variant="default" class="text-xs">
                    Syncing
                  </Badge>
                  <Badge v-else variant="secondary" class="text-xs">Paused</Badge>
                </div>
                <div class="grid grid-cols-2 gap-x-8 gap-y-1 text-sm text-muted-foreground">
                  <div><span class="font-medium">Provider:</span> {{ account.provider }}</div>
                  <div>
                    <span class="font-medium">IMAP:</span>
                    {{ account.imap_host }}:{{ account.imap_port }}
                  </div>
                  <div>
                    <span class="font-medium">SMTP:</span>
                    {{ account.smtp_host }}:{{ account.smtp_port }}
                  </div>
                </div>
              </div>
              <div class="flex gap-2">
                <Button @click="navigateToEdit(account)" variant="outline" size="sm"> Edit </Button>
                <Button
                  @click="deleteAccount(account)"
                  variant="destructive"
                  size="sm"
                  :disabled="isLoading"
                >
                  Delete
                </Button>
              </div>
            </div>
          </Card>
        </div>
      </div>

      <!-- Add Account Form View -->
      <div v-else-if="currentView === 'add'" class="max-w-2xl">
        <form @submit.prevent="addAccount" class="space-y-6">
          <!-- Email -->
          <div>
            <label class="block text-sm font-medium mb-2">Email Address *</label>
            <Input
              v-model="form.email"
              type="email"
              placeholder="user@example.com"
              :class="{ 'border-red-500': errors.email }"
            />
            <p v-if="errors.email" class="text-xs text-red-500 mt-1">{{ errors.email }}</p>
          </div>

          <!-- Provider -->
          <div>
            <label class="block text-sm font-medium mb-2">Provider *</label>
            <Input
              v-model="form.provider"
              placeholder="Gmail, Outlook, etc."
              :class="{ 'border-red-500': errors.provider }"
            />
            <p v-if="errors.provider" class="text-xs text-red-500 mt-1">{{ errors.provider }}</p>
          </div>

          <Separator />

          <!-- IMAP Settings -->
          <div>
            <h3 class="text-sm font-semibold mb-3">IMAP Settings</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium mb-2">Host *</label>
                <Input
                  v-model="form.imap_host"
                  placeholder="imap.gmail.com"
                  :class="{ 'border-red-500': errors.imap_host }"
                />
                <p v-if="errors.imap_host" class="text-xs text-red-500 mt-1">
                  {{ errors.imap_host }}
                </p>
              </div>
              <div>
                <label class="block text-sm font-medium mb-2">Port *</label>
                <Input
                  v-model.number="form.imap_port"
                  type="number"
                  placeholder="993"
                  :class="{ 'border-red-500': errors.imap_port }"
                />
                <p v-if="errors.imap_port" class="text-xs text-red-500 mt-1">
                  {{ errors.imap_port }}
                </p>
              </div>
            </div>
          </div>

          <!-- SMTP Settings -->
          <div>
            <h3 class="text-sm font-semibold mb-3">SMTP Settings</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium mb-2">Host *</label>
                <Input
                  v-model="form.smtp_host"
                  placeholder="smtp.gmail.com"
                  :class="{ 'border-red-500': errors.smtp_host }"
                />
                <p v-if="errors.smtp_host" class="text-xs text-red-500 mt-1">
                  {{ errors.smtp_host }}
                </p>
              </div>
              <div>
                <label class="block text-sm font-medium mb-2">Port *</label>
                <Input
                  v-model.number="form.smtp_port"
                  type="number"
                  placeholder="587"
                  :class="{ 'border-red-500': errors.smtp_port }"
                />
                <p v-if="errors.smtp_port" class="text-xs text-red-500 mt-1">
                  {{ errors.smtp_port }}
                </p>
                <p class="text-xs text-muted-foreground mt-1">
                  Gmail: use port 587 (STARTTLS recommended)
                </p>
              </div>
            </div>
          </div>

          <Separator />

          <!-- Password -->
          <div>
            <label class="block text-sm font-medium mb-2">Password *</label>
            <Input
              v-model="form.password"
              type="password"
              placeholder="Your email password"
              :class="{ 'border-red-500': errors.password }"
            />
            <p v-if="errors.password" class="text-xs text-red-500 mt-1">{{ errors.password }}</p>
            <p class="text-xs text-muted-foreground mt-1">
              Gmail requires an app-specific password. Stored securely in your system keychain.
            </p>
          </div>

          <!-- Test Connection Results -->
          <div v-if="testResult" class="p-4 border rounded-md space-y-2 bg-muted/30">
            <p class="font-medium text-sm">Connection Test Results:</p>
            <div class="flex items-center gap-2">
              <Badge :variant="testResult.imap_success ? 'default' : 'destructive'" class="text-xs">
                IMAP: {{ testResult.imap_success ? '✓ Success' : '✗ Failed' }}
              </Badge>
              <Badge :variant="testResult.smtp_success ? 'default' : 'destructive'" class="text-xs">
                SMTP: {{ testResult.smtp_success ? '✓ Success' : '✗ Failed' }}
              </Badge>
            </div>
            <p v-if="testResult.imap_error" class="text-xs text-red-500">
              IMAP: {{ testResult.imap_error }}
            </p>
            <p v-if="testResult.smtp_error" class="text-xs text-red-500">
              SMTP: {{ testResult.smtp_error }}
            </p>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 justify-end pt-4">
            <Button type="button" @click="navigateToList" variant="outline" :disabled="isLoading">
              Cancel
            </Button>
            <Button
              type="button"
              @click="testConnection"
              variant="secondary"
              :disabled="isLoading || testingConnection"
            >
              {{ testingConnection ? 'Testing...' : 'Test Connection' }}
            </Button>
            <Button type="submit" :disabled="isLoading">
              {{ isLoading ? 'Adding...' : 'Add Account' }}
            </Button>
          </div>
        </form>
      </div>

      <!-- Edit Account Form View -->
      <div v-else-if="currentView === 'edit'" class="max-w-2xl">
        <form @submit.prevent="updateAccount" class="space-y-6">
          <!-- Email -->
          <div>
            <label class="block text-sm font-medium mb-2">Email Address *</label>
            <Input
              v-model="form.email"
              type="email"
              placeholder="user@example.com"
              :class="{ 'border-red-500': errors.email }"
            />
            <p v-if="errors.email" class="text-xs text-red-500 mt-1">{{ errors.email }}</p>
          </div>

          <!-- Provider -->
          <div>
            <label class="block text-sm font-medium mb-2">Provider *</label>
            <Input
              v-model="form.provider"
              placeholder="Gmail, Outlook, etc."
              :class="{ 'border-red-500': errors.provider }"
            />
            <p v-if="errors.provider" class="text-xs text-red-500 mt-1">{{ errors.provider }}</p>
          </div>

          <Separator />

          <!-- IMAP Settings -->
          <div>
            <h3 class="text-sm font-semibold mb-3">IMAP Settings</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium mb-2">Host *</label>
                <Input
                  v-model="form.imap_host"
                  placeholder="imap.gmail.com"
                  :class="{ 'border-red-500': errors.imap_host }"
                />
                <p v-if="errors.imap_host" class="text-xs text-red-500 mt-1">
                  {{ errors.imap_host }}
                </p>
              </div>
              <div>
                <label class="block text-sm font-medium mb-2">Port *</label>
                <Input
                  v-model.number="form.imap_port"
                  type="number"
                  placeholder="993"
                  :class="{ 'border-red-500': errors.imap_port }"
                />
                <p v-if="errors.imap_port" class="text-xs text-red-500 mt-1">
                  {{ errors.imap_port }}
                </p>
              </div>
            </div>
          </div>

          <!-- SMTP Settings -->
          <div>
            <h3 class="text-sm font-semibold mb-3">SMTP Settings</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium mb-2">Host *</label>
                <Input
                  v-model="form.smtp_host"
                  placeholder="smtp.gmail.com"
                  :class="{ 'border-red-500': errors.smtp_host }"
                />
                <p v-if="errors.smtp_host" class="text-xs text-red-500 mt-1">
                  {{ errors.smtp_host }}
                </p>
              </div>
              <div>
                <label class="block text-sm font-medium mb-2">Port *</label>
                <Input
                  v-model.number="form.smtp_port"
                  type="number"
                  placeholder="587"
                  :class="{ 'border-red-500': errors.smtp_port }"
                />
                <p v-if="errors.smtp_port" class="text-xs text-red-500 mt-1">
                  {{ errors.smtp_port }}
                </p>
              </div>
            </div>
          </div>

          <Separator />

          <!-- Password -->
          <div>
            <label class="block text-sm font-medium mb-2">
              Password (leave blank to keep current)
            </label>
            <Input v-model="form.password" type="password" placeholder="New password (optional)" />
            <p class="text-xs text-muted-foreground mt-1">
              Leave blank to use stored password for testing, or enter a new password to update it
            </p>
          </div>

          <!-- Test Connection Results -->
          <div v-if="testResult" class="p-4 border rounded-md space-y-2 bg-muted/30">
            <p class="font-medium text-sm">Connection Test Results:</p>
            <div class="flex items-center gap-2">
              <Badge :variant="testResult.imap_success ? 'default' : 'destructive'" class="text-xs">
                IMAP: {{ testResult.imap_success ? '✓ Success' : '✗ Failed' }}
              </Badge>
              <Badge :variant="testResult.smtp_success ? 'default' : 'destructive'" class="text-xs">
                SMTP: {{ testResult.smtp_success ? '✓ Success' : '✗ Failed' }}
              </Badge>
            </div>
            <p v-if="testResult.imap_error" class="text-xs text-red-500">
              IMAP: {{ testResult.imap_error }}
            </p>
            <p v-if="testResult.smtp_error" class="text-xs text-red-500">
              SMTP: {{ testResult.smtp_error }}
            </p>
          </div>

          <!-- Actions -->
          <div class="flex gap-2 justify-end pt-4">
            <Button type="button" @click="navigateToList" variant="outline" :disabled="isLoading">
              Cancel
            </Button>
            <Button
              type="button"
              @click="testConnection"
              variant="secondary"
              :disabled="isLoading || testingConnection"
            >
              {{ testingConnection ? 'Testing...' : 'Test Connection' }}
            </Button>
            <Button type="submit" :disabled="isLoading">
              {{ isLoading ? 'Updating...' : 'Update Account' }}
            </Button>
          </div>
        </form>
      </div>
    </main>
  </div>
</template>
