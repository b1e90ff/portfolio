import { expect, test } from '@playwright/test';

const ISLAND_PAGES = ['/de-DE/basecamp', '/de-DE/projects', '/de-DE/about', '/de-DE/experience', '/de-DE/contact'];

function collectErrors(page) {
    const errors = [];
    page.on('pageerror', (err) => errors.push(err.message));
    page.on('console', (msg) => { if (msg.type() === 'error') errors.push(msg.text()); });
    return errors;
}

async function openWorld(page, path) {
    await page.goto(path);
    await expect(page.locator('html')).toHaveClass(/world-ready/, { timeout: 60_000 });
    await expect(page.locator('[data-loading]')).toBeHidden({ timeout: 60_000 });
    await page.evaluate(() => { window.__sameDocument = true; });
}

async function expectSameDocument(page) {
    expect(await page.evaluate(() => window.__sameDocument)).toBe(true);
}

test('overview renders the archipelago without errors', async ({ page }) => {
    const errors = collectErrors(page);
    await openWorld(page, '/de-DE');
    await expect(page.getByRole('heading', { level: 1 })).toContainText('Niklas');
    await expect(page.locator('[data-pin="projects"]')).toBeVisible();
    expect(errors).toEqual([]);
});

test('dock, project modal and Escape navigate in place', async ({ page }) => {
    const errors = collectErrors(page);
    await openWorld(page, '/de-DE');
    await page.locator('.dock a[data-island="projects"]').click();
    await expect(page).toHaveURL(/\/de-DE\/projects$/);
    await expect(page.locator('#panel-title')).toHaveText('Ausgewählte Arbeit');

    await page.locator('.projects a.project').first().click();
    await expect(page).toHaveURL(/\/de-DE\/projects\/[\w-]+$/);
    await expect(page.locator('[data-modal] #modal-title')).toBeVisible();

    await page.keyboard.press('Escape');
    await expect(page.locator('[data-modal]')).toHaveCount(0);
    await expect(page).toHaveURL(/\/de-DE\/projects$/);

    await page.keyboard.press('Escape');
    await expect(page).toHaveURL(/\/de-DE$/);
    await expect(page.locator('[data-panel]')).toHaveCount(0);
    await expectSameDocument(page);
    expect(errors).toEqual([]);
});

test('browser history restores the previous island', async ({ page }) => {
    await openWorld(page, '/de-DE/about');
    await page.locator('.dock a[data-island="experience"]').click();
    await expect(page).toHaveURL(/\/experience$/);
    await page.goBack();
    await expect(page).toHaveURL(/\/about$/);
    await expect(page.locator('[data-panel="about"]')).toBeVisible();
    await expectSameDocument(page);
});

test('contact form reports a delivery failure when mail is not configured', async ({ page }) => {
    await openWorld(page, '/de-DE/contact');
    await page.locator('#name').fill('Ada Lovelace');
    await page.locator('#email').fill('ada@example.com');
    await page.locator('#subject').fill('Projektanfrage');
    await page.locator('#message').fill('Hallo Niklas, ich habe ein Projekt im Kopf.');
    const response = page.waitForResponse('**/api/contact');
    await page.locator('[data-contact-submit]').click();
    expect((await response).status()).toBe(503);
    await expect(page.locator('[data-contact-error]')).toBeVisible();
});

test('island pages never overflow sideways', async ({ page }) => {
    for (const path of ISLAND_PAGES) {
        await page.goto(path);
        const overflow = await page.evaluate(() => ['body', '#panel-scroll'].map((sel) => {
            const el = document.querySelector(sel);
            return el.scrollWidth - el.clientWidth;
        }));
        expect(Math.max(...overflow), path).toBeLessThanOrEqual(0);
    }
});

test('the panel collapses on narrow screens only', async ({ page, isMobile }) => {
    await openWorld(page, '/de-DE/about');
    const toggle = page.locator('[data-panel-toggle]');
    if (!isMobile) {
        await expect(toggle).toBeHidden();
        return;
    }
    await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'false');
    await expect(page.locator('#panel-scroll')).toBeHidden();
    await toggle.click();
    await expect(page.locator('#panel-scroll')).toBeVisible();
});

test.describe('when the OS asks for less motion', () => {
    test.use({ reducedMotion: 'reduce' });

    test('nothing gets switched off', async ({ page }) => {
        await openWorld(page, '/de-DE');
        const world = page.locator('#world');
        const shot = () => world.screenshot({ animations: 'disabled', scale: 'css' });
        // The canvas keeps settling briefly after load, so one differing pair proves nothing.
        let previous = await shot();
        for (let i = 0; i < 4; i++) {
            await page.waitForTimeout(500);
            const current = await shot();
            expect(current.equals(previous), `frame ${i + 1} equals the one before`).toBe(false);
            previous = current;
        }
        const duration = await page.locator('.lang a').first().evaluate((el) => getComputedStyle(el).transitionDuration);
        expect(duration).not.toMatch(/^0s(, 0s)*$/);
        expect(await page.locator('.pin-dot').first().evaluate((el) => getComputedStyle(el).animationName)).toBe('pulse');

        await page.evaluate(() => {
            window.__panelEntered = false;
            new MutationObserver((records) => {
                if (records.some((r) => r.target.classList?.contains('is-entering'))) window.__panelEntered = true;
            }).observe(document.querySelector('main'), { subtree: true, attributes: true, attributeFilter: ['class'] });
        });
        await page.locator('.dock a[data-island="about"]').click();
        await expect(page.locator('[data-panel]')).toBeVisible();
        expect(await page.evaluate(() => window.__panelEntered)).toBe(true);
    });
});

test.describe('without JavaScript', () => {
    test.use({ javaScriptEnabled: false });

    test('pages stay readable and navigable', async ({ page }) => {
        await page.goto('/de-DE/about');
        await expect(page.locator('#panel-title')).toHaveText('Niklas Tat');
        await expect(page.locator('.pins')).toBeHidden();
        await page.locator('.dock a[data-island="contact"]').click();
        await expect(page).toHaveURL(/\/de-DE\/contact$/);
        await expect(page.locator('[data-contact-form]')).toBeVisible();
    });
});
