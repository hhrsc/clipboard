async (page) => {
  const results = [];
  const errors = [];
  await page.evaluate(() => sessionStorage.removeItem('responsive-vault'));
  page.on('pageerror', error => errors.push(error.message));
  const check = (name, passed) => {
    if (!passed) throw new Error(name);
    results.push(name);
  };
  // 真实页面使用合成 IPC；不连接桌面进程，不读取历史或密码文件。
  await page.addInitScript(() => {
    window.isTauri = true;
    window.unexpectedResponsiveCommands = [];
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' } },
      async invoke(command) {
        switch (command) {
          case 'app_reset_status': return false;
          case 'vault_status': return { exists: sessionStorage.getItem('responsive-vault') === 'exists', unlocked: false };
          case 'clipboard_store_status': return { exists: true };
          case 'clipboard_store_load': return {
            version: 1, categories: [{ id: 'all', name: '全部' }], records: [],
            preferences: { captureEnabled: false, retentionHours: 24, officialWebsite: 'https://example.com' }
          };
          case 'clipboard_store_replace': return;
          case 'autostart_status': return false;
          case 'get_clipboard_snapshot': return null;
          case 'shortcut_status': return { shortcut: 'Alt+C' };
          default:
            window.unexpectedResponsiveCommands.push(command);
            throw new Error(`Unexpected mock command: ${command}`);
        }
      }
    };
  });
  const navigate = async name => {
    await page.locator('.ref-window').waitFor();
    if (page.viewportSize().width <= 1100) await page.locator('.navigation-toggle').click();
    await page.locator('.ref-primary-nav').getByRole('button', { name, exact: true }).click();
  };
  const reachable = async (selector, name) => {
    const element = page.locator(selector);
    await element.click({ trial: true });
    check(name, await element.evaluate(node => {
      const rect = node.getBoundingClientRect();
      return rect.left >= 0 && rect.right <= innerWidth && rect.top >= 0 && rect.bottom <= innerHeight
        && node.contains(document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2));
    }));
  };
  for (const [width, height] of [[1586, 992], [800, 600], [360, 640], [360, 400]]) {
    const size = `${width}x${height}`;
    await page.setViewportSize({ width, height });
    await page.goto('http://127.0.0.1:1420/');
    await page.locator('.clip-column .empty-state').waitFor();
    check(`${size} real entry, empty Characters`, !await page.locator('.reference-mode').count());
    await navigate('Images');
    await page.locator('.image-empty-detail').waitFor();
    check(`${size} empty Images`, await page.locator('.image-library .empty-state').isVisible());
    await navigate('Passwords');
    await page.locator('.vault-gate').waitFor();
    await reachable('.vault-gate .primary-btn', `${size} vault submit reachable`);
    await page.screenshot({ path: `output/playwright/responsive-vault-gate-${size}.png`, type: 'png' });
    await navigate('Settings');
    await page.locator('.native-settings summary').click();
    await reachable('.native-setting input', `${size} retention reachable`);
    await reachable('[aria-label="Toggle launch at startup"]', `${size} startup reachable`);
    await page.screenshot({ path: `output/playwright/responsive-native-settings-${size}.png`, type: 'png' });
    check(`${size} no unexpected IPC`, await page.evaluate(() => window.unexpectedResponsiveCommands.length === 0));
  }
  await page.evaluate(() => sessionStorage.setItem('responsive-vault', 'exists'));
  await page.reload();
  await navigate('Passwords');
  await page.getByRole('heading', { name: 'Unlock password vault' }).waitFor();
  await reachable('.vault-gate input', 'Existing vault unlock input reachable at minimum size');
  check('No frontend runtime errors', errors.length === 0);
  return { checks: results.length, failures: [], scope: 'Real UI with synthetic IPC only; desktop acceptance pending.' };
}
