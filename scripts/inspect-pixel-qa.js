async (page) => {
  await page.goto('http://127.0.0.1:1420/?reference=recent');
  await page.locator('.reference-mode').waitFor();
  return await page.evaluate(() => ['.ref-window','.ref-brand','.clip-row','.primary-btn','.clip-content','.ref-sidebar','.ref-topbar','.clip-column','.clip-content-box','.quick-actions'].map(selector => {
    const element = document.querySelector(selector);
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return {selector, x: rect.x, y: rect.y, width: rect.width, height: rect.height, font: style.font, color: style.color, background: style.backgroundColor};
  }));
}
