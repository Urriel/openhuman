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

  // Mock Tauri API for browser tests
  await page.addInitScript(() => {
    (window as any).__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args?: any) => {
        console.log('Mock Tauri invoke:', cmd, args);
        if (cmd === 'list_messages') {
          return [
            {
              id: 1,
              subject: 'Test Email 1',
              from_addr: 'sender1@example.com',
              preview: 'This is a test email preview',
              date: '2024-01-01',
              is_read: false,
              is_starred: false,
              has_attachments: false,
            },
            {
              id: 2,
              subject: 'Test Email 2',
              from_addr: 'sender2@example.com',
              preview: 'Another test email preview',
              date: '2024-01-02',
              is_read: true,
              is_starred: false,
              has_attachments: true,
            },
            {
              id: 3,
              subject: 'Test Email 3',
              from_addr: 'sender3@example.com',
              preview: 'Third test email preview',
              date: '2024-01-03',
              is_read: false,
              is_starred: true,
              has_attachments: false,
            },
          ];
        }
        if (cmd === 'bulk_mark_read') {
          return null;
        }
        if (cmd === 'bulk_archive_messages') {
          return null;
        }
        if (cmd === 'bulk_delete_messages') {
          return null;
        }
        return null;
      },
    };
  });
});

test.describe('Email Client Layout', () => {
  test('should display main layout components', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const topHeader = page.getByTestId('top-header');
    await expect(topHeader).toBeAttached();
    await expect(topHeader).toBeVisible();
    await expect(topHeader.getByText('Emails', { exact: true })).toBeVisible();

    // Icon sidebar is desktop-only.
    const iconSidebar = page.getByTestId('app-sidebar');
    await expect(iconSidebar).toBeVisible();

    const folderNav = page.getByTestId('folder-nav');
    await expect(folderNav).toBeVisible();
    await expect(folderNav.getByRole('button', { name: 'Inbox' })).toBeVisible();

    const searchBar = page.getByTestId('search-bar');
    await expect(searchBar).toBeVisible();

    const emailList = page.getByTestId('email-list');
    await expect(emailList).toBeVisible();

    await expect(page.getByText('Select an email to read')).toBeVisible();
  });

  test('should toggle theme', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const html = page.locator('html');
    const topHeader = page.getByTestId('top-header');

    await expect(html).not.toHaveClass(/dark/);

    const themeButton = topHeader.getByRole('button', { name: /toggle theme/i });
    await themeButton.click();
    await expect(html).toHaveClass(/dark/);

    await themeButton.click();
    await expect(html).not.toHaveClass(/dark/);
  });
});

test.describe('Folder Navigation', () => {
  test('should display folder buttons', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const folderNav = page.getByTestId('folder-nav');
    await expect(folderNav.getByRole('button', { name: 'Inbox' })).toBeVisible();
    await expect(folderNav.getByRole('button', { name: 'Sent items' })).toBeVisible();
    await expect(folderNav.getByRole('button', { name: 'Drafts' })).toBeVisible();
  });

  test('should select folders', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const folderNav = page.getByTestId('folder-nav');

    const sentButton = folderNav.getByRole('button', { name: 'Sent items' });
    await sentButton.click();
    await expect(sentButton).toHaveClass(/bg-muted/);

    const inboxButton = folderNav.getByRole('button', { name: 'Inbox' });
    await inboxButton.click();
    await expect(inboxButton).toHaveClass(/bg-muted/);
  });
});

test.describe('Email List', () => {
  test('should display search bar', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const searchBar = page.getByTestId('search-bar');
    await expect(searchBar.getByRole('searchbox')).toBeVisible();
  });

  test('should show empty state when no messages', async ({ page }) => {
    // Override the mock to return empty list for this test
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string) => {
          if (cmd === 'list_messages') return [];
          return null;
        },
      };
    });

    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const folderNav = page.getByTestId('folder-nav');
    await folderNav.getByRole('button', { name: 'Drafts' }).click();

    const emailList = page.getByTestId('email-list');
    await expect(emailList.getByText(/No messages found/i)).toBeVisible();
  });

  test('should display message items if available', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const emailItems = page.getByTestId('email-list-item');
    const count = await emailItems.count();

    if (count > 0) {
      await expect(emailItems.first()).toBeVisible();
    }
  });

  test('should select email and show in reader', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const emailItems = page.getByTestId('email-list-item');
    const count = await emailItems.count();

    if (count > 0) {
      await emailItems.first().click();
      await expect(page.getByText('Select an email to read')).not.toBeVisible();
    }
  });
});

test.describe('Email Reader', () => {
  test('should show empty state when no email selected', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    await expect(page.getByText('Select an email to read')).toBeVisible();
  });
});

test.describe('Compose Email', () => {
  test('should open compose in full-page view (desktop)', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/', { waitUntil: 'networkidle' });

    const folderNav = page.getByTestId('folder-nav');
    await folderNav.getByRole('button', { name: /compose/i }).click();

    // Verify compose view is visible
    await expect(page.getByPlaceholder('recipient@example.com')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'New Message' })).toBeVisible();

    // Verify sidebar IS visible (should remain)
    await expect(page.getByTestId('app-sidebar')).toBeVisible();

    // Verify email list and folder nav are NOT visible (full-width compose)
    await expect(page.getByTestId('email-list')).not.toBeVisible();
    await expect(page.getByTestId('folder-nav')).not.toBeVisible();
  });
});

test.describe('Search Functionality', () => {
  test('should focus search input', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const searchInput = page.getByTestId('search-bar').getByRole('searchbox');
    await searchInput.click();
    await expect(searchInput).toBeFocused();
  });

  test('should accept search input', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    const searchInput = page.getByTestId('search-bar').getByRole('searchbox');
    await searchInput.fill('test query');
    await expect(searchInput).toHaveValue('test query');
  });
});

test.describe('Responsive Design', () => {
  test('should render on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    // Top header should still be visible.
    await expect(page.getByTestId('top-header')).toBeVisible();

    // Folder nav is desktop-only; ensure email list column still shows.
    await expect(page.getByTestId('email-list')).toBeVisible();
  });
});

test.describe('Accessibility', () => {
  test('should have keyboard navigation support', async ({ page }) => {
    await page.goto('/', { waitUntil: 'domcontentloaded' });

    await page.keyboard.press('Tab');

    const focusedElement = page.locator(':focus');
    await expect(focusedElement).toBeVisible();
  });
});

test.describe('Bulk Actions', () => {
  test('should not show bulk actions toolbar when no items are checked', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const toolbar = page.getByTestId('bulk-actions-toolbar');
    await expect(toolbar).not.toBeVisible();
  });

  test('should show bulk actions toolbar when items are checked', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    // Wait for emails to load
    const emailItems = page.getByTestId('email-list-item');
    await expect(emailItems.first()).toBeVisible();

    // Check the first email using the button element within the checkbox
    const firstCheckbox = emailItems.first().locator('button[role="checkbox"]');
    await firstCheckbox.click();
    await page.waitForTimeout(100); // Small delay for state update

    // Toolbar should appear
    const toolbar = page.getByTestId('bulk-actions-toolbar');
    await expect(toolbar).toBeVisible();
    await expect(toolbar.getByText('1 selected')).toBeVisible();
  });

  test('should show correct count when multiple items are checked', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await expect(emailItems.first()).toBeVisible();

    // Check first two emails
    await emailItems.nth(0).locator('button[role="checkbox"]').click();
    await page.waitForTimeout(100);
    await emailItems.nth(1).locator('button[role="checkbox"]').click();
    await page.waitForTimeout(100);

    const toolbar = page.getByTestId('bulk-actions-toolbar');
    await expect(toolbar.getByText('2 selected')).toBeVisible();
  });

  test('should display all bulk action buttons', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await expect(emailItems.first()).toBeVisible();

    // Check an email
    await emailItems.first().locator('button[role="checkbox"]').click();
    await page.waitForTimeout(100);

    const toolbar = page.getByTestId('bulk-actions-toolbar');
    await expect(toolbar.getByTestId('bulk-archive-btn')).toBeVisible();
    await expect(toolbar.getByTestId('bulk-delete-btn')).toBeVisible();
    await expect(toolbar.getByTestId('bulk-mark-read-btn')).toBeVisible();
    await expect(toolbar.getByTestId('bulk-mark-unread-btn')).toBeVisible();
  });

  test('should hide checkbox header when items are checked', async ({ page }) => {
    await page.goto('/', { waitUntil: 'networkidle' });

    const emailItems = page.getByTestId('email-list-item');
    await expect(emailItems.first()).toBeVisible();

    // Initially, the checkbox header should be visible
    const emailList = page.getByTestId('email-list');
    const checkboxHeader = emailList
      .locator('div')
      .filter({ has: page.locator('[aria-label="Select all emails"]') })
      .first();
    await expect(checkboxHeader).toBeVisible();

    // Check an email
    await emailItems.first().locator('button[role="checkbox"]').click();
    await page.waitForTimeout(100);

    // Checkbox header should be hidden (toolbar replaces it)
    await expect(checkboxHeader).toBeHidden();
  });
});
