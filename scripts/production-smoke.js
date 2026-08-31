async (page) => {
  const results = [];
  const invoke = (command, args) => page.evaluate(({ command, args }) => window.__TAURI_INTERNALS__.invoke(command, args), { command, args });
  const check = (name, pass) => { results.push({ name, pass }); if (!pass) throw new Error(name); };
  if (await invoke('plugin:app|identifier') !== 'com.lenovo.my-clipboard') throw new Error('Production profile required');
  page.on('pageerror', error => results.push({ name: 'runtime error', pass: false, error: error.message }));
  try {
    check('production native startup and viewport', await page.evaluate(() => location.hostname === 'tauri.localhost' && innerWidth === 1586 && innerHeight === 992 && !document.querySelector('.reference-mode')));
    const initial = await invoke('clipboard_store_load');
    check('real history loaded without storage error', initial.records.length > 0 && await page.locator('.error-toast').count() === 0);
    for (const name of ['Images', 'Passwords', 'Settings', 'Recent']) {
      await page.locator('.ref-primary-nav').getByRole('button', { name, exact: true }).click();
      check(`production page ${name}`, await page.locator('.ref-primary-nav button.active').innerText() === name);
    }
    await page.getByRole('button', { name: 'New collection', exact: true }).click();
    await page.getByLabel('Collection name').fill('PHASE2-QA temporary collection');
    await page.getByRole('button', { name: 'Create collection', exact: true }).click();
    await page.waitForFunction(async () => (await window.__TAURI_INTERNALS__.invoke('clipboard_store_load')).categories.some(c => c.name === 'PHASE2-QA temporary collection'));
    check('collection creation persisted', true);
    await page.locator('.ref-collection-list').getByRole('button', { name: 'PHASE2-QA temporary collection 0', exact: true }).click({ button: 'right' });
    await page.getByRole('button', { name: 'Rename collection', exact: true }).click();
    await page.getByLabel('Collection name').fill('PHASE2-QA renamed collection');
    await page.getByRole('button', { name: 'Save collection', exact: true }).click();
    await page.waitForFunction(async () => (await window.__TAURI_INTERNALS__.invoke('clipboard_store_load')).categories.some(c => c.name === 'PHASE2-QA renamed collection'));
    check('collection rename persisted', true);
    await page.locator('.ref-collection-list').getByRole('button', { name: 'PHASE2-QA renamed collection 0', exact: true }).click({ button: 'right' });
    await page.getByRole('button', { name: 'Delete collection', exact: true }).click();
    await page.waitForFunction(async () => !(await window.__TAURI_INTERNALS__.invoke('clipboard_store_load')).categories.some(c => c.name === 'PHASE2-QA renamed collection'));
    const final = await invoke('clipboard_store_load');
    check('test collection removed; user records preserved', initial.records.every(r => final.records.some(f => f.id === r.id && f.content === r.content)));
    await page.locator('.ref-primary-nav').getByRole('button', { name: 'Settings', exact: true }).click();
    await page.screenshot({ path: 'design-qa/final/native-production-settings.png', type: 'png', scale: 'css' });
  } catch (error) { results.push({ name: 'stopped', pass: false, error: error.message }); }
  return results;
}
