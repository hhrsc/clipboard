async (page) => {
  const checks = [];
  const errors = [];
  const check = (name, passed) => {
    if (!passed) throw new Error(name);
    checks.push(name);
  };
  page.on('pageerror', error => errors.push(error.message));
  // 仅拦截独立浏览器中的 IPC，不创建或解锁实际密码库。
  await page.addInitScript(() => {
    window.isTauri = true;
    window.masterLengthQA = { calls: 0, length: 0, unlockLength: 0 };
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' } },
      async invoke(command, args) {
        switch (command) {
          case 'app_reset_status': return false;
          case 'vault_status': return { exists: sessionStorage.getItem('length-existing') === 'true', unlocked: false };
          case 'clipboard_store_status': return { exists: true };
          case 'clipboard_store_load': return {
            version: 1, categories: [{ id: 'all', name: '全部' }], records: [],
            preferences: { captureEnabled: false, retentionHours: 24, officialWebsite: 'https://example.com' }
          };
          case 'clipboard_store_replace': return;
          case 'autostart_status': return false;
          case 'get_clipboard_snapshot': return null;
          case 'shortcut_status': return { shortcut: 'Alt+C' };
          case 'vault_setup':
            window.masterLengthQA.calls++;
            window.masterLengthQA.length = Array.from(args.masterPassword).length;
            throw 'QA: setup intercepted';
          case 'vault_unlock':
            window.masterLengthQA.unlockLength = Array.from(args.masterPassword).length;
            throw 'QA: unlock intercepted';
          default: throw new Error(`Unexpected mock command: ${command}`);
        }
      }
    };
  });
  await page.evaluate(() => sessionStorage.removeItem('length-existing'));
  const navigate = async () => {
    await page.goto('http://127.0.0.1:1420/');
    await page.locator('.ref-window').waitFor();
    if (page.viewportSize().width <= 1100) await page.locator('.navigation-toggle').click();
    await page.locator('.ref-primary-nav').getByRole('button', { name: 'Passwords', exact: true }).click();
  };
  await navigate();
  const master = page.getByLabel('Master password', { exact: true });
  const confirm = page.getByLabel('Confirm master password', { exact: true });
  const submit = page.getByRole('button', { name: 'Create vault', exact: true });
  check('Creation fields show 8–16 character hint', await master.getAttribute('placeholder') === '8–16 characters');
  await page.evaluate(async () => { await document.fonts.ready; });
  await page.screenshot({ path: 'output/playwright/master-length-1586x992.png', type: 'png' });
  const fieldBounds = await page.locator('.vault-gate input').evaluateAll(inputs => inputs.map(input => {
    const { x, y, width, height } = input.getBoundingClientRect();
    input.removeAttribute('placeholder');
    return { x, y, width, height };
  }));
  await page.screenshot({ path: 'output/playwright/master-length-without-hint.png', type: 'png' });
  await page.locator('.vault-gate input').evaluateAll(inputs => inputs.forEach(input => input.setAttribute('placeholder', '8–16 characters')));
  for (const [name, value, valid] of [
    ['7 characters', 'a'.repeat(7), false], ['8 characters', 'a'.repeat(8), true],
    ['16 characters', 'a'.repeat(16), true], ['17 characters', 'a'.repeat(17), false],
    ['8 Chinese characters', '密'.repeat(8), true], ['8 non-BMP characters', '🔒'.repeat(8), true],
    ['16 non-BMP characters', '🔒'.repeat(16), true], ['17 non-BMP characters', '🔒'.repeat(17), false]
  ]) {
    await master.fill(value);
    await confirm.fill(value);
    const previous = await page.evaluate(() => window.masterLengthQA.calls);
    await submit.click();
    const state = await page.evaluate(() => window.masterLengthQA);
    check(name, state.calls === previous + Number(valid));
    check(`${name} feedback`, await page.locator('.vault-gate [role=alert]').textContent() === (valid ? 'QA: setup intercepted' : 'Master password must contain 8-16 characters.'));
  }
  await master.fill('12345678');
  await confirm.fill('abcdefgh');
  const beforeMismatch = await page.evaluate(() => window.masterLengthQA.calls);
  await submit.click();
  check('Confirmation still required to match', await page.evaluate(() => window.masterLengthQA.calls) === beforeMismatch && await page.locator('.vault-gate [role=alert]').textContent() === 'The master passwords do not match.');
  await page.setViewportSize({ width: 360, height: 400 });
  await navigate();
  await submit.click({ trial: true });
  await page.screenshot({ path: 'output/playwright/master-length-360x400.png', type: 'png' });
  check('Small window does not overflow horizontally', await page.evaluate(() => document.documentElement.scrollWidth === innerWidth));
  await page.evaluate(() => sessionStorage.setItem('length-existing', 'true'));
  await navigate();
  await master.fill('legacy master password longer than sixteen');
  await page.getByRole('button', { name: 'Unlock vault', exact: true }).click();
  check('Legacy unlock is not limited to 16 characters', await page.evaluate(() => window.masterLengthQA.unlockLength > 16));
  check('No frontend runtime errors', errors.length === 0);
  return { checks: checks.length, failures: [], fieldBounds, scope: 'Synthetic IPC; no real vault access.' };
}
