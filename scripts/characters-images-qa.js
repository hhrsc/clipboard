async (page) => {
  const results = [];
  const errors = [];
  const check = (name, pass) => {
    if (!pass) throw new Error(name);
    results.push(name);
  };
  page.on('pageerror', error => errors.push(error.message));
  const before = await page.evaluate(() => JSON.parse(sessionStorage.getItem('characters-before-layout') || 'null'));
  for (const tab of ['recent', 'images', 'passwords', 'settings']) {
    await page.goto(`http://127.0.0.1:1420/?reference=${tab}`);
    await page.locator('.reference-mode').waitFor();
    await page.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all([...document.images].map(image => image.decode().catch(() => {})));
    });
    await page.screenshot({ path: `output/playwright/characters-after-${tab}.png`, type: 'png' });
    if (before) {
      const rects = await page.locator('.ref-sidebar, .ref-main, .ref-topbar, .clip-column, .clip-detail, .image-library, .image-detail, .password-column, .password-detail, .settings-content').evaluateAll(nodes => nodes.map(n => n.getBoundingClientRect().toJSON()));
      check(`${tab}: unchanged panel geometry`, JSON.stringify(rects) === JSON.stringify(before[tab].map(n => n.rect)));
    }
    check(`${tab}: Characters navigation label`, await page.locator('.ref-primary-nav').getByRole('button', { name: 'Characters', exact: true }).count() === 1);
  }
  await page.goto('http://127.0.0.1:1420/?reference=characters');
  await page.getByRole('heading', { name: 'Characters', exact: true }).waitFor();
  check('Characters reference alias excludes image row', await page.locator('.clip-row').count() === 9 && await page.locator('.clip-row img').count() === 0);

  // 所有数据和 IPC 均为独立浏览器中的合成数据，不连接桌面或系统剪贴板。
  await page.addInitScript(() => {
    localStorage.clear();
    window.isTauri = true;
    window.qaCommands = [];
    window.qaSnapshot = null;
    const now = Date.now();
    const record = (offset, type, content, categoryId = 'all', isPinned = false) => ({
      id: now - offset, type, content, timestamp: new Date(now - offset).toISOString(), categoryId, isPinned
    });
    window.qaStore = JSON.parse(sessionStorage.getItem('characters-qa-store') || 'null') || {
      version: 1,
      categories: [{ id: 'all', name: '全部' }, { id: 'work', name: 'Work' }, { id: 'photos', name: 'Photos' }],
      records: [record(1000, 'image', 'file|qa-first.png'), record(2000, 'text', 'QA plain text'), record(3000, 'image', 'file|qa-second.png', 'photos', true), record(4000, 'text', 'QA pinned text', 'all', true), record(5000, 'text', 'QA work text', 'work')],
      preferences: { captureEnabled: true, retentionHours: 24, officialWebsite: 'https://example.com' }
    };
    window.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' } },
      convertFileSrc: path => `/reference-assets/images/today-${path.includes('second') ? 2 : path.includes('incoming') ? 3 : 1}.png`,
      async invoke(command, args) {
        window.qaCommands.push({ command, args });
        switch (command) {
          case 'app_reset_status': return false;
          case 'vault_status': return { exists: false, unlocked: false };
          case 'clipboard_store_status': return { exists: true };
          case 'clipboard_store_load': return structuredClone(window.qaStore);
          case 'clipboard_store_replace':
            window.qaStore = structuredClone(args.store);
            sessionStorage.setItem('characters-qa-store', JSON.stringify(args.store));
            return;
          case 'autostart_status': return false;
          case 'get_clipboard_snapshot': return window.qaSnapshot;
          case 'shortcut_status': return { shortcut: 'Alt+C' };
          case 'persist_history_image': return 'qa-incoming.png';
          case 'set_clipboard_text': return;
          case 'set_clipboard_image_from_path': return;
          default: throw new Error(`Unexpected mock command: ${command}`);
        }
      }
    };
  });
  await page.goto('http://127.0.0.1:1420/');
  const nav = async name => {
    if (page.viewportSize().width <= 1100) await page.locator('.navigation-toggle').click();
    await page.locator('.ref-primary-nav').getByRole('button', { name, exact: true }).click();
  };
  const collection = async name => {
    if (page.viewportSize().width <= 1100) await page.locator('.navigation-toggle').click();
    await page.locator('.ref-collection-list button').filter({ has: page.locator('span').getByText(name, { exact: true }) }).click();
  };
  const countIs = (selector, count) => page.waitForFunction(({ selector, count }) => document.querySelectorAll(selector).length === count, { selector, count });
  await countIs('.clip-row', 3);
  check('Actual entry: text-only rows', !await page.locator('.reference-mode').count() && await page.locator('.clip-row img').count() === 0);
  check('Newest image never becomes text detail', await page.locator('.clip-content').innerText() === 'QA plain text');
  check('Text footer excludes images', await page.locator('.ref-sidebar-footer').innerText() === '3 clips');
  check('Text collection count excludes image-only group', await page.locator('.ref-collection-list').innerText().then(text => /Photos\s+0/.test(text)));
  await page.locator('.clip-row').filter({ hasText: 'QA pinned text' }).click();
  await page.getByRole('button', { name: 'Filter clips', exact: true }).click();
  await page.locator('.clip-filter').getByRole('button', { name: 'Pinned clips', exact: true }).click();
  await countIs('.clip-row', 1);
  check('Pinned filter excludes pinned images', await page.locator('.clip-row').innerText().then(text => text.includes('QA pinned text')));
  await page.locator('.ref-sidebar-footer').click();
  await page.getByPlaceholder('Search your clipboard').fill('qa-first.png');
  await countIs('.clip-row', 0);
  check('Image search cannot leak into Characters detail', await page.locator('.clip-content').count() === 0);
  await page.getByPlaceholder('Search your clipboard').fill('');
  await nav('Images');
  await countIs('.image-grid button', 2);
  check('Images footer excludes text', await page.locator('.ref-sidebar-footer').innerText() === '2 images');
  check('Images decode', await page.locator('.image-preview').evaluate(image => image.complete && image.naturalWidth > 0));
  await page.locator('.image-grid button').first().click({ button: 'right' });
  await page.screenshot({ path: 'output/playwright/characters-image-collection.png', type: 'png' });
  await page.locator('.image-collection-menu').getByRole('button', { name: 'Work', exact: true }).click();
  await page.waitForFunction(() => {
    const categoryId = window.qaStore.categories.find(c => c.name === 'Work').id;
    return window.qaStore.records.find(r => r.content === 'file|qa-first.png').categoryId === categoryId;
  });
  check('Image category saved; original records retained', await page.evaluate(() => window.qaStore.records.length === 5 && !window.qaCommands.some(c => c.command === 'delete_history_images')));
  await collection('Work');
  await countIs('.image-grid button', 1);
  await page.reload();
  await nav('Images');
  await collection('Work');
  await countIs('.image-grid button', 1);
  check('Image collection survives controller reload', true);
  await page.locator('.image-grid button').click({ button: 'right' });
  await page.locator('.image-collection-menu').getByRole('button', { name: 'All clips', exact: true }).click();
  await countIs('.image-grid button', 0);
  check('Moving last image out of filtered group clears preview', await page.locator('.image-preview').count() === 0 && await page.locator('.image-empty-detail').isVisible());
  await page.locator('.ref-sidebar-footer').click();
  await countIs('.image-grid button', 2);
  check('Images footer resets filters without returning to Characters', await page.locator('.images-page').count() === 1);
  await collection('Photos');
  await nav('Characters');
  await countIs('.clip-row', 0);
  check('Image-only collection shows no stale text preview', await page.locator('.clip-content').count() === 0);
  await page.locator('.ref-sidebar-footer').click();
  await page.locator('.clip-row').filter({ hasText: 'QA pinned text' }).click();
  await page.evaluate(() => { window.qaSnapshot = { type: 'image', content: 'image|cWEtbmV3LWltYWdl' }; });
  await page.waitForFunction(() => window.qaStore.records.some(r => r.content === 'file|qa-incoming.png'));
  check('Live image capture keeps text selection and row count', await page.locator('.clip-row').count() === 3 && await page.locator('.clip-content').innerText() === 'QA pinned text');
  await nav('Images');
  await countIs('.image-grid button', 3);
  check('Live image appears only in Images', true);
  await page.evaluate(() => { window.qaSnapshot = { type: 'text', content: 'QA newly captured text' }; });
  await page.waitForFunction(() => window.qaStore.records.some(r => r.content === 'QA newly captured text'));
  check('New text does not appear in Images', await page.locator('.image-grid button').count() === 3);
  await nav('Characters');
  await countIs('.clip-row', 4);
  for (const [width, height] of [[1586, 992], [360, 400]]) {
    await page.setViewportSize({ width, height });
    await page.screenshot({ path: `output/playwright/characters-live-${width}x${height}.png`, type: 'png' });
    await nav('Images');
    await page.screenshot({ path: `output/playwright/characters-images-${width}x${height}.png`, type: 'png' });
    await page.locator('.image-grid button').first().click({ button: 'right' });
    check(`${width}x${height}: image collection menu is inside viewport`, await page.locator('.image-collection-menu').evaluate(node => {
      const r = node.getBoundingClientRect();
      return r.left >= 0 && r.right <= innerWidth && r.top >= 0 && r.bottom <= innerHeight;
    }));
    await page.keyboard.press('Escape');
    check(`${width}x${height}: Escape closes image menu`, await page.locator('.image-collection-menu').count() === 0);
    await nav('Characters');
  }
  await page.evaluate(() => {
    window.qaStore.records = window.qaStore.records.filter(r => r.type === 'image');
    sessionStorage.setItem('characters-qa-store', JSON.stringify(window.qaStore));
  });
  await page.reload();
  await page.getByText('No text clips yet. Copy text to get started.', { exact: true }).waitFor();
  check('Only-images history gives empty Characters', await page.locator('.clip-content').count() === 0);
  await nav('Images');
  await countIs('.image-grid button', 3);
  check('Empty Characters does not delete stored images', await page.evaluate(() => window.qaStore.records.length === 3));
  check('No frontend runtime errors', errors.length === 0);
  check('No native command errors', await page.locator('.error-toast').count() === 0);
  return { checks: results.length, results, scope: 'Isolated browser with synthetic IPC; actual desktop acceptance remains with user.' };
}
