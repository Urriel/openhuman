# Command Palette (Superhuman-Style) - Initial Idea

**Date:** 2026-01-06

## User's Original Request

Implement the command palette from superhuman:

A. Command Palette → Product DNA
Current state: Likely missing or suboptimal

Superhuman reality: The command palette (Cmd+K / Ctrl+K) IS the interface

What to do:

- Make command palette the PRIMARY navigation (not secondary)
- Fuzzy search with context awareness (different actions for inbox vs. thread view)
- ALL major actions ONLY accessible via command palette + keyboard shortcuts (not buttons)
- Examples: "Mark done", "Snooze", "Star", "New split", "Reply all", "Search"

## Additional Context from Discussion

User preferences established during initial conversation:

1. **Search Integration:** Replace SearchBar with unified command palette (Cmd+K for commands, / for search)
2. **Command Set:** Full Superhuman-inspired command set (~38 commands)
3. **User Feedback:** Toast notifications for all actions with undo support
4. **Testing:** Include tests throughout implementation
5. **Label Picker:** Sub-page within command palette (nested items pattern)
6. **Undo Window:** Until next action (not time-based)
7. **Search Mode:** Email search results shown directly in command palette
8. **Snooze:** Plan architecture but defer implementation to Phase 2
