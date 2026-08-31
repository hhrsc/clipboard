async (page) => {
  const invoke = (command, args) => page.evaluate(({ command, args }) => window.__TAURI_INTERNALS__.invoke(command, args), { command, args });
  if (await invoke('plugin:app|identifier') !== 'com.lenovo.my-clipboard.qa-phase2') throw new Error('QA profile required');
  const store = await invoke('clipboard_store_load');
  const vault = await invoke('vault_status');
  const results = [
    { name: 'text and pin survive process restart', pass: store.records.some(r => r.content === 'PHASE2-QA live text one' && r.isPinned) },
    { name: 'image record survives process restart', pass: store.records.some(r => r.type === 'image') },
    { name: 'retention survives process restart', pass: store.preferences.retentionHours === 25 },
    { name: 'capture pause survives process restart', pass: store.preferences.captureEnabled === false },
    { name: 'shortcut survives process restart', pass: (await invoke('shortcut_status')).shortcut === 'Alt+V' },
    { name: 'vault is locked after process restart', pass: vault.exists && !vault.unlocked }
  ];
  await page.locator('.ref-primary-nav').getByRole('button', { name: 'Images', exact: true }).click();
  await page.waitForFunction(() => [...document.querySelectorAll('.image-grid img')].some(i => i.naturalWidth > 0));
  results.push({ name: 'cached image decodes after restart', pass: true });
  await page.evaluate(results => localStorage.setItem('phase2-restart-results', JSON.stringify(results)), results);
  return results;
}
