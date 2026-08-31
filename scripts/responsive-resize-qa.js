async (page) => {
  const results = [];
  const check = (name, passed) => {
    if (!passed) throw new Error(name);
    results.push(name);
  };
  await page.setViewportSize({ width: 1586, height: 992 });
  await page.goto('http://127.0.0.1:1420/?reference=recent');
  await page.locator('.select-button').click();
  for (const [width, height] of [[800, 600], [360, 400], [1920, 1080], [1586, 992]]) {
    await page.setViewportSize({ width, height });
    await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    const state = await page.locator('.collection-picker').evaluate(menu => {
      const rect = menu.getBoundingClientRect();
      return { count: document.querySelectorAll('.collection-picker').length,
        fits: rect.left >= 0 && rect.right <= innerWidth && rect.top >= 0 && rect.bottom <= innerHeight,
        restored: menu.parentElement.matches('.clip-control-row > label') };
    });
    check(`${width}x${height} open menu fits without duplication`, state.count === 1 && state.fits);
    if (width === 1586) check('Baseline menu returns to original container', state.restored);
  }
  await page.keyboard.press('Escape');
  check('Closed menu leaves no floating DOM', await page.locator('.collection-picker').count() === 0);
  await page.setViewportSize({ width: 800, height: 600 });
  await page.locator('.navigation-toggle').click();
  check('Drawer prevents background keyboard focus', await page.locator('.ref-main').evaluate(main => main.inert));
  await page.setViewportSize({ width: 1586, height: 992 });
  await page.waitForFunction(() => !document.querySelector('.drawer-shade'));
  check('Widening restores main interaction', await page.locator('.ref-main').evaluate(main => !main.inert));
  check('Widening restores sidebar', await page.locator('.ref-sidebar').isVisible());
  return { checks: results.length, failures: [] };
}
