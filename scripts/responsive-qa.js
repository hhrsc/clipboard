async (page) => {
  const results = [];
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  const sizes = [[1586, 992], [1366, 768], [1280, 720], [1024, 768], [800, 600], [640, 480], [360, 640], [360, 400], [1586, 500], [800, 1200], [1920, 1080], [2560, 1440], [760, 700], [761, 700], [1100, 768], [1101, 768], [1585, 992], [1699, 1099], [1700, 1100]];
  for (const name of ['recent', 'images', 'passwords', 'settings']) {
    await page.goto(`http://127.0.0.1:1420/?reference=${name}`);
    await page.locator('.reference-mode').waitFor();
    await page.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all([...document.images].map(image => image.decode().catch(() => {})));
    });
    for (const [width, height] of sizes) {
      await page.setViewportSize({ width, height });
      await page.mouse.move(width - 3, height - 3);
      await page.evaluate(() => {
        for (const element of document.querySelectorAll('main, section, .clip-scroll, .password-scroll, .image-scroll')) element.scrollTop = 0;
      });
      const layout = await page.evaluate(() => {
        const visible = element => element.getClientRects().length && getComputedStyle(element).visibility !== 'hidden';
        const overflow = [...document.querySelectorAll('.ref-main, .clip-detail, .password-detail, .image-detail, .settings-content, .ref-topbar, .password-row, .image-tools, .clip-control-row, .quick-actions, .danger-controls')].filter(visible).flatMap(element => {
          const bounds = element.getBoundingClientRect();
          // 基准设置页分隔线按参考图比内容区宽 68px，但没有越出窗口。
          const referenceDivider = element.matches('.settings-content') && innerWidth >= 1586 && innerWidth < 1700 && innerHeight >= 992 && innerHeight < 1100;
          return bounds.left < -1 || bounds.right > innerWidth + 1 || (!referenceDivider && element.scrollWidth > element.clientWidth + 2)
            ? [{ element: element.className, left: bounds.left, right: bounds.right, client: element.clientWidth, scroll: element.scrollWidth }] : [];
        });
        return { overflow, rootOverflow: document.documentElement.scrollWidth > innerWidth, dpr: devicePixelRatio, zoom: visualViewport.scale };
      });
      await page.screenshot({ path: `output/playwright/responsive-${name}-${width}x${height}.png`, type: 'png' });
      results.push({ page: name, width, height, ...layout });
    }
  }
  const failures = results.filter(result => result.rootOverflow || result.overflow.length || result.dpr !== 1 || result.zoom !== 1);
  if (failures.length || errors.length) throw new Error(JSON.stringify({ failures, errors }));
  return { cases: results.length, failures, errors };
}
