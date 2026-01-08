# Product Roadmap

1. [ ] Email Account Management — Add, edit, and remove email accounts with secure credential storage. Users can connect any POP3/SMTP provider (Gmail, Outlook, custom domains) and manage multiple accounts. `S`

2. [ ] Email Sync Engine — Fetch emails from all connected accounts via POP3, parse MIME messages, and store in local SQLite with incremental sync strategy. Background job orchestration for automatic syncing. `L`

3. [ ] Email Threading & Storage — Parse email headers to build conversation threads, implement SQLite schema for messages/threads/accounts, and create migrations system for schema evolution. `M`

4. [x] Core Email List UI — Display emails in a virtualized list using TanStack Virtual with infinite scrolling, show sender/subject/preview/timestamp, and support multi-select for batch operations. `M` ✅ **Complete** (MVP UI + Workflows)

5. [x] Email Reader & Navigation — View full email content with HTML rendering, navigate between conversations using keyboard (j/k), and display thread history with quoted replies. `M` ✅ **Complete** (MVP UI + Workflows)

6. [x] Basic Email Actions — Archive, delete, mark read/unread, star/unstar, and move messages to folders. All actions accessible via keyboard shortcuts and update both UI and backend SQLite. `S` ✅ **Complete** (MVP UI + Workflows)

7. [~] Email Composer — Compose new emails with rich text editor, reply/reply-all/forward, attach files with drag-and-drop, and send via SMTP with proper MIME formatting. `L` ⚠️ **Partial** (UI complete, attachment sending pending backend work)

8. [ ] Keyboard Shortcuts System — Implement global keyboard shortcut manager covering navigation (j/k, enter), actions (e for archive, # for delete, s for star), and composer (c for compose, r for reply). `M`

9. [ ] Command Palette (Cmd+K) — Fuzzy-searchable command palette to access any action, navigate to any email, switch accounts, and execute workflows without mouse. `M`

10. [ ] Split Inbox — Rule-based inbox views that automatically separate Important, Notifications, and Other messages based on sender patterns, user-defined rules, and historical behavior. `L`

11. [x] Search Functionality — Full-text search across all messages, accounts, and threads with instant results from local SQLite. Filter by sender, date range, account, and read/starred status. `M` ✅ **Complete** (MVP UI + Workflows - FTS5 search with highlighting)

12. [ ] Snooze & Remind Me — Snooze emails to reappear at a specific date/time, with options for "Later Today," "Tomorrow," "Next Week," or custom date picker. Background scheduler to resurface snoozed items. `M`

13. [ ] Send Later — Schedule emails to send at a future date/time, save as drafts with scheduled metadata, and use background job to send via SMTP at scheduled time. `M`

14. [ ] Snippets & Templates — Create, save, and insert reusable email templates with variable substitution. Access via command palette or keyboard shortcut, with common snippets for frequent responses. `S`

15. [ ] Settings & Preferences UI — Configure accounts, keyboard shortcuts, Split Inbox rules, snooze defaults, appearance (theme, density), and notification preferences. `M`

16. [ ] Read Receipts (Optional) — Track when recipients open emails using pixel tracking, display read status in email list and reader, and allow users to enable/disable per message. `M`

17. [ ] AI Compose & Instant Reply (Optional) — AI-powered email composition suggestions, instant reply generation based on context, and tone adjustment (professional, casual, brief). `L`

18. [ ] AI Thread Summaries (Optional) — Automatically summarize long email threads with key points, action items, and decision highlights to save reading time. `M`

19. [ ] Smart Unsubscribe & Cleanup — Detect newsletter senders, bulk unsubscribe from unwanted lists, and identify spam patterns for quick cleanup. `S`

20. [ ] Cross-Platform Packaging — Build and package for macOS, Windows, and Linux with platform-specific installers, code signing, and auto-update mechanism. `M`

> Notes
>
> - Items 1-9 form the MVP (Inbox Zero core experience)
> - Items 10-15 add Superhuman-inspired productivity features
> - Items 16-19 are optional pro/AI features
> - Item 20 is release preparation
> - Each item represents end-to-end (frontend + backend) functionality
> - Technical dependencies: 1→2→3→4, 7→13, 8→9, 12-13 require background job system
