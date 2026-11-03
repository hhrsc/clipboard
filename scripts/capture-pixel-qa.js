async (page) => {
  const measurements = [];
  await page.setViewportSize({ width: 1586, height: 992 });
  for (const name of ['recent', 'images', 'passwords', 'settings']) {
    await page.goto(`http://127.0.0.1:1420/?reference=${name}`);
    await page.locator('.reference-mode').waitFor();
    await page.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all([...document.images].map(image => image.decode().catch(() => {})));
    });
    const geometry = await page.evaluate(() => ({
      viewport: [innerWidth, innerHeight], dpr: devicePixelRatio,
      zoom: getComputedStyle(document.documentElement).zoom,
      visualScale: visualViewport.scale,
      sidebar: document.querySelector('.ref-sidebar').getBoundingClientRect().width
    }));
    if (geometry.viewport.join('x') !== '1586x992' || geometry.dpr !== 1 || geometry.visualScale !== 1 || geometry.zoom !== '1') throw new Error(JSON.stringify(geometry));
    await page.mouse.move(1580, 985);
    await page.screenshot({ path: `design-qa/final/${name}/implementation.png`, type: 'png', fullPage: false, scale: 'css' });
    measurements.push({ page: name, ...geometry });
  }
  return measurements;
}
