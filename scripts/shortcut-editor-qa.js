async (page) => {
  const results = [];
  const check = (name, passed) => {
    if (!passed) throw new Error(name);
    results.push({ name, passed });
  };
  await page.evaluate(() => sessionStorage.removeItem('shortcut-qa'));
  // 模拟 IPC 只存在于隔离浏览器，不注册系统热键或访问 APP 文件。
  await page.addInitScript(() => {
    window.isTauri = true;
    window.shortcutQA = { failure: '', pending: null, requested: '', calls: 0 };
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' } },
      async invoke(command, args) {
        switch (command) {
          case 'app_reset_status': return false;
          case 'vault_status': return { exists: false, unlocked: false };
          case 'clipboard_store_status': return { exists: true };
          case 'clipboard_store_load': return {
            version: 1, categories: [{ id: 'all', name: '全部' }], records: [],
            preferences: { captureEnabled: false, retentionHours: 24, officialWebsite: 'https://example.com' }
          };
          case 'clipboard_store_replace': return;
          case 'autostart_status': return false;
          case 'get_clipboard_snapshot': return null;
          case 'shortcut_status': return { shortcut: sessionStorage.getItem('shortcut-qa') || 'Alt+C' };
          case 'update_global_shortcut': {
            window.shortcutQA.calls++;
            window.shortcutQA.requested = args.shortcut;
            if (window.shortcutQA.failure) throw window.shortcutQA.failure;
            await new Promise(resolve => { window.shortcutQA.pending = resolve; });
            sessionStorage.setItem('shortcut-qa', args.shortcut);
            return { shortcut: args.shortcut };
          }
          default: throw new Error(`Unexpected mock command: ${command}`);
        }
      }
    };
  });
  await page.goto('http://127.0.0.1:1420/');
  await page.locator('.ref-primary-nav').getByRole('button', { name: 'Settings', exact: true }).click();
  const control = page.locator('.shortcut-control');
  const input = page.locator('.shortcut-editor input');
  const apply = page.locator('.shortcut-editor').getByRole('button', { name: 'Apply', exact: true });
  const value = () => control.locator('kbd').allTextContents().then(keys => keys.join('+'));
  check('Loads confirmed native value', await value() === 'Alt+C');
  await control.click();
  check('Recorder focuses automatically and cannot be edited', await input.evaluate(element => element.readOnly && document.activeElement === element));
  check('Empty recording cannot be applied', await input.inputValue() === '' && await apply.isDisabled());
  await page.keyboard.insertText('Alt+V');
  check('Text insertion does not set a shortcut', await input.inputValue() === '');
  await page.keyboard.down('Control');
  check('Modifier press is displayed but cannot be applied', await input.inputValue() === 'Ctrl' && await apply.isDisabled());
  await page.keyboard.down('Shift');
  await page.keyboard.press('KeyJ');
  await page.keyboard.up('Shift');
  await page.keyboard.up('Control');
  check('Combination remains after keys are released', await input.inputValue() === 'Ctrl+Shift+J');
  check('Editing does not change active shortcut', await value() === 'Alt+C');
  await input.press('Escape');
  check('Cancel does not save', await page.evaluate(() => window.shortcutQA.calls === 0));
  await control.click();
  check('Reopen starts a fresh recording', await input.inputValue() === '' && await value() === 'Alt+C');
  await input.press('KeyA');
  await input.press('KeyB');
  check('Successive keys replace rather than append text', await input.inputValue() === 'B');
  await input.press('F6');
  check('Non-letter key is recorded', await input.inputValue() === 'F6');
  await input.press('Alt+Shift+K');
  check('Records actual key combination', await input.inputValue() === 'Alt+Shift+K');
  await page.screenshot({ path: 'output/playwright/shortcut-recorder.png', type: 'png' });
  await input.press('Tab');
  check('Tab leaves recording for Apply', await apply.evaluate(element => document.activeElement === element));
  await apply.click();
  await page.waitForFunction(() => window.shortcutQA.pending !== null);
  check('Sends draft to native command', await page.evaluate(() => window.shortcutQA.requested === 'Alt+Shift+K'));
  check('No premature success while saving', await value() === 'Alt+C' && await input.isDisabled() && await apply.isDisabled());
  await page.evaluate(() => { window.shortcutQA.pending(); window.shortcutQA.pending = null; });
  await page.locator('.shortcut-editor').waitFor({ state: 'detached' });
  check('Updates display only after success', await value() === 'Alt+Shift+K');
  await control.click();
  await input.press('Alt+V');
  await page.evaluate(() => { window.shortcutQA.failure = 'Could not register Alt+V: hotkey already registered'; });
  await apply.click();
  await page.locator('.shortcut-editor [role=alert]').waitFor();
  check('Shows native string error', await page.locator('.shortcut-editor [role=alert]').textContent() === 'Could not register Alt+V: hotkey already registered');
  check('Failure preserves draft and active shortcut', await input.inputValue() === 'Alt+V' && await value() === 'Alt+Shift+K');
  await page.screenshot({ path: 'output/playwright/shortcut-error.png', type: 'png' });
  await page.evaluate(() => { window.shortcutQA.failure = ''; });
  await apply.click();
  await page.waitForFunction(() => window.shortcutQA.pending !== null);
  await page.evaluate(() => { window.shortcutQA.pending(); window.shortcutQA.pending = null; });
  await page.locator('.shortcut-editor').waitFor({ state: 'detached' });
  check('Retry can save', await value() === 'Alt+V');
  await page.reload();
  await page.locator('.ref-primary-nav').getByRole('button', { name: 'Settings', exact: true }).click();
  await page.waitForFunction(() => [...document.querySelectorAll('.shortcut-control kbd')].map(key => key.textContent).join('+') === 'Alt+V');
  check('Reload reads saved mock setting', await value() === 'Alt+V');
  return { results, scope: 'Production UI/controller with simulated IPC; no native hotkey or persistence acceptance.' };
}
