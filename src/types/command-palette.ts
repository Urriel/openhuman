/**
 * Command Palette Type Definitions
 */

export type CommandCategory =
  | 'Email Actions'
  | 'Composition'
  | 'Navigation'
  | 'Selection'
  | 'Labels'
  | 'Search'
  | 'System';

export type ViewContext = 'inbox' | 'thread' | 'compose' | 'global';

export type CommandPaletteMode = 'commands' | 'search' | 'labels';

export interface Command {
  id: string;
  label: string;
  category: CommandCategory;
  keywords?: string[];
  shortcut?: string;
  action: () => void | Promise<void>;
  context?: ViewContext[];
  disabled?: boolean;
}

export interface CommandPaletteState {
  isOpen: boolean;
  mode: CommandPaletteMode;
  query: string;
  selectedCommandId: string | null;
  navigationStack: string[];
}

export interface SearchResultItem {
  id: number;
  subject: string;
  sender: string;
  snippet: string;
  timestamp: string;
  is_read: boolean;
  is_starred: boolean;
}
