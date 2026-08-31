async (page) => {
  await page.goto('http://127.0.0.1:1420/?reference=recent');
  await page.locator('.reference-mode').waitFor();
  await page.evaluate(async () => { await document.fonts.ready; });
  const geometry = await page.evaluate(() => ({
    width: innerWidth, height: innerHeight, dpr: devicePixelRatio,
    scale: visualViewport.scale, zoom: getComputedStyle(document.documentElement).zoom
  }));
  if (JSON.stringify(geometry) !== JSON.stringify({ width: 1586, height: 992, dpr: 1, scale: 1, zoom: '1' })) {
    throw new Error(`Unexpected viewport: ${JSON.stringify(geometry)}`);
  }
  const trigger = page.locator('.clip-control-row .select-button');
  await trigger.click();
  const options = await page.locator('.collection-picker > button').allTextContents();
  const results = [];
  for (const name of [...options.slice(1), options[0]]) {
    if (!await page.locator('.collection-picker').count()) await trigger.click();
    // 仅替换测试页的正文 DOM，不读取或写入真实历史。
    await page.evaluate(() => {
      document.querySelector('.clip-content > span').textContent = '/* Collection menu regression fixture */\nbutton {\n  border: none;\n  display: flex;\n  padding: 0.75rem 1.5rem;\n  color: #fff;\n  font-size: 16px;\n  line-height: 1;\n}\n'.repeat(4);
    });
    const option = page.locator('.collection-picker').getByRole('button', { name, exact: true });
    const hit = await option.evaluate(button => {
      const rect = button.getBoundingClientRect();
      return button.contains(document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2));
    });
    if (!hit) throw new Error(`Collection option is covered: ${name}`);
    if (!results.length) {
      await page.mouse.move(1580, 985);
      await page.screenshot({ path: 'output/playwright/collection-after.png', type: 'png' });
    }
    await option.click();
    if (await page.locator('.collection-picker').count()) throw new Error(`Menu did not close: ${name}`);
    const selected = await trigger.locator('span').textContent();
    if (selected !== name) throw new Error(`Expected ${name}, received ${selected}`);
    results.push({ collection: name, hitTest: true, selected: true, menuClosed: true });
  }
  await page.screenshot({ path: 'output/playwright/collection-selected.png', type: 'png' });
  await page.mouse.move(1580, 985);
  const closed = await page.screenshot({ path: 'output/playwright/collection-closed-after.png', type: 'png' });
  const label = page.locator('.clip-control-row > label');
  await label.evaluate(element => { element.style.zIndex = 'auto'; });
  let closedUnchanged;
  try {
    const original = await page.screenshot({ path: 'output/playwright/collection-closed-before.png', type: 'png' });
    closedUnchanged = closed.equals(original);
  } finally {
    await label.evaluate(element => { element.style.removeProperty('z-index'); });
  }
  if (!closedUnchanged) throw new Error('Closed-menu appearance changed');
  return { geometry, results, closedUnchanged, scope: 'Reference fixture only; native persistence requires desktop acceptance.' };
}
