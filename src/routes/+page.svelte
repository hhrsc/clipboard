<script>
  import { convertFileSrc, invoke, isTauri } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import ClipboardApp from '$lib/ClipboardApp.svelte';
  import { sanitizeClipboardHtml, textSignature, toClipboardHtml } from '$lib/clipboard-format';

  const DEFAULT_CATEGORY = '全部';
  const HISTORY_KEY = 'clip_v5_split';
  const LAST_DATA_KEY = 'clip_v5_last_data';
  const PASSWORDS_KEY = 'clip_v5_passwords';
  const CATEGORIES_KEY = 'clip_v5_categories';
  const PREFERENCES_KEY = 'clip_v6_preferences';
  const RECOVERY_FLAG_KEY = 'clip_v5_recovery_imported';
  const PASSWORD_EXPORT_FORMAT = 'my-clipboard-passwords';
  const PASSWORD_EXPORT_VERSION = 2;
  const MAX_PASSWORD_IMPORT_BYTES = 1024 * 1024;
  const MAX_IMPORTED_PASSWORDS = 1000;

  /**
   * @typedef {Object} HistoryItem
   * @property {'text' | 'image' | string} type
   * @property {string} content
   * @property {number} id
   * @property {string} timestamp
   * @property {string=} category
   * @property {boolean=} restored
   * @property {boolean=} isPinned
   * @property {string=} html
   */

  /**
   * @typedef {Object} PasswordItem
   * @property {number} id
   * @property {string} title
   * @property {string} username
   * @property {string} password
   * @property {boolean} showPass
   * @property {string | null=} collectionId
   */

  let activeTab = 'recent';
  let showToast = false;
  let toastMsg = 'Copied!';

  let searchQuery = '';
  /** @type {HistoryItem[]} */
  let history = [];
  let lastData = '';
  let captureEnabled = true;
  let retentionHours = 24;
  let officialWebsite = 'https://www.gov.cn/';
  let storeReady = false;
  let storeError = '';
  let captureError = '';
  let settingsBusy = false;
  let autostartEnabled = false;
  let autostartError = '';
  let storeWrites = Promise.resolve(true);
  let vaultWrites = Promise.resolve(true);
  let vaultSaving = false;
  let copyInFlight = 0;
  let resetPending = false;
  let resetEpoch = 0;
  let startupBlocked = false;
  let shortcutValue = 'Alt+C';
  let shortcutBusy = false;
  let shortcutError = '';
  let showClearDataConfirm = false;
  let clearDataConfirmation = '';
  let clearDataBusy = false;
  let clearDataError = '';

  let pwdSearchQuery = '';
  /** @type {PasswordItem[]} */
  let passwords = [];
  /** @type {Array<{id: string, name: string}>} */
  let vaultCollections = [];
  let activePasswordCollection = '';
  let vaultExists = false;
  let vaultUnlocked = false;
  let vaultRequirePassword = true;
  let vaultAutoUnlockAvailable = false;
  let vaultStatusReady = false;
  let vaultAutoUnlockFailed = false;
  let vaultAccessError = '';
  let vaultBusy = false;
  let vaultError = '';
  let masterPassword = '';
  let masterPasswordConfirm = '';
  let unlockPassword = '';
  const VAULT_AUTO_LOCK_MS = 5 * 60 * 1000;
  let vaultLastActivity = Date.now();
  let newPwdTitle = '';
  let newPwdUser = '';
  let newPwdPass = '';
  /** @type {number | null} */
  let editingPasswordId = null;
  let editPwdTitle = '';
  let editPwdUser = '';
  let editPwdPass = '';
  /** @type {HTMLInputElement | undefined} */
  let passwordImportInput;

  let showNewPwd = false;
  let isAppCopying = false;

  let categories = [DEFAULT_CATEGORY];
  let activeCategory = DEFAULT_CATEGORY;
  let recentFilter = 'all';
  /** @type {number | null} */
  let selectedClipId = null;
  let isAddingCat = false;
  let newCatName = '';

  let showContextMenu = false;
  let menuX = 0;
  let menuY = 0;
  let targetCategory = '';

  let showImageMenu = false;
  let imageMenuX = 0;
  let imageMenuY = 0;
  /** @type {HistoryItem | null} */
  let targetImage = null;
  /** @type {HistoryItem | null} */
  let previewImage = null;
  let previewScale = 1;

  let isRenamingCat = false;
  let renameCatName = '';

  /** @param {WheelEvent & { currentTarget: HTMLElement }} event */
  function handleCategoryWheel(event) {
    const delta = Math.abs(event.deltaY) > Math.abs(event.deltaX) ? event.deltaY : event.deltaX;
    event.currentTarget.scrollLeft += delta;
  }

  /** @param {number} nextScale */
  function clampPreviewScale(nextScale) {
    return Math.min(4, Math.max(0.2, Number(nextScale.toFixed(2))));
  }

  /** @param {string} content */
  function imageClipboardSignature(content) {
    if (!content) return '';
    const payload = content.includes('|') ? content.split('|')[1] : content;
    if (!payload) return '';
    const head = payload.slice(0, 32);
    const tail = payload.slice(-32);
    return `image:${payload.length}:${head}:${tail}`;
  }

  /**
   * @param {string} type
   * @param {string} content
   * @param {string=} html
   */
  function clipboardSignature(type, content, html) {
    if (type === 'image') return imageClipboardSignature(content);
    return textSignature(content || '', html);
  }

  /** @param {unknown} content */
  function isFileImageContent(content) {
    return typeof content === 'string' && content.startsWith('file|');
  }

  /** @param {string} rawContent */
  async function persistImageContent(rawContent) {
    if (!rawContent) return '';
    const base64 = rawContent.includes('|') ? rawContent.split('|')[1] : rawContent;
    if (!base64) return '';
    const path = /** @type {string} */ (await invoke('persist_history_image', { base64 }));
    return `file|${path}`;
  }

  /** @param {HistoryItem[]} items */
  function collectImagePaths(items) {
    return items
      .filter((item) => item.type !== 'text' && isFileImageContent(item.content))
      .map((item) => item.content.slice(5));
  }

  /** @param {HistoryItem[]} items */
  async function cleanupImageFilesFromItems(items) {
    const paths = collectImagePaths(items);
    if (paths.length === 0) return;
    try {
      await invoke('delete_history_images', { paths });
    } catch {
      // ignore file cleanup failures
    }
  }

  async function migrateLegacyImageItems() {
    let changed = false;
    /** @type {HistoryItem[]} */
    const migrated = [];
    for (const item of history) {
      if (item.type !== 'text' && item.content && !isFileImageContent(item.content)) {
        try {
          const fileContent = await persistImageContent(item.content);
          migrated.push({ ...item, content: fileContent });
          changed = true;
          continue;
        } catch {
          // keep original item if migration fails
        }
      }
      migrated.push(item);
    }
    if (changed) {
      updateAndSaveHistory(migrated);
    }
  }

  /** @param {HistoryItem[]} newArray */
  function updateAndSaveHistory(newArray) {
    if (clearDataBusy || resetPending) return;
    const previousHistory = history;
    const EXPIRATION_MS = retentionHours * 60 * 60 * 1000;
    const now = Date.now();

    const valid = newArray.filter((item) => {
      if (
        item.type === 'text' &&
        !item.restored &&
        !item.isPinned &&
        (!item.category || item.category === DEFAULT_CATEGORY)
      ) {
        return now - item.id < EXPIRATION_MS;
      }
      return true;
    });

    const imgs = valid.filter((item) => item.type !== 'text');
    const categorizedTexts = valid.filter(
      (item) => item.type === 'text' && item.category && item.category !== DEFAULT_CATEGORY
    );
    const pinnedTexts = valid.filter((item) => item.type === 'text' && item.isPinned && (!item.category || item.category === DEFAULT_CATEGORY));
    const uncategorizedTexts = valid.filter(
      (item) => item.type === 'text' && !item.isPinned && (!item.category || item.category === DEFAULT_CATEGORY)
    );

    const merged = [...imgs.slice(0, 10), ...categorizedTexts, ...pinnedTexts, ...uncategorizedTexts.slice(0, 50)];
    merged.sort((a, b) => b.id - a.id);

    const mergedIds = new Set(merged.map((item) => item.id));
    const removedImages = previousHistory.filter(
      (item) => item.type !== 'text' && !mergedIds.has(item.id)
    );

    history = merged;
    localStorage.setItem(LAST_DATA_KEY, lastData);
    void persistClipboardStore().then((saved) => {
      if (saved && removedImages.length) void cleanupImageFilesFromItems(removedImages);
    });
  }

  function persistClipboardStore() {
    if (!storeReady || clearDataBusy || resetPending) return Promise.resolve(false);
      /** @type {Map<string, string>} */
      const catMap = new Map();
      catMap.set(DEFAULT_CATEGORY, "all");
      const nativeCats = [{ id: "all", name: DEFAULT_CATEGORY }];
      let idx = 1;
      for (const catName of categories) {
        if (catName === DEFAULT_CATEGORY || !catName.trim()) continue;
        const id = `category-${idx++}`;
        catMap.set(catName, id);
        nativeCats.push({ id, name: catName });
      }

      const records = history.map((item) => ({
        type: item.type,
        content: item.content,
        html: item.html,
        id: item.id,
        timestamp: item.timestamp,
        categoryId: catMap.get(item.category || DEFAULT_CATEGORY) || "all",
        restored: !!item.restored,
        isPinned: !!item.isPinned
      }));

      const store = {
        version: 1,
        categories: nativeCats,
        records,
        preferences: {
          captureEnabled,
          retentionHours,
          officialWebsite
        }
      };

      // 串行写入完整快照，避免较旧的请求覆盖新记录。
      storeWrites = storeWrites.then(async () => {
        try {
          await invoke('clipboard_store_replace', { store });
          storeError = '';
          return true;
        } catch (error) {
          storeError = `Changes are not saved: ${String(error)}`;
          return false;
        }
      });
      return storeWrites;
  }

  function savePasswords() {
    if (!vaultUnlocked) return Promise.resolve(false);
    return persistVaultPasswords();
  }

  async function persistVaultPasswords() {
    return saveVaultSnapshot(passwords, vaultCollections);
  }

  /** @param {PasswordItem[]} nextPasswords @param {Array<{id: string, name: string}>} nextCollections */
  function saveVaultSnapshot(nextPasswords, nextCollections) {
    if (!vaultUnlocked || clearDataBusy || resetPending) return Promise.resolve(false);
    const snapshot = {
      passwords: nextPasswords.map(({ id, title, username, password, collectionId }) => ({ id, title, username, password, collectionId: collectionId || null })),
      collections: nextCollections.map(item => ({ ...item }))
    };
    vaultSaving = true;
    vaultWrites = vaultWrites.then(async () => {
      try {
        await invoke('vault_replace_passwords', snapshot);
        passwords = snapshot.passwords.map(item => ({ ...item, showPass: false }));
        vaultCollections = snapshot.collections;
        vaultError = '';
        return true;
      } catch (error) {
        vaultError = `Could not save password vault: ${String(error)}`;
        return false;
      }
    });
    const operation = vaultWrites;
    void operation.finally(() => { if (vaultWrites === operation) vaultSaving = false; });
    return operation;
  }

  /** @param {{passwords: Array<{id: number, title: string, username: string, password: string, collectionId?: string | null}>, collections: Array<{id: string, name: string}>}} snapshot */
  function loadVaultPasswords(snapshot) {
    passwords = snapshot.passwords.map(item => ({ ...item, showPass: false }));
    vaultCollections = snapshot.collections || [];
  }

  function clearVaultMemory() {
    passwords = [];
    vaultCollections = [];
    activePasswordCollection = '';
    pwdSearchQuery = '';
    masterPassword = masterPasswordConfirm = unlockPassword = '';
    newPwdTitle = newPwdUser = newPwdPass = '';
    cancelEditPassword();
  }

  async function loadVaultStatus() {
    try {
      const status = await invoke('vault_status');
      vaultExists = status.exists;
      vaultRequirePassword = status.requirePassword !== false;
      vaultAutoUnlockAvailable = status.autoUnlockAvailable === true;
      if (status.unlocked) await invoke('vault_lock');
      vaultUnlocked = false;
      vaultStatusReady = true;
    } catch {
      vaultError = 'Secure password vault is unavailable.';
    }
  }

  async function autoUnlockVault() {
    if (vaultBusy || clearDataBusy || resetPending) return;
    const epoch = resetEpoch;
    vaultBusy = true;
    vaultError = '';
    try {
      const snapshot = await invoke('vault_auto_unlock');
      if (epoch !== resetEpoch || clearDataBusy || resetPending) return;
      loadVaultPasswords(snapshot);
      vaultUnlocked = true;
      vaultLastActivity = Date.now();
    } catch (error) {
      if (epoch !== resetEpoch || clearDataBusy || resetPending) return;
      vaultAutoUnlockFailed = true;
      vaultError = `Automatic opening failed. Use your master password. ${String(error)}`;
    } finally { vaultBusy = false; }
  }

  $: if (activeTab === 'passwords' && vaultStatusReady && vaultExists && !vaultRequirePassword && !vaultUnlocked && !vaultBusy && !vaultAutoUnlockFailed && !clearDataBusy && !resetPending) {
    void autoUnlockVault();
  }

  /** @param {boolean} required @param {string} password */
  async function setVaultRequirePassword(required, password) {
    if (!vaultExists || vaultBusy || vaultSaving || clearDataBusy || resetPending) return false;
    const epoch = resetEpoch;
    vaultAccessError = '';
    vaultBusy = true;
    try {
      if (!await vaultWrites) throw new Error('Save password changes before changing vault access.');
      await invoke('vault_set_require_password', { requirePassword: required, masterPassword: password });
      if (epoch !== resetEpoch || clearDataBusy || resetPending) return false;
      vaultRequirePassword = required;
      vaultAutoUnlockFailed = false;
      vaultError = '';
      if (required) { clearVaultMemory(); vaultUnlocked = false; }
      triggerToast(required ? 'Password requirement enabled' : 'Password requirement disabled on this Windows account');
      return true;
    } catch (error) {
      vaultAccessError = error instanceof Error ? error.message : String(error);
      return false;
    } finally { vaultBusy = false; }
  }

  async function setupVault() {
    if (vaultBusy || clearDataBusy || resetPending) return;
    vaultError = '';
    const length = Array.from(masterPassword).length;
    if (length < 8 || length > 16) {
      vaultError = 'Master password must contain 8-16 characters.';
      return;
    }
    if (masterPassword !== masterPasswordConfirm) {
      vaultError = 'The master passwords do not match.';
      return;
    }

    vaultBusy = true;
    try {
      const records = await invoke('vault_setup', {
        masterPassword,
        legacyPasswords: passwords.map(({ id, title, username, password }) => ({ id, title, username, password }))
      });
      loadVaultPasswords(records);
      localStorage.removeItem(PASSWORDS_KEY);
      vaultExists = true;
      vaultRequirePassword = true;
      vaultAutoUnlockFailed = false;
      vaultUnlocked = true;
      vaultLastActivity = Date.now();
      masterPassword = '';
      masterPasswordConfirm = '';
      triggerToast('Secure vault created');
    } catch (error) {
      vaultError = error instanceof Error ? error.message : typeof error === 'string' ? error : 'Could not create secure vault.';
    } finally {
      vaultBusy = false;
    }
  }

  async function unlockVault() {
    if (vaultBusy || clearDataBusy || resetPending) return;
    vaultError = '';
    vaultBusy = true;
    try {
      const records = await invoke('vault_unlock', { masterPassword: unlockPassword });
      loadVaultPasswords(records);
      unlockPassword = '';
      vaultUnlocked = true;
      vaultLastActivity = Date.now();
      triggerToast('Password vault unlocked');
    } catch (error) {
      vaultError = error instanceof Error ? error.message : String(error);
    } finally {
      vaultBusy = false;
    }
  }

  async function lockVault() {
    if (!vaultUnlocked || !vaultRequirePassword || vaultBusy || clearDataBusy || resetPending) return;
    vaultBusy = true;
    try {
      await vaultWrites;
      await invoke('vault_lock');
      clearVaultMemory();
      vaultUnlocked = false;
      triggerToast('Password vault locked');
    } catch {
      triggerToast('Could not lock password vault');
    } finally { vaultBusy = false; }
  }

  function noteVaultActivity() {
    if (vaultUnlocked) vaultLastActivity = Date.now();
  }

  function saveCategories() {
    void persistClipboardStore();
  }

  function savePreferences() {
    void persistClipboardStore();
  }

  async function toggleCapture() {
    if (!storeReady || settingsBusy || clearDataBusy || resetPending) return;
    settingsBusy = true;
    captureEnabled = !captureEnabled;
    if (await persistClipboardStore()) triggerToast(captureEnabled ? 'Clipboard capture resumed' : 'Clipboard capture paused');
    else captureEnabled = !captureEnabled;
    settingsBusy = false;
  }

  async function saveRetention() {
    if (!storeReady || settingsBusy || clearDataBusy || resetPending) return;
    if (!Number.isInteger(retentionHours) || retentionHours < 1 || retentionHours > 8760) {
      storeError = 'Retention must be between 1 and 8760 hours.';
      return;
    }
    settingsBusy = true;
    if (await persistClipboardStore()) triggerToast('Retention saved');
    settingsBusy = false;
  }

  async function toggleAutostart() {
    if (settingsBusy || clearDataBusy || resetPending) return;
    autostartError = '';
    settingsBusy = true;
    try {
      autostartEnabled = await invoke('set_autostart', { enabled: !autostartEnabled });
    } catch (error) {
      autostartError = String(error);
    } finally { settingsBusy = false; }
  }

  /** @param {string} nextShortcut */
  async function updateShortcut(nextShortcut) {
    if (shortcutBusy || clearDataBusy || resetPending) return false;
    shortcutBusy = true;
    shortcutError = '';
    try {
      const status = await invoke('update_global_shortcut', { shortcut: nextShortcut });
      shortcutValue = status.shortcut;
      triggerToast(`Shortcut set to ${shortcutValue}`);
      return true;
    } catch (error) {
      shortcutError = error instanceof Error ? error.message : String(error);
      try {
        const status = await invoke('shortcut_status');
        shortcutValue = status.shortcut;
      } catch {
        // 查询失败时保留最后一次确认的快捷键。
      }
      return false;
    } finally {
      shortcutBusy = false;
    }
  }

  async function openOfficialWebsite() {
    try {
      await invoke('open_in_chrome', { url: officialWebsite });
    } catch (error) {
      triggerToast(error instanceof Error ? error.message : 'Could not open Google Chrome');
    }
  }

  async function clearAllAppData() {
    if (clearDataBusy) return;
    if (settingsBusy || shortcutBusy || vaultBusy || copyInFlight) { clearDataError = 'Wait for the current operation to finish, then retry Delete.'; return; }
    if (clearDataConfirmation !== 'DELETE') {
      clearDataError = 'Type DELETE to confirm.';
      return;
    }
    clearDataBusy = true;
    clearDataError = '';
    const previousCapture = captureEnabled;
    captureEnabled = false;
    resetEpoch++;
    try {
      await Promise.all([storeWrites, vaultWrites]);
      const defaults = await invoke('reset_app_data', { confirmation: 'DELETE' });
      resetPending = true;
      localStorage.removeItem(HISTORY_KEY);
      localStorage.removeItem(LAST_DATA_KEY);
      localStorage.removeItem(PASSWORDS_KEY);
      localStorage.removeItem(CATEGORIES_KEY);
      localStorage.removeItem(PREFERENCES_KEY);
      localStorage.setItem(RECOVERY_FLAG_KEY, '1');
      history = [];
      categories = [DEFAULT_CATEGORY];
      activeCategory = DEFAULT_CATEGORY;
      selectedClipId = null;
      const baseline = await invoke('get_clipboard_snapshot');
      const safe = sanitizeClipboardHtml(baseline?.html);
      lastData = baseline ? clipboardSignature(baseline.type, baseline.content, safe.html) : '';
      localStorage.setItem(LAST_DATA_KEY, lastData);
      clearVaultMemory();
      vaultExists = false;
      vaultUnlocked = false;
      vaultRequirePassword = true;
      vaultAutoUnlockFailed = false;
      vaultAccessError = '';
      retentionHours = defaults.preferences.retentionHours;
      officialWebsite = defaults.preferences.officialWebsite;
      shortcutValue = 'Alt+C';
      autostartEnabled = false;
      storeError = captureError = autostartError = shortcutError = vaultError = '';
      searchQuery = '';
      recentFilter = 'all';
      await invoke('complete_app_reset');
      resetPending = false;
      storeReady = true;
      captureEnabled = true;
      showClearDataConfirm = false;
      clearDataConfirmation = '';
      triggerToast('All local app data cleared');
      if (startupBlocked) window.location.reload();
    } catch (error) {
      try { resetPending = await invoke('app_reset_status'); } catch { resetPending = true; }
      captureEnabled = resetPending ? false : previousCapture;
      if (resetPending) { clearVaultMemory(); vaultUnlocked = false; storeReady = false; }
      clearDataError = `Reset incomplete: ${String(error)}. ${resetPending ? 'Other changes are blocked; retry Delete to finish.' : 'No app data was deleted.'}`;
    } finally {
      clearDataBusy = false;
    }
  }

  async function maybeImportRecoveryData() {
    if (localStorage.getItem(RECOVERY_FLAG_KEY)) {
      return false;
    }

    try {
      const response = await fetch('/recovery-import.json', { cache: 'no-store' });
      if (!response.ok) return false;

      const data = await response.json();
      if (!Array.isArray(data?.history) || !Array.isArray(data?.passwords) || !Array.isArray(data?.categories)) {
        return false;
      }

      const importedHistory = data.history.map(/** @param {HistoryItem} item */ (item) =>
        item?.type === 'text' ? { ...item, restored: true } : item
      );

      localStorage.setItem(HISTORY_KEY, JSON.stringify(importedHistory));
      localStorage.setItem(LAST_DATA_KEY, typeof data.lastData === 'string' ? data.lastData : '');
      localStorage.setItem(PASSWORDS_KEY, JSON.stringify(data.passwords));
      localStorage.setItem(CATEGORIES_KEY, JSON.stringify(data.categories));
      localStorage.setItem(RECOVERY_FLAG_KEY, '1');
      return true;
    } catch {
      // ignore recovery import failures
      return false;
    }
  }

  onMount(() => {
    /** @type {ReturnType<typeof setInterval> | undefined} */
    let clipboardInterval;
    /** @type {((e: ClipboardEvent) => void) | undefined} */
    let handlePaste;
    /** @type {ReturnType<typeof setInterval> | undefined} */
    let vaultLockInterval;
    let disposed = false;

    const init = async () => {
      if (!isTauri()) return;
      if (disposed) return;
      try {
        resetPending = await invoke('app_reset_status');
        if (resetPending) {
          startupBlocked = true;
          captureEnabled = false;
          activeTab = 'settings';
          clearDataError = 'An unfinished reset was found. Type DELETE and retry to finish; old data will not be imported.';
          return;
        }
      } catch (error) { storeError = `Reset status could not be checked: ${String(error)}`; return; }
      await loadVaultStatus();
      if (disposed) return;
      // 原生写入成功前保留旧存储，避免迁移失败丢失数据。
      try {
        const storeStatus = await invoke("clipboard_store_status");
        if (storeStatus && storeStatus.exists) {
          const loadedStore = await invoke("clipboard_store_load");
          const idToName = new Map();
          for (const cat of loadedStore.categories) {
            idToName.set(cat.id, cat.name);
          }
          categories = loadedStore.categories.map(/** @param {{id: string, name: string}} c */ (c) => c.name);
          if (!categories.includes(DEFAULT_CATEGORY)) {
            categories = [DEFAULT_CATEGORY, ...categories];
          }
          history = loadedStore.records.map(/** @param {{type: string, content: string, html?: string, id: number, timestamp: string, categoryId: string, restored?: boolean, isPinned?: boolean}} r */ (r) => ({
            type: r.type,
            content: r.content,
            html: sanitizeClipboardHtml(r.html).html,
            id: r.id,
            timestamp: r.timestamp,
            category: idToName.get(r.categoryId) || DEFAULT_CATEGORY,
            restored: !!r.restored,
            isPinned: !!r.isPinned
          }));
          if (loadedStore.preferences && typeof loadedStore.preferences.captureEnabled === "boolean") {
            captureEnabled = loadedStore.preferences.captureEnabled;
            retentionHours = loadedStore.preferences.retentionHours;
            officialWebsite = loadedStore.preferences.officialWebsite;
          }
          const cached = localStorage.getItem(HISTORY_KEY);
          if (cached) {
            const legacy = JSON.parse(cached);
            if (!Array.isArray(legacy)) throw new Error('Legacy history is invalid');
            const known = new Set(history.map((item) => item.id));
            const pending = legacy.filter((item) => !known.has(item.id));
            for (const item of pending) {
              if (item.type === 'image' && !isFileImageContent(item.content)) item.content = await persistImageContent(item.content);
            }
            history = [...history, ...pending].sort((a, b) => b.id - a.id);
            categories = [...new Set([...categories, ...pending.map((item) => item.category || DEFAULT_CATEGORY)])];
            storeReady = true;
            if (!await persistClipboardStore()) { storeReady = false; return; }
            localStorage.removeItem(HISTORY_KEY);
            localStorage.removeItem(CATEGORIES_KEY);
            localStorage.removeItem(PREFERENCES_KEY);
          }
        } else {
          /** @type {Array<{type: string, content: string, id: number, timestamp: string, category: string, restored: boolean, isPinned: boolean}>} */
          let legacyRecords = [];
          let legacyCategories = [DEFAULT_CATEGORY];
          let legacyPreferences = { captureEnabled: true, retentionHours: 24, officialWebsite: "https://www.gov.cn/" };

          const savedHistory = localStorage.getItem(HISTORY_KEY);
          if (savedHistory) {
            try {
              const parsed = JSON.parse(savedHistory);
              if (Array.isArray(parsed)) {
                legacyRecords = parsed.map((item) => ({
                  type: item.type || "text",
                  content: item.content || "",
                  id: item.id || Date.now(),
                  timestamp: item.timestamp || "",
                  category: item.category || DEFAULT_CATEGORY,
                  restored: !!item.restored,
                  isPinned: !!item.isPinned
                }));
              } else throw new Error('Legacy history is invalid');
            } catch { throw new Error('Legacy history could not be read'); }
          }

          const savedCats = localStorage.getItem(CATEGORIES_KEY);
          if (savedCats) {
            try {
              const parsed = JSON.parse(savedCats);
              if (!Array.isArray(parsed)) throw new Error('Legacy collections are invalid');
              legacyCategories = parsed;
            } catch { throw new Error('Legacy collections could not be read'); }
          }

          const savedPrefs = localStorage.getItem(PREFERENCES_KEY);
          if (savedPrefs) {
            try {
              const parsed = JSON.parse(savedPrefs);
              if (typeof parsed?.captureEnabled === "boolean") legacyPreferences.captureEnabled = parsed.captureEnabled;
            } catch { throw new Error('Legacy preferences could not be read'); }
          }

          for (const item of legacyRecords) {
            if (item.type === 'image' && !isFileImageContent(item.content)) item.content = await persistImageContent(item.content);
          }
          legacyCategories = [...new Set([...legacyCategories, ...legacyRecords.map((item) => item.category)])];
          const migratedStore = await invoke("clipboard_store_migrate_legacy", {
            legacyRecords,
            legacyCategories,
            preferences: legacyPreferences
          });

          const idToName = new Map();
          for (const cat of migratedStore.categories) {
            idToName.set(cat.id, cat.name);
          }
          categories = migratedStore.categories.map(/** @param {{id: string, name: string}} c */ (c) => c.name);
          if (!categories.includes(DEFAULT_CATEGORY)) {
            categories = [DEFAULT_CATEGORY, ...categories];
          }
          history = migratedStore.records.map(/** @param {{type: string, content: string, id: number, timestamp: string, categoryId: string, restored?: boolean, isPinned?: boolean}} r */ (r) => ({
            type: r.type,
            content: r.content,
            id: r.id,
            timestamp: r.timestamp,
            category: idToName.get(r.categoryId) || DEFAULT_CATEGORY,
            restored: !!r.restored,
            isPinned: !!r.isPinned
          }));
          if (migratedStore.preferences && typeof migratedStore.preferences.captureEnabled === "boolean") {
            captureEnabled = migratedStore.preferences.captureEnabled;
            retentionHours = migratedStore.preferences.retentionHours;
            officialWebsite = migratedStore.preferences.officialWebsite;
          }

          localStorage.removeItem(HISTORY_KEY);
          localStorage.removeItem(CATEGORIES_KEY);
          localStorage.removeItem(PREFERENCES_KEY);
        }
        storeReady = true;
      } catch (err) {
        storeError = `History could not be loaded. Existing data was preserved: ${String(err)}`;
        return;
      }

      const savedLastData = localStorage.getItem(LAST_DATA_KEY);
      if (savedLastData) lastData = savedLastData;

      const savedPwds = localStorage.getItem(PASSWORDS_KEY);
      if (!vaultExists && savedPwds) {
        try {
          passwords = JSON.parse(savedPwds);
        } catch {
          passwords = [];
        }
      }

      try {
        const status = await invoke("shortcut_status");
        shortcutValue = status.shortcut;
      } catch {}
      try { autostartEnabled = await invoke('autostart_status'); }
      catch (error) { autostartError = String(error); }

      updateAndSaveHistory(history);
      void migrateLegacyImageItems();

      let polling = false;
      clipboardInterval = setInterval(async () => {
        if (polling || clearDataBusy || resetPending || copyInFlight) return;
        const epoch = resetEpoch;
        const now = Date.now();
        const EXPIRATION_MS = retentionHours * 60 * 60 * 1000;
        if (
          history.some(
            (item) =>
              item.type === 'text' &&
              !item.restored &&
              !item.isPinned &&
              (!item.category || item.category === DEFAULT_CATEGORY) &&
              now - item.id > EXPIRATION_MS
          )
        ) {
          updateAndSaveHistory(history);
        }

        if (!captureEnabled) return;
        polling = true;
        try {
          const data = await invoke('get_clipboard_snapshot');
          captureError = '';
          if (disposed || !captureEnabled || clearDataBusy || resetPending || copyInFlight || epoch !== resetEpoch) return;
          if (data) {
            const { type, content } = data;
            const safe = sanitizeClipboardHtml(data.html);
            const signature = clipboardSignature(type, content, safe.html);
            if (signature !== lastData) {
              if (isAppCopying && type === 'image') {
                lastData = signature;
                isAppCopying = false;
                updateAndSaveHistory(history);
                return;
              }

              let storedContent = content;
              if (type === 'image') {
                storedContent = await persistImageContent(content);
              }
              if (clearDataBusy || resetPending || epoch !== resetEpoch) return;

              const defaultCat =
                type === 'text' && activeCategory !== DEFAULT_CATEGORY ? activeCategory : DEFAULT_CATEGORY;
              const newItem = {
                type,
                content: storedContent,
                html: safe.html,
                id: Date.now(),
                timestamp: new Date().toISOString(),
                category: defaultCat
              };

              lastData = signature;
              updateAndSaveHistory([newItem, ...history]);
              if (data.warning || safe.warning) triggerToast(data.warning || safe.warning);
            }
          }
        } catch (error) {
          captureError = `Clipboard could not be read: ${String(error)}`;
        } finally { polling = false; }
      }, 2000);

      vaultLockInterval = setInterval(() => {
        if (vaultRequirePassword && vaultUnlocked && Date.now() - vaultLastActivity >= VAULT_AUTO_LOCK_MS) {
          void lockVault();
        }
      }, 30_000);

      /** @param {ClipboardEvent} e */
      handlePaste = (e) => {
        if (!captureEnabled || clearDataBusy || resetPending || copyInFlight) return;
        const epoch = resetEpoch;
        const active = document.activeElement;
        if (active && (active.tagName === 'INPUT' || active.tagName === 'TEXTAREA')) return;
        if (!e.clipboardData) return;
        const safe = sanitizeClipboardHtml(e.clipboardData.getData('text/html'));

        const items = e.clipboardData.items;
        for (let i = 0; i < items.length; i++) {
          if (items[i].type.includes('image')) {
            const blob = items[i].getAsFile();
            if (!blob) continue;
            const reader = new FileReader();
            reader.onload = async (event) => {
              if (epoch !== resetEpoch || clearDataBusy || resetPending) return;
              const result = event?.target?.result;
              if (typeof result !== 'string') return;
              const base64 = result.split(',')[1];
              if (!base64) return;

              const content = `image|${base64}`;
              const signature = clipboardSignature('image', content);
              if (signature !== lastData) {
                let storedContent = content;
                try {
                  storedContent = await persistImageContent(content);
                } catch {
                  return;
                }
                const newItem = {
                  type: 'image',
                  content: storedContent,
                  id: Date.now(),
                  timestamp: new Date().toISOString(),
                  category: DEFAULT_CATEGORY
                };
                if (epoch !== resetEpoch || clearDataBusy || resetPending) return;
                lastData = signature;
                updateAndSaveHistory([newItem, ...history]);
                triggerToast('Image Pasted!');
              }
            };
            reader.readAsDataURL(blob);
          } else if (items[i].type === 'text/plain') {
            /** @param {string} text */
            items[i].getAsString((text) => {
              if (epoch !== resetEpoch || clearDataBusy || resetPending) return;
              const signature = clipboardSignature('text', text, safe.html);
              if (signature !== lastData) {
                const defaultCat = activeCategory !== DEFAULT_CATEGORY ? activeCategory : DEFAULT_CATEGORY;
                const newItem = {
                  type: 'text',
                  content: text,
                  html: safe.html,
                  id: Date.now(),
                  timestamp: new Date().toISOString(),
                  category: defaultCat
                };
                lastData = signature;
                updateAndSaveHistory([newItem, ...history]);
                triggerToast(safe.warning || 'Text Pasted!');
              }
            });
          }
        }
      };

      window.addEventListener('paste', handlePaste);
    };

    void init();

    return () => {
      disposed = true;
      if (clipboardInterval) clearInterval(clipboardInterval);
      if (vaultLockInterval) clearInterval(vaultLockInterval);
      if (handlePaste) window.removeEventListener('paste', handlePaste);
    };
  });

  /** @param {string} [msg] */
  function triggerToast(msg = 'Copied!') {
    toastMsg = msg;
    showToast = true;
    setTimeout(() => {
      showToast = false;
    }, 900);
  }

  /** @param {string} text */
  async function copyText(text) {
    if (!text || clearDataBusy || resetPending) return;
    copyInFlight++;
    try {
    await invoke('set_clipboard_text', { text });
    lastData = clipboardSignature('text', text);
    updateAndSaveHistory(history);
    triggerToast('Copied');
    } catch (error) { triggerToast(`Copy failed: ${String(error)}`); }
    finally { copyInFlight--; }
  }

  /** @param {string} text @param {string=} html */
  async function copyRichText(text, html) {
    if (clearDataBusy || resetPending) return;
    copyInFlight++;
    try {
      const safe = toClipboardHtml(text, html);
      await invoke('set_clipboard_rich_text', { text, html: safe });
      const actual = await invoke('get_clipboard_snapshot');
      if (actual?.type === 'text' && actual.content === text) {
        lastData = clipboardSignature('text', text, sanitizeClipboardHtml(actual.html).html);
        localStorage.setItem(LAST_DATA_KEY, lastData);
      }
      triggerToast('Copied as HTML');
    } catch (error) { triggerToast(`Copy failed: ${String(error)}`); }
    finally { copyInFlight--; }
  }

  /** @param {string} content */
  async function copyImage(content) {
    if (!content || clearDataBusy || resetPending) return;
    copyInFlight++;
    try {
      isAppCopying = true;
      if (isFileImageContent(content)) {
        await invoke('set_clipboard_image_from_path', { path: content.slice(5) });
      } else {
        const base64 = content.includes('|') ? content.split('|')[1] : content;
        await invoke('set_clipboard_image', { base64 });
      }
      triggerToast('Image Copied');
      setTimeout(() => {
        isAppCopying = false;
      }, 3000);
    } catch {
      triggerToast('Failed to copy');
      isAppCopying = false;
    } finally { copyInFlight--; }
  }

  /** @param {number} id */
  async function deleteItem(id) {
    if (clearDataBusy || resetPending) return;
    const item = history.find((it) => it.id === id);
    const newHistory = history.filter((it) => it.id !== id);
    if (previewImage?.id === id) closeImagePreview();
    if (targetImage?.id === id) {
      targetImage = null;
      closeImageMenu();
    }
    if (selectedClipId === id) selectedClipId = null;
    if (item?.type === 'text' && clipboardSignature('text', item.content, item.html) === lastData) {
      await invoke('set_clipboard_text', { text: '' });
      lastData = '';
    }
    updateAndSaveHistory(newHistory);
  }

  async function clearImages() {
    if (clearDataBusy || resetPending) throw new Error('App reset is in progress');
    const removedImages = history.filter((item) => item.type !== 'text');
    const newHistory = history.filter((item) => item.type === 'text');
    if (previewImage) closeImagePreview();
    targetImage = null;
    closeImageMenu();
    if (lastData.startsWith('image:')) {
      await invoke('set_clipboard_text', { text: '' });
      lastData = '';
    }
    const previous = history;
    history = newHistory;
    if (!await persistClipboardStore()) { history = previous; throw new Error(storeError || 'Could not save image deletion'); }
    const paths = collectImagePaths(removedImages);
    if (paths.length) await invoke('delete_history_images', { paths });
    triggerToast('All images cleared');
  }

  async function clearText() {
    const toDelete = history.filter(
      (item) =>
        item.type === 'text' &&
        (activeCategory === DEFAULT_CATEGORY || (item.category || DEFAULT_CATEGORY) === activeCategory)
    );
    const hasLastData = toDelete.some((it) => it.content === lastData);
    const newHistory = history.filter((item) => !toDelete.includes(item));

    if (hasLastData) {
      await invoke('set_clipboard_text', { text: '' });
      lastData = '';
    }
    updateAndSaveHistory(newHistory);
  }

  function addCategory() {
    const trimmed = newCatName.trim();
    if (trimmed && !categories.includes(trimmed)) {
      categories = [...categories, trimmed];
      saveCategories();
    }
    isAddingCat = false;
    newCatName = '';
  }

  /**
   * @param {number} id
   * @param {string} newCat
   */
  function changeCategory(id, newCat) {
    const newHistory = history.map((item) => (item.id === id ? { ...item, category: newCat } : item));
    updateAndSaveHistory(newHistory);
  }

  /**
   * @param {MouseEvent} e
   * @param {string} cat
   */
  function handleContextMenu(e, cat) {
    if (cat === DEFAULT_CATEGORY) return;
    targetCategory = cat;
    menuX = e.clientX;
    menuY = e.clientY;
    showImageMenu = false;
    showContextMenu = true;
  }

  function closeContextMenu() {
    showContextMenu = false;
  }

  /**
   * @param {MouseEvent} e
   * @param {HistoryItem} item
   */
  function handleImageContextMenu(e, item) {
    targetImage = item;
    imageMenuX = e.clientX;
    imageMenuY = e.clientY;
    showContextMenu = false;
    showImageMenu = true;
  }

  function closeImageMenu() {
    showImageMenu = false;
  }

  /** @param {HistoryItem | null} [item] */
  function openImagePreview(item = targetImage) {
    if (!item) return;
    previewImage = item;
    previewScale = 1;
    closeImageMenu();
  }

  function closeImagePreview() {
    previewImage = null;
    previewScale = 1;
  }

  /** @param {number} delta */
  function zoomPreview(delta) {
    previewScale = clampPreviewScale(previewScale + delta);
  }

  /** @param {WheelEvent} event */
  function handlePreviewWheel(event) {
    const delta = event.deltaY < 0 ? 0.15 : -0.15;
    zoomPreview(delta);
  }

  /** @param {KeyboardEvent} event */
  function handleWindowKeydown(event) {
    noteVaultActivity();
    if (event.key === 'Escape') {
      closeImageMenu();
      closeContextMenu();
      if (previewImage) closeImagePreview();
    }
  }

  /** @param {MouseEvent} event */
  function handlePreviewBackdropMouseDown(event) {
    if (event.target === event.currentTarget) {
      closeImagePreview();
    }
  }

  function startRename() {
    renameCatName = targetCategory;
    isRenamingCat = true;
    closeContextMenu();
  }

  function confirmRename() {
    const trimmed = renameCatName.trim();
    if (trimmed && trimmed !== targetCategory && !categories.includes(trimmed)) {
      categories = categories.map((cat) => (cat === targetCategory ? trimmed : cat));
      const newHistory = history.map((item) =>
        item.category === targetCategory ? { ...item, category: trimmed } : item
      );
      if (activeCategory === targetCategory) activeCategory = trimmed;
      saveCategories();
      updateAndSaveHistory(newHistory);
    }
    isRenamingCat = false;
  }

  function deleteCategoryFromMenu() {
    const cat = targetCategory;
    categories = categories.filter((name) => name !== cat);
    const newHistory = history.map((item) =>
      item.category === cat ? { ...item, category: DEFAULT_CATEGORY } : item
    );
    saveCategories();
    if (activeCategory === cat) activeCategory = DEFAULT_CATEGORY;
    updateAndSaveHistory(newHistory);
    closeContextMenu();
  }

  async function addPassword() {
    if (!newPwdTitle.trim() || !newPwdPass.trim()) return;
    const newItem = {
      id: Date.now(),
      title: newPwdTitle.trim(),
      username: newPwdUser.trim(),
      password: newPwdPass,
      showPass: false
    };
    const previous = passwords;
    passwords = [newItem, ...passwords];
    if (!await savePasswords()) { passwords = previous; return false; }
    newPwdTitle = '';
    newPwdUser = '';
    newPwdPass = '';
    showNewPwd = false;
    triggerToast('Saved');
    return newItem.id;
  }

  /** @param {PasswordItem} item */
  function startEditPassword(item) {
    editingPasswordId = item.id;
    editPwdTitle = item.title;
    editPwdUser = item.username;
    editPwdPass = item.password;
  }

  function cancelEditPassword() {
    editingPasswordId = null;
    editPwdTitle = '';
    editPwdUser = '';
    editPwdPass = '';
  }

  async function saveEditedPassword() {
    if (editingPasswordId === null || !editPwdTitle.trim() || !editPwdPass.trim()) return;
    const previous = passwords;
    passwords = passwords.map((pwd) =>
      pwd.id === editingPasswordId
        ? {
            ...pwd,
            title: editPwdTitle.trim(),
            username: editPwdUser.trim(),
            password: editPwdPass
          }
        : pwd
    );
    if (!await savePasswords()) { passwords = previous; return false; }
    cancelEditPassword();
    triggerToast('Updated');
    return true;
  }

  /** @param {number} id */
  async function deletePassword(id) {
    if (!vaultUnlocked || vaultSaving || vaultBusy) return false;
    return saveVaultSnapshot(passwords.filter((pwd) => pwd.id !== id), vaultCollections);
  }

  function clearAllPasswords() {
    passwords = [];
    savePasswords();
  }

  function exportPasswords() {
    if (clearDataBusy || resetPending || vaultSaving) return;
    if (!vaultUnlocked) { activeTab = 'passwords'; triggerToast('Unlock the vault before exporting'); return; }
    const data = {
      format: PASSWORD_EXPORT_FORMAT,
      version: PASSWORD_EXPORT_VERSION,
      exportedAt: new Date().toISOString(),
      collections: vaultCollections,
      passwords: passwords.map(({ title, username, password, collectionId }) => ({ title, username, password, collectionId: collectionId || null }))
    };
    const url = URL.createObjectURL(new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' }));
    const link = document.createElement('a');
    link.href = url;
    link.download = `my-clipboard-passwords-${data.exportedAt.slice(0, 10)}.json`;
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 0);
    triggerToast(`Exported ${passwords.length} password${passwords.length === 1 ? '' : 's'}`);
  }

  function requestPasswordImport() {
    if (clearDataBusy || resetPending || vaultSaving) return;
    if (!vaultUnlocked) { activeTab = 'passwords'; triggerToast('Unlock the vault before importing'); return; }
    passwordImportInput?.click();
  }

  /** @param {unknown} value */
  function isPasswordRecord(value) {
    return typeof value === 'object' && value !== null && !Array.isArray(value);
  }

  /** @param {Event & { currentTarget: HTMLInputElement }} event */
  async function importPasswords(event) {
    if (!vaultUnlocked || vaultSaving || vaultBusy || clearDataBusy || resetPending) return;
    const epoch = resetEpoch;
    const file = event.currentTarget.files?.[0];
    event.currentTarget.value = '';
    if (!file) return;
    if (file.size > MAX_PASSWORD_IMPORT_BYTES) {
      triggerToast('Import file is too large');
      return;
    }
    try {
      const parsed = JSON.parse(await file.text());
      if (epoch !== resetEpoch || !vaultUnlocked || clearDataBusy || resetPending) return;
      const entries = Array.isArray(parsed) ? parsed : parsed?.passwords;
      if (
        (!Array.isArray(parsed) &&
          (parsed?.format !== PASSWORD_EXPORT_FORMAT || ![1, PASSWORD_EXPORT_VERSION].includes(parsed?.version))) ||
        !Array.isArray(entries)
      ) {
        throw new Error('unsupported file');
      }

      const seen = new Set(passwords.map((item) => `${item.title}\u0000${item.username}\u0000${item.password}`));
      const usedIds = new Set(passwords.map((item) => item.id));
      const nextCollections = vaultCollections.map(item => ({ ...item }));
      /** @type {Map<string, string>} */
      const collectionMap = new Map();
      if (parsed?.version === 2) {
        if (!Array.isArray(parsed.collections) || parsed.collections.length > 100) throw new Error('Invalid collections');
        for (const collection of parsed.collections) {
          if (typeof collection?.id !== 'string' || typeof collection?.name !== 'string' || collectionMap.has(collection.id)) throw new Error('Invalid collection');
          const name = collection.name.trim();
          if (!name || new TextEncoder().encode(name).length > 120) throw new Error('Invalid collection name');
          let target = nextCollections.find(item => item.name.toLowerCase() === name.toLowerCase());
          if (!target) { target = { id: crypto.randomUUID(), name }; nextCollections.push(target); }
          collectionMap.set(collection.id, target.id);
        }
        if (nextCollections.length > 100) throw new Error('Too many collections');
      }
      let nextId = Date.now();
      let skipped = 0;
      /** @type {PasswordItem[]} */
      const imported = [];

      for (const entry of entries.slice(0, MAX_IMPORTED_PASSWORDS)) {
        if (!isPasswordRecord(entry) || typeof entry.title !== 'string' || typeof entry.password !== 'string') {
          skipped++;
          continue;
        }

        const title = entry.title.trim();
        const username = typeof entry.username === 'string' ? entry.username.trim() : '';
        const password = entry.password;
        const signature = `${title}\u0000${username}\u0000${password}`;
        if (!title || !password || title.length > 500 || username.length > 500 || password.length > 2000 || seen.has(signature)) {
          skipped++;
          continue;
        }

        while (usedIds.has(nextId)) nextId++;
        const collectionId = parsed?.version === 2 && entry.collectionId ? collectionMap.get(entry.collectionId) : null;
        if (parsed?.version === 2 && entry.collectionId && !collectionId) throw new Error('Missing collection');
        imported.push({ id: nextId++, title, username, password, collectionId, showPass: false });
        seen.add(signature);
      }

      skipped += Math.max(0, entries.length - MAX_IMPORTED_PASSWORDS);
      if (imported.length === 0 && nextCollections.length === vaultCollections.length) {
        triggerToast(skipped ? 'No new passwords imported' : 'No passwords found');
        return;
      }

      if (imported.length + passwords.length > MAX_IMPORTED_PASSWORDS) throw new Error('Too many passwords');
      if (!await saveVaultSnapshot([...imported, ...passwords], nextCollections)) return;
      cancelEditPassword();
      triggerToast(`Imported ${imported.length}${skipped ? `, skipped ${skipped}` : ''}`);
    } catch {
      triggerToast('Invalid password export file');
    }
  }

  /** @param {number} id */
  function togglePassword(id) {
    passwords = passwords.map((pwd) => (pwd.id === id ? { ...pwd, showPass: !pwd.showPass } : pwd));
  }

  /** @param {number} id */
  function selectClip(id) {
    selectedClipId = id;
  }

  /** @param {number} id */
  function togglePinnedClip(id) {
    updateAndSaveHistory(history.map((item) => (item.id === id ? { ...item, isPinned: !item.isPinned } : item)));
  }

  async function copySelectedClipText() {
    const selectedText = window.getSelection()?.toString().trim();
    if (!selectedText) {
      triggerToast('Select text to copy');
      return;
    }
    await copyText(selectedText);
  }

  /** @param {string} content */
  function getImgSrc(content) {
    try {
      if (isFileImageContent(content)) {
        const path = content.slice(5);
        return convertFileSrc(path);
      }
      const base64 = content.includes('|') ? content.split('|')[1] : content;
      return `data:image/png;base64,${base64}`;
    } catch {
      return '';
    }
  }

  $: filteredImages = history
    .filter((item) => item.type === 'image')
    .filter((item) => activeCategory === DEFAULT_CATEGORY || (item.category || DEFAULT_CATEGORY) === activeCategory)
    .filter((item) => !searchQuery || `${item.timestamp} ${item.content.startsWith('file|') ? item.content.slice(5) : 'PNG image'}`.toLowerCase().includes(searchQuery.toLowerCase()));
  $: displayedText = history
    .filter((item) => item.type !== 'image')
    .filter((item) => activeCategory === DEFAULT_CATEGORY || (item.category || DEFAULT_CATEGORY) === activeCategory)
    .filter((item) => recentFilter !== 'pinned' || item.isPinned)
    .filter((item) => !searchQuery || item.content.toLowerCase().includes(searchQuery.toLowerCase()));
  $: selectedClip = displayedText.find((item) => item.id === selectedClipId) || displayedText[0] || null;
  $: filteredPasswords = pwdSearchQuery
    ? passwords.filter((pwd) =>
        `${pwd.title} ${pwd.username}`.toLowerCase().includes(pwdSearchQuery.toLowerCase())
      )
    : passwords;

  /** @param {number | null} id @param {string} title @param {string} username @param {string} password */
  async function saveReferencePassword(id, title, username, password) {
    if (!vaultUnlocked || vaultSaving || vaultBusy || !title.trim() || !password) return false;
    const existing = id === null ? null : passwords.find(item => item.id === id);
    if (id !== null && !existing) { vaultError = 'Password no longer exists'; return false; }
    let nextId = id ?? Date.now();
    if (id === null) while (passwords.some(item => item.id === nextId)) nextId++;
    const item = { id: nextId, title: title.trim(), username: username.trim(), password, showPass: false, collectionId: existing?.collectionId || (id === null ? activePasswordCollection || null : null) };
    const next = id === null ? [item, ...passwords] : passwords.map(row => row.id === id ? item : row);
    return await saveVaultSnapshot(next, vaultCollections) ? nextId : false;
  }

  /** @param {string | null} id @param {string} name */
  async function saveVaultCollection(id, name) {
    if (!vaultUnlocked || vaultSaving || vaultBusy) return false;
    name = name.trim();
    if (!name || new TextEncoder().encode(name).length > 120 || vaultCollections.some(item => item.id !== id && item.name.toLowerCase() === name.toLowerCase())) {
      vaultError = 'Use a non-empty, unique collection name (up to 120 UTF-8 bytes).';
      return false;
    }
    const collection = { id: id || crypto.randomUUID(), name };
    const next = id ? vaultCollections.map(item => item.id === id ? collection : item) : [...vaultCollections, collection];
    return saveVaultSnapshot(passwords, next);
  }

  /** @param {string} id */
  async function removeVaultCollection(id) {
    if (!vaultUnlocked || vaultSaving || vaultBusy) return false;
    const saved = await saveVaultSnapshot(passwords.map(item => item.collectionId === id ? { ...item, collectionId: null } : item), vaultCollections.filter(item => item.id !== id));
    if (saved && activePasswordCollection === id) activePasswordCollection = '';
    return saved;
  }

  /** @param {number} id @param {string | null} collectionId */
  async function movePassword(id, collectionId) {
    if (!vaultUnlocked || vaultSaving || vaultBusy) return false;
    return saveVaultSnapshot(passwords.map(item => item.id === id ? { ...item, collectionId } : item), vaultCollections);
  }

  /** @param {string} name */
  function addReferenceCollection(name) {
    newCatName = name;
    addCategory();
  }

  /** @param {string} name @param {string} next */
  function renameCollection(name, next) {
    targetCategory = name;
    renameCatName = next;
    confirmRename();
  }

  /** @param {string} name */
  function removeCollection(name) {
    targetCategory = name;
    deleteCategoryFromMenu();
  }
</script>

<svelte:window
  on:click={() => {
    noteVaultActivity();
    closeContextMenu();
    closeImageMenu();
  }}
  on:keydown={handleWindowKeydown}
/>

<ClipboardApp
  bind:activeTab
  bind:searchQuery
  bind:pwdSearchQuery
  bind:activeCategory
  bind:captureEnabled
  bind:recentFilter
  bind:masterPassword
  bind:masterPasswordConfirm
  bind:unlockPassword
  bind:clearDataConfirmation
  bind:shortcutValue
  bind:retentionHours
  bind:activePasswordCollection
  {vaultCollections}
  {vaultSaving}
  {resetPending}
  {storeReady}
  {storeError}
  {captureError}
  {settingsBusy}
  {autostartEnabled}
  {autostartError}
  saveRetentionFn={saveRetention}
  toggleAutostartFn={toggleAutostart}
  {categories}
  {vaultExists}
  {vaultUnlocked}
  {vaultRequirePassword}
  {vaultAutoUnlockAvailable}
  {vaultStatusReady}
  bind:vaultAccessError
  {vaultBusy}
  {vaultError}
  {clearDataBusy}
  {clearDataError}
  {shortcutBusy}
  {shortcutError}
  {showToast}
  {toastMsg}
  {history}
  {passwords}
  {selectedClip}
  {filteredImages}
  {filteredPasswords}
  copyTextFn={copyText}
  copyRichTextFn={copyRichText}
  copyImageFn={copyImage}
  deleteItemFn={deleteItem}
  clearImagesFn={clearImages}
  lockVaultFn={lockVault}
  requestPasswordImportFn={requestPasswordImport}
  exportPasswordsFn={exportPasswords}
  toggleCaptureFn={toggleCapture}
  openOfficialWebsiteFn={openOfficialWebsite}
  selectClipFn={selectClip}
  togglePinnedClipFn={togglePinnedClip}
  copySelectedClipTextFn={copySelectedClipText}
  getImgSrcFn={getImgSrc}
  setupVaultFn={setupVault}
  unlockVaultFn={unlockVault}
  setVaultRequirePasswordFn={setVaultRequirePassword}
  savePasswordFn={saveReferencePassword}
  saveVaultCollectionFn={saveVaultCollection}
  removeVaultCollectionFn={removeVaultCollection}
  movePasswordFn={movePassword}
  deletePasswordFn={deletePassword}
  addCollectionFn={addReferenceCollection}
  renameCollectionFn={renameCollection}
  removeCollectionFn={removeCollection}
  changeCategoryFn={changeCategory}
  clearDataFn={clearAllAppData}
  updateShortcutFn={updateShortcut}
/>

<input hidden type="file" accept="application/json,.json" bind:this={passwordImportInput} on:change={importPasswords} />
