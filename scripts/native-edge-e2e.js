async (page) => {
  const results = [];
  const invoke = (command, args) => page.evaluate(({ command, args }) => window.__TAURI_INTERNALS__.invoke(command, args), { command, args });
  if (await invoke('plugin:app|identifier') !== 'com.lenovo.my-clipboard.qa-phase2') throw new Error('QA profile required');
  const check = (name, pass) => { results.push({ name, pass }); if (!pass) throw new Error(name); };
  try {
    await page.locator('.ref-primary-nav').getByRole('button', { name: 'Passwords', exact: true }).click();
    if (!(await invoke('vault_status')).unlocked) {
      await page.getByLabel('Master password', { exact: true }).fill('Phase2-QA-only-master-8431');
      await page.getByRole('button', { name: 'Unlock vault', exact: true }).click();
    }
    await page.getByRole('button', { name: 'Add password', exact: true }).waitFor();
    await page.locator('input[type=file]').setInputFiles('scripts/fixtures/passwords-qa.json');
    await page.locator('.password-row').filter({ hasText: 'PHASE2-QA imported' }).waitFor();
    check('JSON import through existing file input handler', (await invoke('vault_unlock', { masterPassword: 'Phase2-QA-only-master-8431' })).some(p => p.title === 'PHASE2-QA imported'));
    await page.locator('.password-row').filter({ hasText: 'PHASE2-QA imported' }).click();
    await page.locator('.password-detail').getByRole('button', { name: 'More', exact: true }).click();
    await page.getByRole('button', { name: 'Delete password', exact: true }).click();
    await page.waitForFunction(() => ![...document.querySelectorAll('.password-row')].some(e => e.textContent.includes('PHASE2-QA imported')));
    check('password deletion persisted', !(await invoke('vault_unlock', { masterPassword: 'Phase2-QA-only-master-8431' })).some(p => p.title === 'PHASE2-QA imported'));
    await invoke('vault_lock');
    const store = await invoke('clipboard_store_load');
    const sample = store.records.find(r => r.type === 'image');
    if (!sample?.content.includes('com.lenovo.my-clipboard.qa-phase2')) throw new Error('Expected isolated image cache');
    const missing = { ...sample, id: Date.now(), content: sample.content.replace(/[^\\/]+$/, 'phase2-intentionally-missing.png') };
    store.records.unshift(missing);
    await invoke('clipboard_store_replace', { store });
    await page.reload();
    await page.locator('.ref-primary-nav').getByRole('button', { name: 'Images', exact: true }).click();
    await page.getByText('Image unavailable', { exact: true }).first().waitFor();
    check('missing file has explicit fallback', true);
    check('missing file copy disabled', await page.getByRole('button', { name: 'Copy image', exact: true }).isDisabled());
    await page.getByRole('button', { name: 'Delete image', exact: true }).click();
    await page.waitForFunction(async id => !(await window.__TAURI_INTERNALS__.invoke('clipboard_store_load')).records.some(r => r.id === id), missing.id);
    check('image deletion persisted', true);
    await page.evaluate(async () => {
      const original = JSON.parse(localStorage.getItem('phase2-original-clipboard') || 'null');
      if (original?.[0] === 'text') await window.__TAURI_INTERNALS__.invoke('set_clipboard_text', { text: original[1] });
      else if (original?.[0] === 'image') await window.__TAURI_INTERNALS__.invoke('set_clipboard_image', { base64: original[1].slice(6) });
      else await window.__TAURI_INTERNALS__.invoke('set_clipboard_text', { text: '' });
      localStorage.removeItem('phase2-original-clipboard');
    });
    check('original Windows clipboard restored', true);
  } catch (error) { results.push({ name: 'stopped', pass: false, error: error.message }); }
  await page.evaluate(results => localStorage.setItem('phase2-edge-results', JSON.stringify(results)), results);
  return results;
}
