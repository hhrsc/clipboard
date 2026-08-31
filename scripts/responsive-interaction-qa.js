async (page) => {
  const results = [];
  const failures = [];
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  const check = (name, passed, detail = '') => {
    results.push({ name, passed, detail });
    if (!passed) failures.push({ name, detail });
  };
  const inspect = async (selector, name) => {
    const state = await page.locator(selector).evaluate(element => {
      const rect = element.getBoundingClientRect();
      return {
        fits: rect.left >= 0 && rect.right <= innerWidth && rect.top >= 0 && rect.bottom <= innerHeight,
        hit: element.contains(document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2)),
        bounds: [rect.x, rect.y, rect.width, rect.height]
      };
    });
    check(name, state.fits && state.hit, state);
  };
  for (const [width, height] of [[1586, 992], [1366, 768], [800, 600], [640, 480], [360, 640], [360, 400], [1586, 500]]) {
    await page.setViewportSize({ width, height });
    const size = `${width}x${height}`;
    await page.goto('http://127.0.0.1:1420/?reference=recent');
    await page.locator('.reference-mode').waitFor();
    if (width <= 1100) {
      await page.locator('.navigation-toggle').click();
      await inspect('.ref-sidebar', `${size} drawer`);
      await page.locator('.ref-collection-list button').last().click({ trial: true });
      await inspect('.ref-collection-list button:last-child', `${size} last sidebar collection reachable`);
      await page.screenshot({ path: `output/playwright/responsive-drawer-${size}.png`, type: 'png' });
      await page.locator('.ref-primary-nav').getByRole('button', { name: 'Images', exact: true }).click();
      check(`${size} drawer navigation`, await page.locator('.images-page').count() === 1 && !await page.locator('.ref-sidebar').isVisible());
      await page.locator('.navigation-toggle').click();
      await page.keyboard.press('Escape');
      check(`${size} drawer Escape`, !await page.locator('.ref-sidebar').isVisible());
      await page.locator('.navigation-toggle').click();
      await page.locator('.ref-collections-head button').click();
      await inspect('.collection-dialog', `${size} collection dialog`);
      await page.locator('.collection-dialog').getByRole('button', { name: 'Cancel' }).click();
      await page.locator('.drawer-shade').click({ position: { x: width - 15, y: height / 2 } });
      check(`${size} drawer backdrop`, !await page.locator('.ref-sidebar').isVisible());
    }
    await page.goto('http://127.0.0.1:1420/?reference=recent');
    await page.locator('.select-button').click();
    await inspect('.collection-picker', `${size} collection picker`);
    await page.locator('.collection-picker').evaluate(element => { element.scrollTop = element.scrollHeight; });
    await page.locator('.collection-picker').getByRole('button', { name: 'Ideas', exact: true }).click({ timeout: 3000 });
    check(`${size} last collection selectable`, await page.locator('.select-button span').textContent() === 'Ideas');
    await page.locator('.delete-clip').click({ trial: true });
    await inspect('.delete-clip', `${size} delete clip reachable`);
    await page.locator('.square-more').click();
    await inspect('.quick-menu', `${size} quick menu`);
    await page.screenshot({ path: `output/playwright/responsive-recent-actions-${size}.png`, type: 'png' });
    await page.goto('http://127.0.0.1:1420/?reference=images');
    await page.locator('.date-filter').click();
    await inspect('.date-menu', `${size} date menu`);
    await page.locator('.date-menu').getByRole('button', { name: 'Yesterday', exact: true }).click();
    await page.locator('.image-action').last().click({ trial: true });
    await inspect('.image-action:last-of-type', `${size} clear images reachable`);
    await page.goto('http://127.0.0.1:1420/?reference=passwords');
    await page.locator('.password-filter-button').click();
    await inspect('.password-filter-menu', `${size} password filter`);
    await page.locator('.password-filter-menu').getByRole('button', { name: 'Title A–Z' }).click();
    await page.locator('.row-more').first().click();
    await inspect('.password-row-menu', `${size} password row menu`);
    await page.keyboard.press('Escape');
    await page.locator('.password-detail .icon-actions').getByRole('button', { name: 'More', exact: true }).click();
    await inspect('.password-menu', `${size} password detail menu`);
    await page.keyboard.press('Escape');
    await page.locator('.save-changes').click({ trial: true });
    await inspect('.save-changes', `${size} save password reachable`);
    await page.screenshot({ path: `output/playwright/responsive-password-actions-${size}.png`, type: 'png' });
    await page.goto('http://127.0.0.1:1420/?reference=settings');
    await page.locator('.shortcut-control').click();
    await inspect('.shortcut-editor', `${size} shortcut editor`);
    await page.locator('.shortcut-editor input').press('Control+Shift+J');
    check(`${size} shortcut recorded`, await page.locator('.shortcut-editor input').inputValue() === 'Ctrl+Shift+J');
    await page.screenshot({ path: `output/playwright/responsive-shortcut-${size}.png`, type: 'png' });
    await page.keyboard.press('Escape');
    await page.locator('.danger-controls input').click();
    await inspect('.danger-controls', `${size} reset controls reachable`);
  }
  check('No frontend runtime errors', errors.length === 0, errors);
  if (failures.length) throw new Error(JSON.stringify(failures));
  return { checks: results.length, failures };
}
