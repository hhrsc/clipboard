async (page) => {
  const screenshots = [];
  for (const name of ['recent', 'images', 'passwords', 'settings']) {
    await page.setViewportSize({ width: 1586, height: 992 });
    await page.goto(`http://127.0.0.1:1420/?reference=${name}`);
    await page.locator('.reference-mode').waitFor();
    await page.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all([...document.images].map(image => image.decode().catch(() => {})));
    });
    await page.mouse.move(1580, 985);
    const path = `output/playwright/responsive-after-${name}.png`;
    await page.screenshot({ path, type: 'png' });
    screenshots.push(path);
  }
  return screenshots;
}
