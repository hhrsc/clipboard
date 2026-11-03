async (page) => {
  const results = [];
  const errors = [];
  const check = (name, pass) => { if (!pass) throw new Error(name); results.push(name); };
  page.on('pageerror', error => errors.push(error.message));
  for (const tab of ['recent', 'images', 'passwords', 'settings']) {
    await page.goto(`http://127.0.0.1:1420/?reference=${tab}`);
    await page.locator('.reference-mode').waitFor();
    await page.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all([...document.images].map(image => image.decode().catch(() => {})));
    });
    await page.mouse.move(1580, 985);
    await page.screenshot({ path: `output/playwright/vault-access-reference-${tab}.png`, type: 'png' });
  }
  // 合成 IPC 和浏览器存储，不读写真实密码库、主密码、Windows 凭据或剪贴板。
  await page.addInitScript(() => {
    localStorage.clear();
    window.isTauri = true;
    window.qaCalls = [];
    window.qaFailSave = false;
    window.qaDeferSave = false;
    window.qaUnlocked = false;
    window.qaVault = {
      version: 2,
      passwords: [{ id: 1, title: 'QA vault entry', username: 'synthetic@example.test', password: 'entry-password', collectionId: 'work' }],
      collections: [{ id: 'work', name: 'QA work' }]
    };
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' } },
      async invoke(command, args) {
        window.qaCalls.push(command);
        switch (command) {
          case 'app_reset_status': return false;
          case 'vault_status': return { exists: sessionStorage.getItem('qa-no-vault') !== 'yes', unlocked: window.qaUnlocked, requirePassword: sessionStorage.getItem('qa-access-off') !== 'yes', autoUnlockAvailable: true };
          case 'vault_lock': window.qaUnlocked = false; return;
          case 'vault_set_require_password':
            if (args.masterPassword !== 'Access-QA-123') throw 'master password is incorrect';
            if (window.qaFailSave) throw 'Could not save local vault access: access denied';
            if (window.qaDeferSave) await new Promise(resolve => { window.qaReleaseSave = resolve; });
            sessionStorage.setItem('qa-access-off', args.requirePassword ? 'no' : 'yes');
            if (args.requirePassword) window.qaUnlocked = false;
            return;
          case 'vault_auto_unlock':
            if (sessionStorage.getItem('qa-access-off') !== 'yes') throw 'Password is required';
            if (sessionStorage.getItem('qa-damaged-key') === 'yes') throw 'Local key is unavailable';
            window.qaUnlocked = true;
            return structuredClone(window.qaVault);
          case 'vault_unlock':
            if (args.masterPassword !== 'Access-QA-123') throw 'master password is incorrect';
            window.qaUnlocked = true;
            return structuredClone(window.qaVault);
          case 'clipboard_store_status': return { exists: true };
          case 'clipboard_store_load': return { version: 1, categories: [{ id: 'all', name: '全部' }], records: [], preferences: { captureEnabled: false, retentionHours: 24, officialWebsite: 'https://example.com' } };
          case 'clipboard_store_replace': return;
          case 'autostart_status': return false;
          case 'shortcut_status': return { shortcut: 'Alt+C' };
          case 'get_clipboard_snapshot': return null;
          default: throw new Error(`Unexpected command: ${command}`);
        }
      }
    };
  });
  const nav = async name => {
    if (page.viewportSize().width <= 1100) await page.locator('.navigation-toggle').click();
    await page.locator('.ref-primary-nav').getByRole('button', { name, exact: true }).click();
  };
  const toggle = page.getByRole('button', { name: 'Require password to open password vault', exact: true });
  const dialog = page.getByRole('dialog', { name: 'Password vault access', exact: true });
  const fillAndConfirm = async password => {
    await dialog.getByLabel('Current master password').fill(password);
    await dialog.getByRole('button', { name: 'Confirm', exact: true }).click();
  };
  await page.goto('http://127.0.0.1:1420/');
  await nav('Settings');
  check('Default requires password', await toggle.getAttribute('aria-pressed') === 'true' && await toggle.isEnabled());
  await page.screenshot({ path: 'output/playwright/vault-access-settings-on.png', type: 'png' });
  await toggle.click();
  check('Confirmation uses focused masked password input', await dialog.getByLabel('Current master password').evaluate(input => input.type === 'password' && document.activeElement === input));
  check('Empty confirmation disabled', await dialog.getByRole('button', { name: 'Confirm', exact: true }).isDisabled());
  await dialog.getByRole('button', { name: 'Cancel', exact: true }).click();
  check('Cancel performs no native mutation', await page.evaluate(() => !window.qaCalls.includes('vault_set_require_password')));
  await toggle.click();
  await fillAndConfirm('wrong');
  await dialog.getByRole('alert').waitFor();
  check('Wrong password leaves requirement on', await toggle.getAttribute('aria-pressed') === 'true');
  await page.evaluate(() => { window.qaFailSave = true; });
  await fillAndConfirm('Access-QA-123');
  await dialog.getByText('Could not save local vault access: access denied', { exact: true }).waitFor();
  check('Failed write is shown, not treated as saved', await toggle.getAttribute('aria-pressed') === 'true');
  await page.evaluate(() => { window.qaFailSave = false; window.qaDeferSave = true; });
  await fillAndConfirm('Access-QA-123');
  await dialog.getByRole('button', { name: 'Saving…', exact: true }).waitFor();
  check('Pending save disables repeated submission', await dialog.getByRole('button', { name: 'Saving…', exact: true }).isDisabled() && await dialog.getByRole('button', { name: 'Cancel', exact: true }).isDisabled());
  await page.keyboard.press('Escape');
  check('Escape cannot discard a pending save', await dialog.isVisible());
  await page.evaluate(() => { window.qaReleaseSave(); });
  await dialog.waitFor({ state: 'hidden' });
  check('Successful save switches requirement off', await toggle.getAttribute('aria-pressed') === 'false');
  await page.screenshot({ path: 'output/playwright/vault-access-settings-off.png', type: 'png' });
  await nav('Passwords');
  await page.locator('.password-list').waitFor();
  check('Off opens vault without password entry', await page.locator('.vault-gate').count() === 0 && await page.locator('.password-row').count() === 1);
  check('Collections load during automatic unlock', await page.locator('.ref-collection-list').innerText().then(text => text.includes('QA work')));
  check('Manual lock is disabled in password-free mode', await page.locator('.sidebar-lock').isDisabled() && await page.locator('.lock-detail').isDisabled());
  await page.reload();
  check('Restart does not eagerly expose the vault outside Passwords', await page.evaluate(() => !window.qaCalls.includes('vault_auto_unlock')));
  await nav('Passwords');
  await page.locator('.password-list').waitFor();
  check('Off survives reload and opens automatically', await page.evaluate(() => window.qaCalls.filter(c => c === 'vault_auto_unlock').length === 1));
  await nav('Settings');
  await toggle.click();
  await fillAndConfirm('Access-QA-123');
  await dialog.waitFor({ state: 'hidden' });
  check('Re-enabling immediately locks native session', await toggle.getAttribute('aria-pressed') === 'true' && await page.evaluate(() => !window.qaUnlocked));
  await nav('Passwords');
  await page.locator('.vault-gate').waitFor();
  check('Re-enabled vault clears entries and collections from view', await page.locator('.password-row').count() === 0 && !await page.locator('.ref-collection-list').innerText().then(text => text.includes('QA work')));
  await page.locator('.vault-gate input').fill('Access-QA-123');
  await page.locator('.vault-gate button').click();
  await page.locator('.password-list').waitFor();
  check('Original master password remains valid', await page.locator('.sidebar-lock').isEnabled());
  await page.locator('.sidebar-lock').click();
  await page.locator('.vault-gate').waitFor();
  check('Manual locking still works when enabled', true);

  await page.evaluate(() => { sessionStorage.setItem('qa-access-off', 'yes'); sessionStorage.setItem('qa-damaged-key', 'yes'); });
  await page.reload();
  await nav('Passwords');
  await page.locator('.vault-gate [role=alert]').waitFor();
  check('Unavailable device key falls back to password, not empty vault', await page.locator('.vault-gate [role=alert]').innerText().then(text => text.includes('Automatic opening failed')));
  await nav('Settings');
  await nav('Passwords');
  check('Failed automatic unlock does not retry in a loop', await page.evaluate(() => window.qaCalls.filter(c => c === 'vault_auto_unlock').length === 1));
  await page.locator('.vault-gate input').fill('Access-QA-123');
  await page.locator('.vault-gate button').click();
  await page.locator('.password-list').waitFor();
  check('Password fallback preserves existing entries', await page.locator('.password-row').count() === 1);
  for (const [width, height] of [[800, 600], [360, 400]]) {
    await page.setViewportSize({ width, height });
    await nav('Settings');
    await toggle.click();
    await dialog.getByLabel('Current master password').fill('Access-QA-123');
    await dialog.getByRole('button', { name: 'Confirm', exact: true }).click({ trial: true });
    check(`${width}x${height}: confirmation reachable`, await dialog.locator('form').evaluate(node => {
      const r = node.getBoundingClientRect();
      return r.left >= 0 && r.top >= 0 && r.right <= innerWidth && r.bottom <= innerHeight;
    }));
    await dialog.getByLabel('Current master password').fill('');
    await page.screenshot({ path: `output/playwright/vault-access-dialog-${width}x${height}.png`, type: 'png' });
    await page.keyboard.press('Escape');
    check(`${width}x${height}: Escape dismisses idle confirmation`, !await dialog.count());
  }
  await page.evaluate(() => { sessionStorage.setItem('qa-no-vault', 'yes'); sessionStorage.removeItem('qa-access-off'); });
  await page.reload();
  await nav('Settings');
  check('Uncreated vault cannot accidentally enable no-password mode', await toggle.isDisabled() && await page.getByText('Create a password vault first to change this option.', { exact: true }).isVisible());
  check('No frontend runtime errors', errors.length === 0);
  return { checks: results.length, results, scope: 'Isolated browser with mocked native IPC; no real vault or master password used.' };
}
