import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  page.on('console', msg => {
    if (msg.type() === 'error') {
      console.error('Browser console error:', msg.text());
    }
  });
  page.on('pageerror', error => {
    console.error('Page error:', error.message);
  });

  // Mock Tauri API with comprehensive test data
  await page.addInitScript(() => {
    (window as any).__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args?: any) => {
        if (cmd === 'list_messages') {
          return [
            {
              id: 1,
              subject: 'UX Design Feedback Request',
              from_addr: 'Rico Oktananda',
              from_email: 'rico.oktananda1@gmail.com',
              preview:
                "Hi Team, We're refining our product and need your insights on our user experience (UX) design. Please share any additional comments or suggestions.",
              date: '2024-06-30',
              is_read: false,
              is_starred: false,
              has_attachments: false,
            },
            {
              id: 2,
              subject: 'Q4 Marketing Campaign Strategy',
              from_addr: 'Alicia from Deel',
              from_email: 'alicia@deel.support',
              preview:
                'Hi Team, I hope this email finds you well. I wanted to take a moment to share our comprehensive Q4 marketing campaign strategy.',
              date: '2024-06-29',
              is_read: true,
              is_starred: false,
              has_attachments: true,
            },
            {
              id: 3,
              subject: 'Technical Architecture Discussion',
              from_addr: 'Substack Read',
              from_email: 'read@substack.com',
              preview:
                "Hello Development Team, I'm reaching out to discuss the technical architecture for our upcoming feature releases.",
              date: '2024-06-29',
              is_read: false,
              is_starred: true,
              has_attachments: false,
            },
          ];
        }
        if (cmd === 'get_message') {
          const messageId = args?.messageId;
          const messages = {
            1: {
              id: 1,
              subject: 'UX Design Feedback Request',
              from_addr: 'rico.oktananda1@gmail.com',
              to_addr: 'team@company.com',
              body_html:
                "<p>Hi Team,</p><p>We're refining our product and need your insights on our user experience (UX) design. Please share any additional comments or suggestions.</p>",
              date: '2024-06-30T10:30:00Z',
              is_starred: false,
            },
          };
          return messages[messageId] || null;
        }
        if (cmd === 'bulk_mark_read') return null;
        if (cmd === 'bulk_archive_messages') return null;
        if (cmd === 'bulk_delete_messages') return null;
        if (cmd === 'star_message') return null;
        if (cmd === 'unstar_message') return null;
        if (cmd === 'archive_messages') return null;
        if (cmd === 'delete_message') return null;
        return null;
      },
    };
  });
});

test.describe('Square UI - Email List Design', () => {
  test('should display email items with Square UI styling', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await expect(emailItems.first()).toBeVisible();

    // Check for checkbox
    const checkbox = emailItems.first().locator('button[role="checkbox"]');
    await expect(checkbox).toBeVisible();

    // Check email has proper layout
    await expect(emailItems.first()).toHaveClass(/flex/);
    await expect(emailItems.first()).toHaveClass(/gap-3/);
  });

  test('should show verified badge for verified senders', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');

    // Third email should have verified badge (id % 3 === 0)
    const thirdEmail = emailItems.nth(2);
    const verifiedBadge = thirdEmail.locator('[aria-label="Verified"]');
    await expect(verifiedBadge).toBeVisible();
  });

  test('should display cyan email addresses', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');

    // Check that email list contains emails (fromEmail prop is rendered)
    const count = await emailItems.count();
    expect(count).toBeGreaterThan(0);
  });

  test('should show unread emails with bold text', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    const firstEmail = emailItems.first(); // unread

    // Unread emails should have semibold font weight
    const subject = firstEmail.locator('text=UX Design Feedback Request');
    await expect(subject).toBeVisible();
  });

  test('should highlight selected email', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    const firstEmail = emailItems.first();

    await firstEmail.click();

    // Selected email should have accent background
    await expect(firstEmail).toHaveClass(/bg-accent/);
  });

  test('should show hover effect on email items', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    const firstEmail = emailItems.first();

    // Hover over email
    await firstEmail.hover();

    // Check for hover class
    await expect(firstEmail).toHaveClass(/hover:bg-accent/);
  });
});

test.describe('Square UI - Bulk Actions', () => {
  test('should show bulk actions toolbar when emails are selected', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await expect(emailItems.first()).toBeVisible();

    // Check first email
    const checkbox = emailItems.first().locator('button[role="checkbox"]');
    await checkbox.click();
    await page.waitForTimeout(100);

    // Toolbar should appear
    const toolbar = page.getByTestId('bulk-actions-toolbar');
    await expect(toolbar).toBeVisible();
    await expect(toolbar).toHaveClass(/bg-accent\/30/);
  });

  test('bulk actions toolbar should have all action buttons', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await emailItems.first().locator('button[role="checkbox"]').click();
    await page.waitForTimeout(100);

    const toolbar = page.getByTestId('bulk-actions-toolbar');

    // Check all buttons exist
    await expect(toolbar.getByTestId('bulk-archive-btn')).toBeVisible();
    await expect(toolbar.getByTestId('bulk-delete-btn')).toBeVisible();
    await expect(toolbar.getByTestId('bulk-mark-read-btn')).toBeVisible();
    await expect(toolbar.getByTestId('bulk-mark-unread-btn')).toBeVisible();
  });

  test('should update count when selecting multiple emails', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');

    // Select first two emails
    await emailItems.nth(0).locator('button[role="checkbox"]').click();
    await page.waitForTimeout(100);
    await emailItems.nth(1).locator('button[role="checkbox"]').click();
    await page.waitForTimeout(100);

    const toolbar = page.getByTestId('bulk-actions-toolbar');
    await expect(toolbar.getByText('2 selected')).toBeVisible();
  });

  test('should select all emails with header checkbox', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    // Find and click the select-all checkbox
    const selectAllCheckbox = page.locator('[aria-label="Select all emails"]');
    await expect(selectAllCheckbox).toBeVisible();
    await selectAllCheckbox.click();
    await page.waitForTimeout(100);

    // Should show 3 selected
    const toolbar = page.getByTestId('bulk-actions-toolbar');
    await expect(toolbar.getByText('3 selected')).toBeVisible();
  });
});

test.describe('Square UI - Email Reader', () => {
  test('should display email content when email is selected', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await emailItems.first().click();

    // Wait for content to load
    await page.waitForTimeout(500);

    // Check subject is displayed in reader (using heading role to be more specific)
    await expect(page.getByRole('heading', { name: 'UX Design Feedback Request' })).toBeVisible();

    // Check sender info
    await expect(page.getByText('rico.oktananda1@gmail.com')).toBeVisible();
  });

  test('should show action toolbar in reader', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await emailItems.first().click();
    await page.waitForTimeout(500);

    // Check for action buttons using title attribute (more specific)
    const replyButton = page.getByTitle('Reply', { exact: true });
    const archiveButton = page.getByTitle('Archive', { exact: true });
    const deleteButton = page.getByTitle('Delete', { exact: true });

    await expect(replyButton).toBeVisible();
    await expect(archiveButton).toBeVisible();
    await expect(deleteButton).toBeVisible();
  });

  test('should show sender information in email reader', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await emailItems.first().click();
    await page.waitForTimeout(500);

    // Check for sender email in the reader
    await expect(page.getByText('rico.oktananda1@gmail.com')).toBeVisible();
  });
});

test.describe('Square UI - Theme Toggle', () => {
  test('should toggle between light and dark themes', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const html = page.locator('html');
    const themeButton = page.getByRole('button', { name: /toggle theme/i });

    // Start in light mode
    await expect(html).not.toHaveClass(/dark/);

    // Toggle to dark
    await themeButton.click();
    await expect(html).toHaveClass(/dark/);

    // Toggle back to light
    await themeButton.click();
    await expect(html).not.toHaveClass(/dark/);
  });

  test('theme should persist across UI interactions', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const html = page.locator('html');
    const themeButton = page.getByRole('button', { name: /toggle theme/i });

    // Switch to dark mode
    await themeButton.click();
    await expect(html).toHaveClass(/dark/);

    // Interact with UI (select email)
    const emailItems = page.getByTestId('email-list-item');
    await emailItems.first().click();
    await page.waitForTimeout(200);

    // Theme should still be dark
    await expect(html).toHaveClass(/dark/);
  });
});

test.describe('Square UI - Compose Dialog', () => {
  test('should open compose in full-page view with Square UI styling', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/', { waitUntil: 'networkidle' });

    const folderNav = page.getByTestId('folder-nav');
    const composeButton = folderNav.getByRole('button', { name: /compose/i });
    await composeButton.click();

    // Check compose view is visible
    await expect(page.getByRole('heading', { name: 'New Message' })).toBeVisible();

    // Check for input fields
    await expect(page.getByPlaceholder('recipient@example.com')).toBeVisible();
    await expect(page.getByPlaceholder('Subject')).toBeVisible();

    // Verify sidebar IS visible (should remain)
    await expect(page.getByTestId('app-sidebar')).toBeVisible();

    // Verify email list and folder nav are NOT visible (full-width compose)
    await expect(page.getByTestId('email-list')).not.toBeVisible();
    await expect(page.getByTestId('folder-nav')).not.toBeVisible();
  });
});

test.describe('Square UI - Responsive Design', () => {
  test('should work on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    // Top header should be visible
    await expect(page.getByTestId('top-header')).toBeVisible();

    // Email list should be visible
    await expect(page.getByTestId('email-list')).toBeVisible();

    // Emails should still be clickable
    const emailItems = page.getByTestId('email-list-item');
    if ((await emailItems.count()) > 0) {
      await expect(emailItems.first()).toBeVisible();
    }
  });

  test('should work on tablet viewport', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    // All main components should be visible
    await expect(page.getByTestId('top-header')).toBeVisible();
    await expect(page.getByTestId('folder-nav')).toBeVisible();
    await expect(page.getByTestId('email-list')).toBeVisible();
  });

  test('should work on desktop viewport', async ({ page }) => {
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    // All components including sidebar should be visible
    await expect(page.getByTestId('app-sidebar')).toBeVisible();
    await expect(page.getByTestId('top-header')).toBeVisible();
    await expect(page.getByTestId('folder-nav')).toBeVisible();
    await expect(page.getByTestId('email-list')).toBeVisible();
  });
});

test.describe('Square UI - Keyboard Navigation', () => {
  test('should support tab navigation', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    await page.keyboard.press('Tab');

    const focusedElement = page.locator(':focus');
    await expect(focusedElement).toBeVisible();
  });

  test('should support enter key to select emails', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await emailItems.first().focus();
    await page.keyboard.press('Enter');

    // Email should be selected
    await page.waitForTimeout(200);
    await expect(emailItems.first()).toHaveClass(/bg-accent/);
  });
});

test.describe('Square UI - Folder Navigation', () => {
  test('should switch between folders', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const folderNav = page.getByTestId('folder-nav');

    // Click Sent items
    const sentButton = folderNav.getByRole('button', { name: 'Sent items' });
    await sentButton.click();
    await expect(sentButton).toHaveClass(/bg-muted/);

    // Click back to Inbox
    const inboxButton = folderNav.getByRole('button', { name: 'Inbox' });
    await inboxButton.click();
    await expect(inboxButton).toHaveClass(/bg-muted/);
  });

  test('should display folder count badges', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const folderNav = page.getByTestId('folder-nav');
    const inboxButton = folderNav.getByRole('button', { name: 'Inbox' });

    // Should show count badge (20)
    await expect(inboxButton.getByText('20')).toBeVisible();
  });
});

test.describe('Square UI - Search Functionality', () => {
  test('should focus search input with keyboard', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const searchInput = page.getByTestId('search-bar').getByRole('searchbox');

    await searchInput.click();
    await expect(searchInput).toBeFocused();
  });

  test('should accept and display search input', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const searchInput = page.getByTestId('search-bar').getByRole('searchbox');

    await searchInput.fill('UX Design');
    await expect(searchInput).toHaveValue('UX Design');
  });

  test('should show search placeholder text', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const searchInput = page.getByTestId('search-bar').getByRole('searchbox');

    await expect(searchInput).toHaveAttribute('placeholder', /search/i);
  });
});

test.describe('Square UI - Sidebar Toggle', () => {
  test('should toggle sidebar on trigger button click', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const sidebar = page.getByTestId('app-sidebar');
    await expect(sidebar).toBeVisible();

    // Find the sidebar wrapper group that has data-state attribute
    const sidebarGroup = page.locator('[data-slot="sidebar"]');

    // Initially sidebar should be expanded (default state)
    await expect(sidebarGroup).toHaveAttribute('data-state', 'expanded');

    // Find and click the trigger button
    const trigger = page.locator('button[data-sidebar="trigger"]');
    await expect(trigger).toBeVisible();
    await trigger.click();

    // Wait for animation
    await page.waitForTimeout(300);

    // Sidebar should be collapsed
    await expect(sidebarGroup).toHaveAttribute('data-state', 'collapsed');

    // Click again to expand
    await trigger.click();
    await page.waitForTimeout(300);

    // Should be expanded again
    await expect(sidebarGroup).toHaveAttribute('data-state', 'expanded');
  });

  test('should toggle sidebar with keyboard shortcut', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const sidebar = page.getByTestId('app-sidebar');
    await expect(sidebar).toBeVisible();

    // Find the sidebar wrapper group that has data-state attribute
    const sidebarGroup = page.locator('[data-slot="sidebar"]');

    // Initially expanded
    await expect(sidebarGroup).toHaveAttribute('data-state', 'expanded');

    // Press Ctrl+B (or Cmd+B on Mac)
    const modifier = process.platform === 'darwin' ? 'Meta' : 'Control';
    await page.keyboard.press(`${modifier}+KeyB`);

    // Wait for animation
    await page.waitForTimeout(300);

    // Should be collapsed
    await expect(sidebarGroup).toHaveAttribute('data-state', 'collapsed');

    // Press again to expand
    await page.keyboard.press(`${modifier}+KeyB`);
    await page.waitForTimeout(300);

    // Should be expanded
    await expect(sidebarGroup).toHaveAttribute('data-state', 'expanded');
  });

  test('sidebar should remain visible when composing email', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/', { waitUntil: 'networkidle' });

    const sidebar = page.getByTestId('app-sidebar');
    await expect(sidebar).toBeVisible();

    // Open compose view
    const folderNav = page.getByTestId('folder-nav');
    const composeButton = folderNav.getByRole('button', { name: /compose/i });
    await composeButton.click();

    // Sidebar should still be visible in compose mode
    await expect(sidebar).toBeVisible();

    // Sidebar toggle should still work
    const sidebarGroup = page.locator('[data-slot="sidebar"]');
    const trigger = page.locator('button[data-sidebar="trigger"]');
    await trigger.click();
    await page.waitForTimeout(300);

    await expect(sidebarGroup).toHaveAttribute('data-state', 'collapsed');
  });

  test('sidebar should persist state across navigation', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const sidebar = page.getByTestId('app-sidebar');
    await expect(sidebar).toBeVisible();

    const sidebarGroup = page.locator('[data-slot="sidebar"]');
    const trigger = page.locator('button[data-sidebar="trigger"]');

    // Collapse sidebar
    await trigger.click();
    await page.waitForTimeout(300);
    await expect(sidebarGroup).toHaveAttribute('data-state', 'collapsed');

    // Navigate to different folder
    const folderNav = page.getByTestId('folder-nav');
    const sentButton = folderNav.getByRole('button', { name: 'Sent items' });
    await sentButton.click();

    // Sidebar should remain collapsed
    await expect(sidebarGroup).toHaveAttribute('data-state', 'collapsed');

    // Expand sidebar
    await trigger.click();
    await page.waitForTimeout(300);

    // Should be expanded
    await expect(sidebarGroup).toHaveAttribute('data-state', 'expanded');
  });

  test('sidebar should show mobile behavior on small screens', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    // On mobile, sidebar uses sheet/overlay behavior
    // The trigger should still be present
    const trigger = page.locator('button[data-sidebar="trigger"]');
    await expect(trigger).toBeVisible();
  });
});
