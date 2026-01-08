# Design Principles

OpenHuman follows a coherent design philosophy that prioritizes power users, speed, and keyboard-first workflows. These principles apply across the entire application—both inbox and composition interfaces—and should guide all UI/UX decisions.

---

## Core Design Principles

### 1. Keyboard First, Mouse Optional

**Philosophy:** The UI should feel closer to a terminal or code editor than a traditional web app.

**Implementation Guidelines:**

- **Every action has a keyboard shortcut** - No critical functionality should be mouse-only
- **Visible shortcuts** - Display keyboard hints in tooltips and menus (e.g., "Archive `e`")
- **Command palette everywhere** - Cmd+K should be the fastest way to do anything
- **Tab/Enter navigation** - Forms and dialogs should be fully navigable with keyboard
- **Focus management** - Clear visual focus indicators, predictable tab order
- **Minimal modal usage** - Modals interrupt keyboard flow; prefer inline actions or command palette

**Examples:**

```typescript
// Good: Keyboard shortcut with visual indicator
<Button onClick={archive} title="Archive (e)">
  Archive <kbd>e</kbd>
</Button>

// Bad: Mouse-only action with no keyboard alternative
<IconButton onClick={archive} /> // No shortcut, no text
```

**Metrics:**

- Target: 95%+ of user actions achievable via keyboard alone
- Power users should complete email triage without touching mouse

---

### 2. Density Over Whitespace

**Philosophy:** Information density is high but organized—no excessive padding, but clear visual hierarchy.

**Implementation Guidelines:**

- **Compact layouts** - Use smaller padding (8px/12px instead of 24px/32px)
- **Visible information** - Show more emails per screen (target: 15-20 in list view)
- **Smart truncation** - Show previews/snippets, truncate intelligently with "..."
- **Clear hierarchy** - Use font weight, size, and color to create structure (not just whitespace)
- **Minimal chrome** - Reduce UI decorations, focus on content
- **Efficient use of screen real estate** - Three-pane layout maximizes content visibility

**Examples:**

```vue
<!-- Good: Compact, information-dense email list item -->
<div class="flex items-start gap-2 py-2 px-3">
  <Checkbox />
  <div class="flex-1 min-w-0">
    <div class="flex items-baseline gap-2">
      <span class="font-semibold text-sm truncate">{{ from }}</span>
      <span class="text-xs text-muted-foreground">{{ time }}</span>
    </div>
    <div class="text-sm truncate">{{ subject }}</div>
    <div class="text-xs text-muted-foreground truncate">{{ preview }}</div>
  </div>
</div>

<!-- Bad: Excessive whitespace, low information density -->
<div class="p-8 mb-6">
  <h3 class="mb-4">{{ from }}</h3>
  <p class="mb-4">{{ subject }}</p>
  <p class="mb-4">{{ preview }}</p>
</div>
```

**Metrics:**

- Target: 15-20 emails visible in list view (1080p screen)
- Line height: 1.4-1.5 (not 2.0)
- Padding: 8-12px (not 24-32px)

---

### 3. AI as a Collaborator, Not a Replacement

**Philosophy:** AI drafts, suggests, and refines, but the human always makes the final call.

**Implementation Guidelines:**

- **AI suggestions, not commands** - Always show "AI suggested..." with option to accept/reject
- **Human-in-the-loop** - Never send emails or make destructive actions without confirmation
- **Transparent AI** - Show when AI is active, what it's doing, and confidence levels
- **Manual override always available** - User can edit/reject any AI suggestion
- **AI as a time-saver** - Focus on reducing repetitive work (drafting replies, summarizing threads)
- **Optional, not required** - Core features work perfectly without AI

**Examples:**

```vue
<!-- Good: AI suggestion with human approval -->
<div class="border-l-2 border-blue-500 bg-blue-50 p-3 rounded">
  <div class="flex items-center gap-2 mb-2">
    <Sparkles class="w-4 h-4 text-blue-600" />
    <span class="text-xs font-medium text-blue-700">AI Suggested Reply</span>
  </div>
  <p class="text-sm mb-3">{{ aiDraft }}</p>
  <div class="flex gap-2">
    <Button size="sm" @click="acceptAIDraft">Use this draft</Button>
    <Button size="sm" variant="outline" @click="editAIDraft">Edit</Button>
    <Button size="sm" variant="ghost" @click="dismissAI">Dismiss</Button>
  </div>
</div>

<!-- Bad: AI sends email without user review -->
<Button @click="sendAIGeneratedEmail">Send AI Reply</Button>
```

**Metrics:**

- AI suggestions should save >30 seconds per use
- User should be able to review/edit AI output in <5 seconds
- 0% of actions should be AI-only without human confirmation

---

### 4. Speed Through Reduction

**Philosophy:** Remove what slows you down (heavy modals, redundant UI, clutter), keep what accelerates (shortcuts, previews, smart defaults).

**Implementation Guidelines:**

- **Remove friction** - Minimize clicks/steps to complete common actions
- **Smart defaults** - Pre-fill forms with intelligent defaults (last used account, reply-all, etc.)
- **Instant feedback** - Show loading states, optimistic updates, immediate response
- **No unnecessary confirmations** - Only confirm destructive actions (delete, remove account)
- **Keyboard shortcuts over menus** - Direct keyboard access faster than menu navigation
- **Inline actions** - Edit in place instead of opening separate screens/modals
- **Batch operations** - Multi-select and bulk actions for repetitive tasks

**What to Remove:**

- ❌ Heavy modal dialogs for simple actions (use inline or command palette)
- ❌ Redundant buttons (if keyboard shortcut exists, button is optional)
- ❌ Excessive animations (subtle transitions only, <200ms)
- ❌ Confirmation dialogs for non-destructive actions
- ❌ Multi-step wizards (combine into single view when possible)

**What to Keep:**

- ✅ Keyboard shortcuts for every action
- ✅ Preview panes (avoid opening separate windows)
- ✅ Smart autocomplete (email addresses, snippets)
- ✅ Undo for reversible actions (archive, mark read, etc.)
- ✅ Batch selection with Shift+Click, Cmd+A

**Examples:**

```vue
<!-- Good: Inline action with keyboard shortcut, no confirmation -->
<EmailListItem @keydown.e="archive(email)" />

<!-- Bad: Modal dialog with multiple steps -->
<Modal v-if="showArchiveConfirm">
  <p>Are you sure you want to archive this email?</p>
  <Button @click="confirmArchive">Yes</Button>
  <Button @click="cancelArchive">No</Button>
</Modal>
```

**Metrics:**

- Target: <2 seconds to archive an email
- Target: <5 seconds to compose and send a quick reply
- Target: <1 second for UI to respond to user input (optimistic updates)

---

## Target Audience

These design principles appeal strongly to:

- **Founders** - Need to process hundreds of emails daily, value speed over aesthetics
- **Sales Teams** - High email volume, keyboard shortcuts save hours per week
- **Customer Success Managers** - Need fast responses, templates, and efficient workflows
- **Executives** - Measure productivity in emails-per-hour, see email as high-stakes communication
- **Power Users** - Tech-savvy individuals who prefer terminal-like efficiency over GUI simplicity

**Not optimized for:**

- Casual email users who check email once per day
- Users who prefer visual/graphical interfaces over keyboard shortcuts
- Users who want minimalist design with lots of whitespace

---

## UI/UX Implementation Checklist

When designing or implementing any UI component, verify:

### Keyboard First

- [ ] Every action has a keyboard shortcut
- [ ] Shortcuts are documented in tooltips/help
- [ ] Tab order is logical and predictable
- [ ] Focus indicators are clearly visible
- [ ] Forms can be submitted with Enter
- [ ] Modals can be dismissed with Escape

### Density Over Whitespace

- [ ] Padding is compact (8-12px, not 24-32px)
- [ ] Line height is readable but not excessive (1.4-1.5)
- [ ] Multiple items visible per screen (15-20 emails)
- [ ] Visual hierarchy uses typography, not just spacing
- [ ] Screen real estate is used efficiently

### AI as Collaborator

- [ ] AI suggestions are clearly labeled
- [ ] User can accept, edit, or reject AI output
- [ ] No AI actions are automatic/destructive
- [ ] AI is optional, not required for core functionality
- [ ] Transparency: user knows when/how AI is being used

### Speed Through Reduction

- [ ] Removed unnecessary confirmation dialogs
- [ ] Removed redundant UI elements
- [ ] Smart defaults reduce user input
- [ ] Actions complete in <2 seconds
- [ ] Optimistic UI updates for perceived speed
- [ ] Batch operations available for repetitive tasks

---

## Anti-Patterns to Avoid

### ❌ Mouse-First Design

```vue
<!-- BAD: No keyboard alternative -->
<DropdownMenu>
  <DropdownMenuItem @click="archive">Archive</DropdownMenuItem>
</DropdownMenu>
```

### ❌ Excessive Whitespace

```vue
<!-- BAD: Low information density -->
<div class="p-12 my-8">
  <h1 class="mb-8 text-4xl">{{ email.subject }}</h1>
  <p class="mb-8 leading-loose">{{ email.preview }}</p>
</div>
```

### ❌ AI Replacing User

```vue
<!-- BAD: AI takes action without user consent -->
<Button @click="aiAutoReplyAndSend">Let AI Handle This</Button>
```

### ❌ Speed Reduction

```vue
<!-- BAD: Unnecessary confirmation for non-destructive action -->
<Modal v-if="confirmArchive">
  <p>Archive this email?</p>
  <Button @click="archive">Yes</Button>
  <Button @click="cancel">No</Button>
</Modal>
```

---

## Component-Specific Guidelines

### Email List

- **Density:** Show 15-20 emails per screen (1080p)
- **Keyboard:** j/k navigation, e to archive, # to delete, s to star
- **Speed:** Virtual scrolling at 60 FPS, optimistic updates

### Email Reader

- **Keyboard:** j/k to navigate between emails, r to reply, a to reply-all
- **Density:** Show message content + thread history in single view
- **Speed:** Inline reply (no separate compose window)

### Email Composer

- **Keyboard:** Tab through fields, Cmd+Enter to send
- **AI:** Suggest drafts, allow editing before send
- **Speed:** Smart defaults (reply-to, subject), autocomplete recipients

### Command Palette

- **Keyboard:** Cmd+K to open, fuzzy search, Enter to execute
- **Speed:** <100ms to show results, <1 second to execute action
- **Density:** Show 10-15 results per screen

---

## Related Documentation

- **Architecture Principles:** `agent-os/product/tech-stack.md` (lines 118-125)
- **Component Standards:** `agent-os/standards/frontend/components.md`
- **Mission & User Personas:** `agent-os/product/mission.md`
- **Keyboard Shortcuts:** Future: `docs/KEYBOARD_SHORTCUTS.md`
- **Performance Budgets:** `agent-os/product/tech-stack.md` (lines 106-116)

---

## Enforcement

AI coding agents should:

1. **Reference this document** when designing new UI components
2. **Verify checklist** before marking UI tasks complete
3. **Reject designs** that violate core principles (mouse-first, low density, AI-replacement, speed reduction)
4. **Prioritize keyboard shortcuts** in all task implementations
5. **Question requirements** that add friction or reduce information density

**When in doubt:** Ask "Would a Superhuman power user find this fast and efficient?"
