async (page) => {
  const results = [];
  const check = (name, ok) => { results.push({ name, pass: !!ok }); if (!ok) throw new Error(name); };
  const invoke = (command, args) => page.evaluate(({ command, args }) => window.__TAURI_INTERNALS__.invoke(command, args), { command, args });
  try {
    check('isolated QA identifier', await invoke('plugin:app|identifier') === 'com.lenovo.my-clipboard.qa-phase2');
    await page.locator('.ref-primary-nav').getByRole('button', { name: 'Passwords', exact: true }).click();
    const status = await invoke('vault_status');
    let masterPassword = 'Phase2-QA-8431';
    if (status.exists) {
      try { await invoke('vault_unlock', { masterPassword }); }
      catch { masterPassword = 'Phase2-QA-only-master-8431'; }
    }
    if (!status.exists) {
      await page.getByLabel('Master password', { exact: true }).fill(masterPassword);
      await page.getByLabel('Confirm master password', { exact: true }).fill(masterPassword);
      await page.getByRole('button', { name: 'Create vault', exact: true }).click();
    } else if (!status.unlocked) {
      await page.getByLabel('Master password', { exact: true }).fill(masterPassword);
      await page.getByRole('button', { name: 'Unlock vault', exact: true }).click();
    }
    await page.getByRole('button', { name: 'Add password', exact: true }).waitFor();
    check('existing encrypted vault setup/unlock UI', true);
    await page.getByRole('button', { name: 'Add password', exact: true }).click();
    await page.getByLabel('Title', { exact: true }).fill('PHASE2-QA credential');
    await page.getByLabel('Username', { exact: true }).fill('qa@example.invalid');
    await page.getByLabel('Password', { exact: true }).fill('qa-fixture-value-123');
    await page.getByRole('button', { name: 'Save changes', exact: true }).click();
    await page.locator('.password-row').filter({ hasText: 'PHASE2-QA credential' }).waitFor();
    check('password creation persisted', (await invoke('vault_unlock', { masterPassword })).passwords.some(p => p.title === 'PHASE2-QA credential'));
    await page.getByLabel('Title', { exact: true }).fill('PHASE2-QA edited');
    await page.getByRole('button', { name: 'Save changes', exact: true }).click();
    await page.locator('.password-row').filter({ hasText: 'PHASE2-QA edited' }).waitFor();
    check('password edit persisted', (await invoke('vault_unlock', { masterPassword })).passwords.some(p => p.title === 'PHASE2-QA edited'));
    await page.getByRole('button', { name: 'Copy username', exact: true }).click();
    check('copy username', (await invoke('get_clipboard_data'))[1] === 'qa@example.invalid');
    await page.getByRole('button', { name: 'Copy password', exact: true }).click();
    check('copy password', (await invoke('get_clipboard_data'))[1] === 'qa-fixture-value-123');
    await page.locator('.ref-search input').fill('no-such-entry');
    check('password search', await page.locator('.password-row').count() === 0);
    await page.locator('.ref-search input').fill('');
    await page.locator('.password-footer').getByRole('button', { name: 'Lock vault', exact: true }).click();
    await page.getByRole('button', { name: 'Unlock vault', exact: true }).waitFor();
    check('vault lock clears native key', !(await invoke('vault_status')).unlocked);
    await page.getByLabel('Master password', { exact: true }).fill(masterPassword);
    await page.getByRole('button', { name: 'Unlock vault', exact: true }).click();
    await page.locator('.password-row').filter({ hasText: 'PHASE2-QA edited' }).waitFor();
    check('vault unlock reloads persisted data', true);
    await page.screenshot({ path: 'design-qa/final/native-passwords.png', type: 'png', scale: 'css' });
  } catch (error) { results.push({ name: 'stopped', pass: false, error: error.message }); }
  await page.evaluate(results => localStorage.setItem('phase2-vault-results', JSON.stringify(results)), results);
  return results;
}
