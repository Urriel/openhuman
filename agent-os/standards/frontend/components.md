## UI component best practices

- **Single Responsibility**: Each component should have one clear purpose and do it well
- **Reusability**: Design components to be reused across different contexts with configurable props
- **Composability**: Build complex UIs by combining smaller, simpler components rather than monolithic structures
- **Clear Interface**: Define explicit, well-documented props with sensible defaults for ease of use
- **Encapsulation**: Keep internal implementation details private and expose only necessary APIs
- **Consistent Naming**: Use clear, descriptive names that indicate the component's purpose and follow team conventions
- **State Management**: Keep state as local as possible; lift it up only when needed by multiple components
- **Minimal Props**: Keep the number of props manageable; if a component needs many props, consider composition or splitting it
- **Documentation**: Document component usage, props, and provide examples for easier adoption by team members

## Design Principles Compliance

All UI components must follow OpenHuman's design principles. See `agent-os/product/design-principles.md` for full guidelines.

**Required for all components:**

### Keyboard First

- Every action must have a keyboard shortcut
- Keyboard shortcuts must be documented in tooltips (e.g., "Archive `e`")
- Tab order must be logical and predictable
- Focus indicators must be clearly visible
- Forms must be submittable with Enter
- Modals must be dismissible with Escape

### Density Over Whitespace

- Use compact padding (8-12px, not 24-32px)
- Line height should be 1.4-1.5 (not 2.0)
- Show maximum information per screen (15-20 emails in list view)
- Create visual hierarchy with typography, not excessive whitespace
- Use screen real estate efficiently (three-pane layout, inline actions)

### AI as Collaborator

- All AI suggestions must be clearly labeled with "AI suggested..." indicator
- User must be able to accept, edit, or reject AI output
- Never take AI actions automatically without user confirmation
- AI features must be optional, not required for core functionality

### Speed Through Reduction

- Remove unnecessary confirmation dialogs (only confirm destructive actions)
- Use optimistic UI updates for perceived speed
- Smart defaults to reduce user input (last used account, reply-all, etc.)
- Inline actions preferred over modals/separate screens
- Target <2 seconds for common actions (archive, star, mark read)

**Component Checklist:**

Before marking a UI component complete, verify:

- [ ] All actions have keyboard shortcuts
- [ ] Shortcuts are visible in tooltips
- [ ] Padding is compact (8-12px)
- [ ] Information density is high but readable
- [ ] AI features (if any) are clearly labeled and optional
- [ ] No unnecessary modals or confirmation dialogs
- [ ] Optimistic updates implemented where appropriate
- [ ] Component works perfectly without mouse interaction
