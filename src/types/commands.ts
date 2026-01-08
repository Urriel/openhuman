/**
 * Type definitions for Tauri commands
 *
 * This file defines the TypeScript types for all Tauri commands to ensure
 * type safety between the frontend (TypeScript) and backend (Rust).
 *
 * ## Type Safety Pattern
 *
 * For each Tauri command, define:
 * 1. Request type (if command accepts parameters)
 * 2. Response type (what the command returns)
 *
 * ## Example Usage
 *
 * ```typescript
 * import { invoke } from '@tauri-apps/api/core';
 * import type { GreetRequest, GreetResponse } from '@/types/commands';
 *
 * const request: GreetRequest = { name: 'World' };
 * const response = await invoke<GreetResponse>('greet', request);
 * console.log(response); // Type-safe access to response
 * ```
 */

// Greet Command Types
export interface GreetRequest {
  name: string;
}

export type GreetResponse = string;

/**
 * Type-safe wrapper for invoking the greet command
 *
 * @param name - The name to greet
 * @returns Promise resolving to greeting string
 *
 * @example
 * ```typescript
 * const greeting = await invokeGreet('World');
 * console.log(greeting); // "Hello, World! You've been greeted from Rust!"
 * ```
 */
export async function invokeGreet(name: string): Promise<GreetResponse> {
  // Dynamic import to avoid bundling issues
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<GreetResponse>('greet', { name });
}

// Label Operations Types

export interface Label {
  id: number;
  account_id: number;
  name: string;
  color?: string;
  created_at: string;
  message_count?: number;
}

export interface CreateLabelRequest {
  accountId: number;
  name: string;
  color?: string;
}

export interface ListLabelsRequest {
  accountId: number;
  includeCounts: boolean;
}

export interface DeleteLabelRequest {
  labelId: number;
}

export interface ApplyLabelRequest {
  messageId: number;
  labelId: number;
}

export interface RemoveLabelRequest {
  messageId: number;
  labelId: number;
}

export interface GetMessageLabelsRequest {
  messageId: number;
}

export interface ArchiveMessagesRequest {
  messageIds: number[];
}

// Folder Operations Types

export interface Folder {
  id: number;
  account_id: number;
  name: string;
  message_count: number;
}

export interface ListFoldersRequest {
  accountId?: number;
}

export type ListFoldersResponse = Folder[];

// Account Management Types

export interface Account {
  id: number;
  email: string;
  provider: string;
  imap_host: string;
  imap_port: number;
  smtp_host: string;
  smtp_port: number;
  sync_enabled: boolean;
}

export interface AddAccountRequest {
  email: string;
  provider: string;
  imap_host: string;
  imap_port: number;
  smtp_host: string;
  smtp_port: number;
  password: string;
}

export type AddAccountResponse = Account;

export type ListAccountsResponse = Account[];

export interface RemoveAccountRequest {
  accountId: number;
}

export interface UpdateAccountRequest {
  accountId: number;
  email: string;
  provider: string;
  imap_host: string;
  imap_port: number;
  smtp_host: string;
  smtp_port: number;
  password?: string; // Optional - only update if provided
}

export type UpdateAccountResponse = Account;

export interface TestConnectionRequest {
  imap_host: string;
  imap_port: number;
  smtp_host: string;
  smtp_port: number;
  email: string;
  password: string;
}

export interface TestConnectionResult {
  imap_success: boolean;
  smtp_success: boolean;
  imap_error: string | null;
  smtp_error: string | null;
}

export type TestConnectionResponse = TestConnectionResult;

// IMAP-specific Types

export interface ImapFolder {
  id: number;
  account_id: number;
  name: string;
  folder_type?: string;
  selectable: boolean;
  flags?: string;
  uidvalidity?: number;
  uidnext?: number;
  message_count: number;
}

export interface ImapSyncState {
  account_id: number;
  uid_validity: number;
  uid_next: number;
  uid_mappings: string; // JSON string of UID mappings
}

export type ImapMessageFlag = '\\Seen' | '\\Flagged' | '\\Deleted' | '\\Draft' | '\\Answered';

// Email Sync Types

export interface SyncStats {
  messages_synced: number;
  new_messages: number;
  errors: string[];
}

export interface SyncStatusInfo {
  account_id: number;
  sync_status: string;
  last_sync_at: string | null;
  messages_fetched: number;
  error_message: string | null;
}

export interface SyncEmailsRequest {
  accountId?: number; // undefined = sync all accounts
}

export type SyncEmailsResponse = SyncStats;

export interface GetSyncStatusRequest {
  accountId: number;
}

export type GetSyncStatusResponse = SyncStatusInfo;

export interface CancelSyncRequest {
  accountId: number;
}

// Email Operations Types

export interface SendEmailParams {
  account_id: number;
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string;
  body_plain: string;
  body_html?: string;
}

export type SendEmailResponse = number; // Returns outbox ID

export interface MarkReadRequest {
  messageIds: number[];
}

export interface MarkUnreadRequest {
  messageIds: number[];
}

export interface StarMessageRequest {
  messageId: number;
}

export interface UnstarMessageRequest {
  messageId: number;
}

export interface DeleteMessageRequest {
  messageId: number;
}

export interface BulkMarkReadRequest {
  messageIds: number[];
  isRead: boolean;
}

export interface BulkArchiveMessagesRequest {
  messageIds: number[];
}

// Email List Types

export interface MessageListItem {
  id: number;
  subject: string | null;
  from_addr: string;
  preview: string;
  date: string | null;
  is_read: boolean;
  is_starred: boolean;
  has_attachments: boolean;
}

export interface ListMessagesRequest {
  accountId?: number;
  folder?: string;
  labelId?: number;
  isRead?: boolean;
  isStarred?: boolean;
  limit?: number;
}

export type ListMessagesResponse = MessageListItem[];

// Search Types

export interface MessageSearchResult {
  id: number;
  account_id: number;
  message_id: string;
  subject: string | null;
  from_addr: string;
  to_addr: string | null;
  date: string | null;
  is_read: boolean;
  is_starred: boolean;
  snippet: string; // Contains <mark> tags for highlighting
}

export interface SearchMessagesRequest {
  query: string;
  accountId?: number;
}

export type SearchMessagesResponse = MessageSearchResult[];

// =============================================================================
// Tauri Event Types
// =============================================================================

/**
 * Event payload for sync_started event
 */
export interface SyncStartedEvent {
  account_id: number;
  timestamp: string;
}

/**
 * Event payload for sync_progress event
 */
export interface SyncProgressEvent {
  account_id: number;
  messages_synced: number;
  total_messages: number;
  current_message: string;
}

/**
 * Event payload for sync_completed event
 */
export interface SyncCompletedEvent {
  account_id: number;
  stats: SyncStats;
  timestamp: string;
}

/**
 * Event payload for sync_failed event
 */
export interface SyncFailedEvent {
  account_id: number;
  error: string;
  timestamp: string;
}

/**
 * Event payload for send_status event
 */
export interface SendStatusEvent {
  outbox_id: number;
  status: 'sending' | 'sent' | 'failed';
  error?: string;
  timestamp: string;
}

/**
 * Event payload for auth_required event
 */
export interface AuthRequiredEvent {
  account_id: number;
  email: string;
  reason: string;
  timestamp: string;
}

/**
 * All possible Tauri event names
 */
export type TauriEventName =
  | 'sync_started'
  | 'sync_progress'
  | 'sync_completed'
  | 'sync_failed'
  | 'send_status'
  | 'auth_required';

/**
 * Tauri event payload mapping
 */
export interface TauriEventMap {
  sync_started: SyncStartedEvent;
  sync_progress: SyncProgressEvent;
  sync_completed: SyncCompletedEvent;
  sync_failed: SyncFailedEvent;
  send_status: SendStatusEvent;
  auth_required: AuthRequiredEvent;
}

// =============================================================================
// Type-Safe Command Wrapper Functions
// =============================================================================

/**
 * Account Management Commands
 */
export async function invokeAddAccount(params: AddAccountRequest): Promise<AddAccountResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<AddAccountResponse>('add_account', {
    email: params.email,
    provider: params.provider,
    imapHost: params.imap_host,
    imapPort: params.imap_port,
    smtpHost: params.smtp_host,
    smtpPort: params.smtp_port,
    password: params.password,
  });
}

export async function invokeListAccounts(): Promise<ListAccountsResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<ListAccountsResponse>('list_accounts');
}

export async function invokeRemoveAccount(accountId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('delete_account', { accountId });
}

export async function invokeUpdateAccount(
  params: UpdateAccountRequest
): Promise<UpdateAccountResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<UpdateAccountResponse>('update_account', {
    accountId: params.accountId,
    email: params.email,
    provider: params.provider,
    imapHost: params.imap_host,
    imapPort: params.imap_port,
    smtpHost: params.smtp_host,
    smtpPort: params.smtp_port,
    password: params.password,
  });
}

export async function invokeTestAccountConnection(
  params: TestConnectionRequest
): Promise<TestConnectionResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<TestConnectionResponse>('test_account_connection', {
    imapHost: params.imap_host,
    imapPort: params.imap_port,
    smtpHost: params.smtp_host,
    smtpPort: params.smtp_port,
    email: params.email,
    password: params.password,
  });
}

/**
 * Email Sync Commands
 */
export async function invokeSyncEmails(accountId?: number): Promise<SyncEmailsResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<SyncEmailsResponse>('sync_emails', { accountId });
}

export async function invokeGetSyncStatus(accountId: number): Promise<GetSyncStatusResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<GetSyncStatusResponse>('get_sync_status', { accountId });
}

export async function invokeCancelSync(accountId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('cancel_sync', { accountId });
}

/**
 * Email Operations Commands
 */
export async function invokeSendEmail(params: SendEmailParams): Promise<SendEmailResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<SendEmailResponse>('send_email', { params });
}

export async function invokeMarkRead(messageIds: number[]): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('mark_read', { messageIds });
}

export async function invokeMarkUnread(messageIds: number[]): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('mark_unread', { messageIds });
}

export async function invokeStarMessage(messageId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('star_message', { messageId });
}

export async function invokeUnstarMessage(messageId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('unstar_message', { messageId });
}

export async function invokeDeleteMessage(messageId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('delete_message', { messageId });
}

export async function invokeBulkMarkRead(messageIds: number[], isRead: boolean): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('bulk_mark_read', { messageIds, isRead });
}

export async function invokeBulkArchiveMessages(messageIds: number[]): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('bulk_archive_messages', { messageIds });
}

/**
 * Email List Commands
 */
export async function invokeListMessages(
  params: ListMessagesRequest = {}
): Promise<ListMessagesResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<ListMessagesResponse>('list_messages', {
    accountId: params.accountId,
    folder: params.folder,
    labelId: params.labelId,
    isRead: params.isRead,
    isStarred: params.isStarred,
    limit: params.limit,
  });
}

/**
 * Search Commands
 */
export async function invokeSearchMessages(
  query: string,
  accountId?: number
): Promise<SearchMessagesResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<SearchMessagesResponse>('search_messages', { query, accountId });
}

/**
 * Label Operations Commands
 */
export async function invokeCreateLabel(
  accountId: number,
  name: string,
  color?: string
): Promise<Label> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<Label>('create_label', { accountId, name, color });
}

export async function invokeListLabels(accountId: number, includeCounts = false): Promise<Label[]> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<Label[]>('list_labels', { accountId, includeCounts });
}

export async function invokeDeleteLabel(labelId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('delete_label', { labelId });
}

export async function invokeApplyLabel(messageId: number, labelId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('apply_label', { messageId, labelId });
}

export async function invokeRemoveLabel(messageId: number, labelId: number): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('remove_label', { messageId, labelId });
}

export async function invokeGetMessageLabels(messageId: number): Promise<Label[]> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<Label[]>('get_message_labels', { messageId });
}

export async function invokeArchiveMessages(messageIds: number[]): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('archive_messages', { messageIds });
}

/**
 * Folder Operations Commands
 */
export async function invokeListFolders(accountId?: number): Promise<ListFoldersResponse> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<ListFoldersResponse>('list_folders', { accountId });
}

/**
 * Start the background sync scheduler
 * This will automatically sync emails at regular intervals
 */
export async function invokeStartSyncScheduler(): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<void>('start_sync_scheduler');
}
