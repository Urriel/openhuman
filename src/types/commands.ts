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
