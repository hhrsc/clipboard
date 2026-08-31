<script>
  import { toMarkdown, toClipboardHtml } from '$lib/clipboard-format';
  import { responsivePopover } from '$lib/responsive-popover';
  import { onMount, tick } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { isTauri } from '@tauri-apps/api/core';
  import { IconMarkdown } from '@tabler/icons-svelte';
  import {
    Inbox as IconArchive, BookOpen as IconBook, BriefcaseBusiness as IconBriefcase,
    Lightbulb as IconBulb, Calendar as IconCalendar, ChevronDown as IconChevronDown,
    Clock3 as IconClock, CodeXml as IconCode, Copy as IconCopy,
    CreditCard as IconCreditCard, Ellipsis as IconDots, Download as IconDownload,
    ExternalLink as IconExternalLink, Eye as IconEye, File as IconFile,
    FileText as IconFileText, ListFilter as IconFilter2, Folder as IconFolder,
    Code as IconHtml, LockKeyhole as IconLock, MessageCircle as IconMessage,
    Minus as IconMinus, Menu as IconMenu, Pencil as IconPencil, Image as IconPhoto,
    Pilcrow as IconPilcrow, Pin as IconPin, Plus as IconPlus,
    PencilRuler as IconRulerMeasure, Search as IconSearch, Settings as IconSettings,
    ShoppingBag as IconShoppingBag, Square as IconSquare, Trash2 as IconTrash,
    Type as IconType, Upload as IconUpload, UserRound as IconUser, X as IconX
  } from '@lucide/svelte';

  export let activeTab = 'recent';
  /** @type {Array<{id: number, type?: string, content: string, timestamp?: string, time?: string, category?: string, tag?: string, code?: boolean, thumb?: string, isPinned?: boolean}>} */
  export let history = [];
  /** @type {Array<{id: number, title: string, username: string, password: string, time?: string, showPass?: boolean}>} */
  export let passwords = [];
  export let activeCategory = '全部';
  export let activePasswordCollection = '';
  /** @type {Array<{id: string, name: string}>} */
  export let vaultCollections = [];
  export let vaultSaving = false;
  export let resetPending = false;
  /** @type {(id: string | null, name: string) => Promise<boolean>} */
  export let saveVaultCollectionFn = async (id, name) => false;
  /** @type {(id: string) => Promise<boolean>} */
  export let removeVaultCollectionFn = async (id) => false;
  /** @type {(id: number, collectionId: string | null) => Promise<boolean>} */
  export let movePasswordFn = async (id, collectionId) => false;
  /** @type {(text: string, html?: string) => Promise<void>} */
  export let copyRichTextFn = async (text, html) => {};
  export let searchQuery = '';
  export let pwdSearchQuery = '';
  export let captureEnabled = true;
  export let retentionHours = 24;
  export let storeReady = false;
  export let storeError = '';
  export let captureError = '';
  export let settingsBusy = false;
  export let autostartEnabled = false;
  export let autostartError = '';
  export let saveRetentionFn = async () => {};
  export let toggleAutostartFn = async () => {};
  export let categories = ['全部'];
  export let recentFilter = 'all';
  export let vaultExists = false;
  export let vaultUnlocked = false;
  export let vaultRequirePassword = true;
  export let vaultAutoUnlockAvailable = false;
  export let vaultStatusReady = false;
  export let vaultAccessError = '';
  /** @type {(required: boolean, password: string) => Promise<boolean>} */
  export let setVaultRequirePasswordFn = async (required, password) => false;
  export let vaultBusy = false;
  export let vaultError = '';
  export let masterPassword = '';
  export let masterPasswordConfirm = '';
  export let unlockPassword = '';
  export let clearDataConfirmation = '';
  export let clearDataBusy = false;
  export let clearDataError = '';
  export let shortcutValue = 'Alt+C';
  export let shortcutBusy = false;
  export let shortcutError = '';
  export let showToast = false;
  export let toastMsg = '';
  export let setupVaultFn = async () => {};
  export let unlockVaultFn = async () => {};
  export let clearDataFn = async () => {};
  /** @type {(shortcut: string) => Promise<boolean>} */
  export let updateShortcutFn = async (shortcut) => false;
  /** @type {(id: number | null, title: string, username: string, password: string) => Promise<number | boolean | undefined>} */
  export let savePasswordFn = async (id, title, username, password) => undefined;
  /** @type {(id: number) => Promise<boolean>} */
  export let deletePasswordFn = async (id) => false;
  /** @type {(name: string) => void} */
  export let addCollectionFn = (name) => {};
  /** @type {(name: string, next: string) => void} */
  export let renameCollectionFn = (name, next) => {};
  /** @type {(name: string) => void} */
  export let removeCollectionFn = (name) => {};
  /** @type {(id: number, category: string) => void} */
  export let changeCategoryFn = (id, category) => {};
  /** @type {{id: number, type?: string, content: string, timestamp?: string, time?: string, category?: string, tag?: string, code?: boolean, thumb?: string, isPinned?: boolean} | null} */
  export let selectedClip = null;
  /** @type {Array<{id: number, type?: string, content: string, timestamp?: string, time?: string, category?: string, tag?: string, code?: boolean, thumb?: string, isPinned?: boolean}>} */
  export let filteredImages = [];
  /** @type {Array<{id: number, title: string, username: string, password: string, time?: string, showPass?: boolean}>} */
  export let filteredPasswords = [];
  /** @type {(text: string) => Promise<void>} */
  export let copyTextFn = async (text) => {};
  /** @type {(content: string) => Promise<void>} */
  export let copyImageFn = async (content) => {};
  /** @type {(id: number) => Promise<void>} */
  export let deleteItemFn = async (id) => {};
  export let clearImagesFn = async () => {};
  export let lockVaultFn = async () => {};
  export let requestPasswordImportFn = () => {};
  export let exportPasswordsFn = () => {};
  export let toggleCaptureFn = () => {};
  export let openOfficialWebsiteFn = async () => {};
  /** @type {(id: number) => void} */
  export let selectClipFn = (id) => {};
  /** @type {(id: number) => void} */
  export let togglePinnedClipFn = (id) => {};
  export let copySelectedClipTextFn = async () => {};
  /** @type {(content: string) => string} */
  export let getImgSrcFn = (content) => '';

  /** @type {ReturnType<typeof getCurrentWindow> | null} */
  let appWindow = null;
  /** @type {Array<{id: number, type?: string, content: string, timestamp?: string, time?: string, category?: string, tag?: string, code?: boolean, thumb?: string, isPinned?: boolean}>} */
  let referenceClips = [
    { id: 1, content: 'Design is not just what it looks like and feels like. Design is how it works.', time: '10:24 AM', category: 'All clips' },
    { id: 2, content: 'https://www.notion.so/help/keyboard-shortcuts', time: '10:15 AM', category: 'All clips' },
    { id: 3, content: 'invoice_0423_final.pdf', time: '9:42 AM', tag: 'PDF', category: 'All clips' },
    { id: 4, content: 'user@example.com', time: '9:15 AM', tag: 'EMAIL', category: 'All clips' },
    { id: 5, content: '••••••••••••••••••', time: '9:15 AM', tag: 'PASSWORD', category: 'All clips' },
    { id: 6, content: 'const clamp = (n, min, max) =>\nMath.min(Math.max(n, min), max);', time: '8:47 AM', tag: 'JS', code: true, category: 'Code' },
    { id: 7, content: 'The quick brown fox jumps over the lazy dog.', time: '8:31 AM', category: 'All clips' },
    { id: 8, content: '# Project Roadmap\nn- Research - Concepts - Prototyping\n- Testing - Launch', time: 'Yesterday', tag: 'MD', code: true, category: 'Work snippets' },
    { id: 9, type: 'image', content: 'IMG_2024_0521_1830.png', time: 'Yesterday', tag: 'PNG', thumb: '/reference-assets/recent-thumbnail.png', category: 'All clips' },
    { id: 10, content: '1Password – Home', time: 'Yesterday', tag: 'LOGIN', category: 'Personal' }
  ];
  /** @type {Array<{id: number, title: string, username: string, password: string, time?: string}>} */
  let referencePasswords = [
    { id: 1, title: 'Work Email', username: 'user@example.com', password: 'strong-password-x', time: '10:24 AM' },
    { id: 2, title: 'Project Management', username: 'user@example.com', password: 'strong-password-x', time: '9:47 AM' },
    { id: 3, title: 'Cloud Storage', username: 'user@example.com', password: 'strong-password-x', time: 'Yesterday' },
    { id: 4, title: 'Online Banking', username: 'user@example.com', password: 'strong-password-x', time: 'Yesterday' },
    { id: 5, title: 'Shopping Account', username: 'user@example.com', password: 'strong-password-x', time: 'Yesterday' },
    { id: 6, title: 'Social Media', username: 'user@example.com', password: 'strong-password-x', time: 'Yesterday' },
    { id: 7, title: 'Developer Portal', username: 'user@example.com', password: 'strong-password-x', time: '2 days ago' },
    { id: 8, title: 'Travel Rewards', username: 'user@example.com', password: 'strong-password-x', time: '2 days ago' }
  ];
  /** @type {Array<{id: number, src: string, content?: string, group?: string}>} */
  let referenceImages = [
    ...Array.from({ length: 8 }, (_, index) => ({ id: index + 1, src: `/reference-assets/images/today-${index + 1}.png`, group: 'Today' })),
    ...Array.from({ length: 4 }, (_, index) => ({ id: index + 9, src: `/reference-assets/images/yesterday-${index + 1}.png`, group: 'Yesterday' }))
  ];
  let recentCollections = [
    { label: 'All clips', count: 328, icon: IconArchive },
    { label: 'Common phrases', count: 24, icon: IconMessage },
    { label: 'Work snippets', count: 67, icon: IconFolder },
    { label: 'Code', count: 42, icon: IconCode },
    { label: 'References', count: 31, icon: IconBook },
    { label: 'Receipts', count: 16, icon: IconFileText },
    { label: 'Personal', count: 18, icon: IconUser },
    { label: 'Ideas', count: 12, icon: IconBulb }
  ];
  let passwordCollections = [
    { label: 'All items', count: 120, icon: IconFolder },
    { label: 'Work', count: 28, icon: IconBriefcase },
    { label: 'Personal', count: 42, icon: IconUser },
    { label: 'Finance', count: 18, icon: IconCreditCard },
    { label: 'Shopping', count: 12, icon: IconShoppingBag }
  ];

  let referenceMode = false;
  let selectedReferenceClipId = 1;
  let selectedPasswordId = 1;
  let selectedImageId = 1;
  let addingPassword = false;
  let draftTitle = '';
  let draftUsername = '';
  let draftPassword = '';
  let revealPassword = false;
  let passwordMenu = false;
  let passwordFilterMenu = false;
  let passwordFilter = 'all';
  let passwordSort = 'newest';
  /** @type {number | null} */
  let rowMenuId = null;
  let rowMenuTop = 0;
  let rowMenuLeft = 0;
  let choosingPasswordCollection = false;
  let confirmation = '';
  let confirmationCollection = '';
  let dialogError = '';
  let dialogBusy = false;
  let vaultAccessDialog = false;
  let vaultAccessTarget = true;
  let vaultAccessPassword = '';
  /** @type {HTMLInputElement | undefined} */
  let vaultAccessInput;
  let loadedPasswordKey = '';
  let collectionMenu = false;
  let addingCollection = false;
  let collectionName = '';
  let editingCollection = '';
  let collectionContext = '';
  let collectionContextY = 0;
  let editingShortcut = false;
  let shortcutDraft = '';
  let shortcutHasKey = false;
  /** @type {HTMLInputElement | undefined} */
  let shortcutInput;
  let narrowWindow = false;
  let sidebarOpen = false;
  /** @type {HTMLButtonElement | undefined} */
  let navigationToggle;
  let filterMenu = false;
  let dateFilter = 'All time';
  let dateMenu = false;
  let imageDimensions = '';
  let imageSize = '';
  /** @type {number | null} */
  let imageCollectionId = null;
  let imageMenuTop = 0;
  let imageMenuLeft = 0;
  let referenceShortcut = 'Ctrl+Shift+V';
  let savingPassword = false;
  let clipMenu = false;
  let quickMenu = false;
  let failedImages = new Set();
  /** @type {HTMLInputElement | undefined} */
  let titleInput;

  onMount(() => {
    try {
      appWindow = getCurrentWindow();
    } catch {
      appWindow = null;
    }
    const page = new URLSearchParams(window.location.search).get('reference');
    referenceMode = import.meta.env.DEV && !isTauri() && ['recent', 'characters', 'images', 'passwords', 'settings'].includes(page || '');
    if (referenceMode) activeTab = page === 'characters' ? 'recent' : page || 'recent';
    const media = window.matchMedia('(max-width: 1100px)');
    const updateWidth = () => { narrowWindow = media.matches; if (!narrowWindow) sidebarOpen = false; };
    updateWidth();
    media.addEventListener('change', updateWidth);
    return () => media.removeEventListener('change', updateWidth);
  });

  function closeNavigation() {
    if (!sidebarOpen) return;
    sidebarOpen = false;
    navigationToggle?.focus();
  }

  function closePopovers() {
    collectionMenu = filterMenu = dateMenu = clipMenu = quickMenu = passwordMenu = passwordFilterMenu = false;
    imageCollectionId = null;
    rowMenuId = null;
    collectionContext = '';
    if (!shortcutBusy) editingShortcut = false;
  }

  async function openVaultAccessDialog() {
    closePopovers();
    vaultAccessTarget = !vaultRequirePassword;
    vaultAccessPassword = vaultAccessError = '';
    vaultAccessDialog = true;
    await tick();
    vaultAccessInput?.focus();
  }

  function closeVaultAccessDialog() {
    if (vaultBusy) return;
    vaultAccessDialog = false;
    vaultAccessPassword = vaultAccessError = '';
  }

  async function saveVaultAccess() {
    if (!vaultAccessPassword || vaultBusy) return;
    if (await setVaultRequirePasswordFn(vaultAccessTarget, vaultAccessPassword)) closeVaultAccessDialog();
  }

  $: if (clearDataBusy || resetPending) {
    vaultAccessDialog = false;
    vaultAccessPassword = '';
  }

  async function toggleShortcutEditor() {
    if (shortcutBusy || clearDataBusy || resetPending) return;
    editingShortcut = !editingShortcut;
    if (editingShortcut) {
      shortcutDraft = '';
      shortcutHasKey = false;
      await tick();
      shortcutInput?.focus();
    }
  }

  /** @param {KeyboardEvent} event */
  function recordShortcut(event) {
    if (shortcutBusy || clearDataBusy || resetPending) return;
    if (event.key === 'Tab' && !(event.ctrlKey || event.altKey || event.metaKey)) return;
    event.preventDefault();
    event.stopPropagation();
    if (event.key === 'Escape' && !(event.ctrlKey || event.altKey || event.shiftKey || event.metaKey)) {
      editingShortcut = false;
      return;
    }
    if (event.repeat || event.isComposing) return;
    const code = event.code || event.key;
    if (!code || code === 'Unidentified') return;
    const modifier = /^(Control|Alt|Shift|Meta)(Left|Right)$/.test(code) || ['Control', 'Alt', 'Shift', 'Meta'].includes(event.key);
    const key = /^(?:Key([A-Z])|Digit([0-9]))$/.exec(code);
    shortcutHasKey = !modifier;
    shortcutDraft = [event.ctrlKey && 'Ctrl', event.altKey && 'Alt', event.shiftKey && 'Shift', event.metaKey && 'Win', !modifier && (key ? key[1] || key[2] : code)].filter(Boolean).join('+');
  }

  async function applyShortcut() {
    if (!shortcutHasKey || shortcutBusy || clearDataBusy || resetPending) return;
    if (referenceMode) {
      referenceShortcut = shortcutDraft;
      editingShortcut = false;
    } else if (await updateShortcutFn(shortcutDraft)) editingShortcut = false;
  }

  /** @param {string | undefined} timestamp */
  function displayTime(timestamp) {
    if (!timestamp) return '';
    const parsed = new Date(timestamp);
    if (Number.isNaN(parsed.getTime())) return timestamp;
    return parsed.toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' });
  }

  /** @param {{id: number, timestamp?: string}} item */
  function imageGroup(item) {
    const date = new Date(item.timestamp || item.id);
    const today = new Date();
    const yesterday = new Date();
    yesterday.setDate(today.getDate() - 1);
    return date.toDateString() === today.toDateString() ? 'Today' : date.toDateString() === yesterday.toDateString() ? 'Yesterday' : 'Earlier';
  }

  /** @param {{type?: string, content: string}} item */
  function clipLabel(item) {
    return item.type === 'image' ? item.content.startsWith('file|') ? item.content.split(/[\\/]/).pop() || 'PNG image' : 'PNG image' : item.content;
  }

  /** @param {string} src */
  function failImage(src) { failedImages = new Set([...failedImages, src]); }

  /** @param {string} content */
  function inferTag(content) {
    if (/\.pdf$/i.test(content)) return 'PDF';
    if (/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(content)) return 'EMAIL';
    if (/^(const|let|var|function)\s/.test(content)) return 'JS';
    if (/^#\s/.test(content)) return 'MD';
    return '';
  }

  $: liveClips = history
    .filter((item) => item.type !== 'image')
    .filter((item) => activeCategory === '全部' || (item.category || '全部') === activeCategory)
    .filter((item) => recentFilter !== 'pinned' || item.isPinned)
    .filter((item) => item.content.toLowerCase().includes(searchQuery.toLowerCase())).map((item) => ({
    ...item,
    time: displayTime(item.timestamp),
    tag: item.type === 'image' ? 'PNG' : inferTag(item.content),
    thumb: item.type === 'image' ? getImgSrcFn(item.content) : undefined,
    code: /^(const|let|var|function|#\s)/.test(item.content)
  }));
  $: clipRows = referenceMode
    ? referenceClips.filter((item) => item.type !== 'image')
        .filter((item) => item.content.toLowerCase().includes(searchQuery.toLowerCase()))
        .filter((item) => activeCategory === '全部' || item.category === activeCategory)
        .filter((item) => recentFilter !== 'pinned' || item.isPinned)
    : liveClips;
  $: activeReferenceClip = clipRows.find((item) => item.id === selectedReferenceClipId) || clipRows[0] || null;
  $: detailClip = referenceMode ? activeReferenceClip : selectedClip;
  $: livePasswordRows = filteredPasswords
    .filter((item) => !activePasswordCollection || ('collectionId' in item && item.collectionId === activePasswordCollection))
    .filter((item) => passwordFilter === 'all' || (passwordFilter === 'with-username' ? !!item.username.trim() : !item.username.trim()))
    .map((item) => ({ ...item, time: displayTime(new Date(item.id).toISOString()) }))
    .sort((a, b) => passwordSort === 'title' ? a.title.localeCompare(b.title) || b.id - a.id : passwordSort === 'oldest' ? a.id - b.id : b.id - a.id);
  $: passwordRows = referenceMode
    ? referencePasswords.filter((item) => `${item.title} ${item.username}`.toLowerCase().includes(pwdSearchQuery.toLowerCase()))
    : livePasswordRows;
  $: selectedPassword = passwordRows.find((item) => item.id === selectedPasswordId) || passwordRows[0] || null;
  $: passwordKey = JSON.stringify([selectedPassword?.id, selectedPassword?.title, selectedPassword?.username, selectedPassword?.password]);
  $: if (!addingPassword && passwordKey !== loadedPasswordKey) {
    draftTitle = selectedPassword?.title || '';
    draftUsername = selectedPassword?.username || '';
    draftPassword = selectedPassword?.password || '';
    loadedPasswordKey = passwordKey;
  }
  $: liveImages = filteredImages.map((item) => ({ ...item, src: getImgSrcFn(item.content), group: imageGroup(item) }));
  $: imageRows = (referenceMode ? referenceImages : liveImages)
    .filter((item) => dateFilter === 'All time' || item.group === dateFilter)
    .filter((item) => !referenceMode || !searchQuery || `${item.src} ${item.group}`.toLowerCase().includes(searchQuery.toLowerCase()));
  $: selectedImage = imageRows.find((item) => item.id === selectedImageId) || imageRows[0] || null;
  $: reconcileImageSelection(imageRows);
  $: todayImages = imageRows.filter((item) => item.group === 'Today');
  $: yesterdayImages = imageRows.filter((item) => item.group === 'Yesterday');
  $: earlierImages = imageRows.filter((item) => item.group === 'Earlier');
  $: collectionHistory = history.filter(item => activeTab === 'images' ? item.type === 'image' : item.type !== 'image');
  $: if (imageCollectionId !== null && !imageRows.some(item => item.id === imageCollectionId)) imageCollectionId = null;
  $: activeCollections = referenceMode
    ? (activeTab === 'passwords' ? passwordCollections : recentCollections).map((item, index) => ({ ...item, id: index ? item.label : '' }))
    : activeTab === 'passwords'
      ? [{ id: '', label: 'All items', count: passwords.length, icon: IconFolder }, ...vaultCollections.map(item => ({ id: item.id, label: item.name, count: passwords.filter(password => 'collectionId' in password && password.collectionId === item.id).length, icon: IconFolder }))]
      : categories.map((category) => ({ id: category === '全部' ? '' : category, label: category === '全部' ? 'All clips' : category, count: category === '全部' ? collectionHistory.length : collectionHistory.filter((item) => item.category === category).length, icon: category === '全部' ? IconArchive : IconFolder }));
  $: menuPassword = (referenceMode ? referencePasswords : passwords).find(item => item.id === rowMenuId) || null;
  $: shortcutKeys = (referenceMode ? referenceShortcut : shortcutValue).split('+');
  $: if (!vaultUnlocked && !referenceMode) {
    addingPassword = false;
    draftTitle = draftUsername = draftPassword = loadedPasswordKey = '';
    revealPassword = false;
    rowMenuId = null;
    passwordMenu = passwordFilterMenu = false;
    if (activeTab === 'passwords') { addingCollection = false; collectionName = editingCollection = ''; }
    if (confirmation === 'collection') { confirmation = ''; confirmationCollection = ''; }
  }

  /** @param {{id: number}} item */
  function chooseClip(item) {
    if (referenceMode) selectedReferenceClipId = item.id;
    else selectClipFn(item.id);
  }

  /** @param {{id: number, src: string, content?: string, group?: string}} item */
  function chooseImage(item) {
    if (selectedImageId !== item.id) imageDimensions = imageSize = '';
    selectedImageId = item.id;
  }

  /** @param {{id: number, src: string}} item @param {MouseEvent} event */
  function openImageCollection(item, event) {
    if (referenceMode) return;
    closePopovers();
    chooseImage(item);
    imageCollectionId = item.id;
    imageMenuTop = Math.max(12, Math.min(event.clientY, innerHeight - 332));
    imageMenuLeft = Math.max(12, Math.min(event.clientX, innerWidth - 260));
  }

  /** @param {string} category */
  function assignImageCollection(category) {
    if (imageCollectionId !== null) changeCategoryFn(imageCollectionId, category || '全部');
    imageCollectionId = null;
  }

  /** @param {Array<{id: number}>} rows */
  function reconcileImageSelection(rows) {
    if (!rows.some(item => item.id === selectedImageId)) {
      selectedImageId = rows[0]?.id || 0;
      imageDimensions = imageSize = '';
    }
  }

  async function copyCurrentImage() {
    if (!selectedImage) return;
    if (referenceMode) {
      const blob = await (await fetch(selectedImage.src)).blob();
      await navigator.clipboard.write([new ClipboardItem({ [blob.type]: blob })]);
    }
    else copyImageFn(selectedImage.content || '');
  }

  function removeCurrentImage() {
    if (!selectedImage) return;
    if (referenceMode) referenceImages = referenceImages.filter((item) => item.id !== selectedImage.id);
    else deleteItemFn(selectedImage.id);
  }

  function beginPassword() {
    if (!referenceMode && !vaultUnlocked) return;
    addingPassword = true;
    draftTitle = draftUsername = draftPassword = '';
    revealPassword = false;
  }

  async function savePassword() {
    if (vaultSaving || vaultBusy || clearDataBusy || resetPending) return;
    if (!draftTitle.trim() || !draftPassword) return;
    const id = addingPassword ? Date.now() : selectedPassword?.id;
    if (!id) return;
    if (referenceMode) {
      const item = { id, title: draftTitle.trim(), username: draftUsername.trim(), password: draftPassword, time: 'Just now' };
      referencePasswords = addingPassword ? [item, ...referencePasswords] : referencePasswords.map((row) => row.id === id ? item : row);
    } else {
      savingPassword = true;
      const saved = await savePasswordFn(addingPassword ? null : id, draftTitle, draftUsername, draftPassword);
      savingPassword = false;
      if (!saved) return;
      selectedPasswordId = typeof saved === 'number' ? saved : id;
    }
    if (referenceMode) selectedPasswordId = id;
    addingPassword = false;
  }

  /** @param {number | undefined} [id] */
  async function removePassword(id = selectedPassword?.id) {
    if (!id || vaultSaving || vaultBusy) return;
    if (referenceMode) referencePasswords = referencePasswords.filter((item) => item.id !== id);
    else if (!await deletePasswordFn(id)) return;
    passwordMenu = false;
    rowMenuId = null;
  }

  async function addCollection() {
    if (!collectionName.trim()) return;
    dialogError = '';
    if (activeTab === 'passwords' && !referenceMode) {
      dialogBusy = true;
      const saved = await saveVaultCollectionFn(editingCollection || null, collectionName);
      dialogBusy = false;
      if (!saved) { dialogError = vaultError || 'Could not save collection'; return; }
    }
    else if (referenceMode) {
      const collection = { label: collectionName.trim(), count: 0, icon: IconFolder };
      if (activeTab === 'passwords') passwordCollections = [...passwordCollections, collection];
      else recentCollections = [...recentCollections, collection];
    }
    else if (editingCollection) renameCollectionFn(editingCollection, collectionName.trim());
    else addCollectionFn(collectionName);
    editingCollection = '';
    collectionName = '';
    addingCollection = false;
  }

  /** @param {string} id @param {MouseEvent} event */
  function openCollectionContext(id, event) {
    if (referenceMode || !id || (activeTab === 'passwords' && (!vaultUnlocked || vaultSaving))) return;
    collectionContext = id;
    collectionContextY = Math.min(event.clientY, innerHeight - 150);
  }

  /** @param {number} id @param {MouseEvent} event */
  function openPasswordMenu(id, event) {
    const bounds = event.currentTarget instanceof HTMLElement ? event.currentTarget.getBoundingClientRect() : null;
    rowMenuId = rowMenuId === id ? null : id;
    rowMenuTop = Math.max(10, Math.min(bounds?.bottom || event.clientY, innerHeight - 330));
    rowMenuLeft = Math.max(10, Math.min(bounds?.left || event.clientX, innerWidth - 260));
    choosingPasswordCollection = false;
    passwordMenu = false;
  }

  /** @param {number} id @param {string | null} collectionId */
  async function assignPasswordCollection(id, collectionId) {
    if (await movePasswordFn(id, collectionId)) { rowMenuId = null; passwordMenu = false; choosingPasswordCollection = false; }
  }

  async function confirmAction() {
    if (dialogBusy) return;
    dialogBusy = true;
    dialogError = '';
    try {
      if (confirmation === 'collection') {
        if (!await removeVaultCollectionFn(confirmationCollection)) { dialogError = vaultError || 'Could not delete collection'; return; }
      } else if (confirmation === 'images') await clearImagesFn();
      confirmation = '';
    } catch (error) { dialogError = String(error); }
    finally { dialogBusy = false; }
  }

  /** @param {string} category */
  function assignCollection(category) {
    if (!detailClip) return;
    if (referenceMode) referenceClips = referenceClips.map((item) => item.id === detailClip.id ? { ...item, category } : item);
    else changeCategoryFn(detailClip.id, category === 'All clips' ? '全部' : category);
    collectionMenu = false;
  }

  async function copySelection() {
    if (!referenceMode) return copySelectedClipTextFn();
    const text = window.getSelection()?.toString();
    if (text) await copyPlainText(text);
  }

  function pinClip() {
    if (!detailClip) return;
    if (referenceMode) referenceClips = referenceClips.map((item) => item.id === detailClip.id ? { ...item, isPinned: !item.isPinned } : item);
    else togglePinnedClipFn(detailClip.id);
  }

  function removeClip() {
    if (!detailClip) return;
    if (referenceMode) referenceClips = referenceClips.filter((item) => item.id !== detailClip.id);
    else deleteItemFn(detailClip.id);
    clipMenu = false;
  }

  function copyClip() {
    if (!detailClip) return;
    return detailClip.type === 'image' ? copyImageFn(detailClip.content) : copyPlainText(detailClip.content);
  }

  /** @param {'markdown' | 'html' | 'single-line'} format */
  function copyFormatted(format) {
    if (!detailClip) return;
    const content = detailClip.content;
    const html = 'html' in detailClip && typeof detailClip.html === 'string' ? detailClip.html : undefined;
    if (format === 'html') {
      if (referenceMode) return navigator.clipboard.write([new ClipboardItem({ 'text/html': new Blob([toClipboardHtml(content, html)], { type: 'text/html' }), 'text/plain': new Blob([content], { type: 'text/plain' }) })]);
      return copyRichTextFn(content, html);
    }
    const result = format === 'single-line' ? content.replace(/\s*(?:\r\n|\r|\n)\s*/g, ' ') : toMarkdown(content, html);
    return copyPlainText(result);
  }

  /** @param {Event} event */
  async function measureImage(event) {
    const image = event.currentTarget;
    if (!(image instanceof HTMLImageElement)) return;
    imageDimensions = `${image.naturalWidth} × ${image.naturalHeight}`;
    const imageId = selectedImage?.id;
    imageSize = '—';
    if (referenceMode) return;
    try {
      const response = await fetch(image.src);
      if (!response.ok) return;
      const bytes = (await response.blob()).size;
      if (selectedImage?.id === imageId) imageSize = bytes >= 1048576 ? `${(bytes / 1048576).toFixed(1)} MB` : `${Math.round(bytes / 1024)} KB`;
    } catch { imageSize = '—'; }
  }

  /** @param {string} text */
  async function copyPlainText(text) {
    if (referenceMode) {
      await navigator.clipboard?.writeText(text);
      return;
    }
    await copyTextFn(text);
  }

  function minimizeWindow() {
    appWindow?.minimize().catch(() => {});
  }

  function maximizeWindow() {
    appWindow?.toggleMaximize().catch(() => {});
  }

  function closeWindow() {
    appWindow?.close().catch(() => {});
  }

  const navItems = [
    { id: 'recent', label: 'Characters', icon: IconType },
    { id: 'images', label: 'Images', icon: IconPhoto },
    { id: 'passwords', label: 'Passwords', icon: IconLock },
    { id: 'settings', label: 'Settings', icon: IconSettings }
  ];
</script>

<svelte:window on:click={(event) => { collectionContext = ''; if (!(event.target instanceof Element) || !event.target.closest('.popover, [data-menu-trigger]')) { imageCollectionId = rowMenuId = null; passwordMenu = passwordFilterMenu = false; } }} on:keydown={(event) => { if (event.key === 'Escape') { closeVaultAccessDialog(); closeNavigation(); closePopovers(); addingCollection = false; editingCollection = ''; if (!dialogBusy) confirmation = ''; } }} />

<div class="ref-window" class:reference-mode={referenceMode}>
  <div class="compact-titlebar" data-tauri-drag-region></div>
  <button bind:this={navigationToggle} class="navigation-toggle" aria-label={sidebarOpen ? 'Close navigation' : 'Open navigation'} aria-expanded={sidebarOpen} aria-controls="app-sidebar" on:click={() => { closePopovers(); if (sidebarOpen) closeNavigation(); else sidebarOpen = true; }}>{#if sidebarOpen}<IconX size={22} strokeWidth={1.45} />{:else}<IconMenu size={22} strokeWidth={1.45} />{/if}</button>
  {#if sidebarOpen}<button class="drawer-shade" aria-label="Close navigation backdrop" tabindex="-1" on:click={closeNavigation}></button>{/if}
  <aside id="app-sidebar" class="ref-sidebar" class:sidebar-open={sidebarOpen} inert={clearDataBusy || resetPending || (narrowWindow && !sidebarOpen)}>
    <div class="ref-brand" data-tauri-drag-region>Clipboard</div>

    <nav class="ref-primary-nav" aria-label="Primary">
      {#each navItems as item}
        <button class:active={activeTab === item.id} on:click={() => { closePopovers(); activeTab = item.id; closeNavigation(); }}>
          <svelte:component this={item.icon} size={24} strokeWidth={1.55} />
          <span>{item.label}</span>
        </button>
      {/each}
    </nav>

    <div class="ref-collections-head">
      <span>Collections</span>
      <button aria-label="Add collection" disabled={!referenceMode && activeTab === 'passwords' && (!vaultUnlocked || vaultSaving || vaultBusy)} title="Add collection" on:click={() => { addingCollection = true; dialogError = ''; collectionName = editingCollection = ''; }}><IconPlus size={19} strokeWidth={1.45} /></button>
    </div>

    <nav class="ref-collection-list" aria-label="Collections">
      {#each activeCollections as item, index}
        <button
          class:active={activeTab === 'passwords' ? !referenceMode && activePasswordCollection === item.id : (activeTab === 'recent' || (!referenceMode && activeTab === 'images')) && (activeCategory === '全部' ? index === 0 : activeCategory === item.label)}
          on:click={() => { if (activeTab === 'passwords') activePasswordCollection = item.id; else activeCategory = index === 0 ? '全部' : item.label; closeNavigation(); }}
          on:contextmenu|preventDefault={(event) => openCollectionContext(item.id, event)}
        >
          <svelte:component this={item.icon} size={22} strokeWidth={1.45} />
          <span>{item.label}</span>
          <small>{item.count}</small>
        </button>
      {/each}
      {#if activeTab !== 'passwords'}
        <button class="new-collection" on:click={() => (addingCollection = true)}><IconPlus size={20} strokeWidth={1.45} /><span>New collection</span></button>
      {/if}
    </nav>

    {#if collectionContext}<div class="popover collection-context" style:top={`${collectionContextY}px`}><button on:click={() => { editingCollection = collectionContext; collectionName = activeCollections.find(item => item.id === collectionContext)?.label || collectionContext; dialogError = ''; addingCollection = true; }}>Rename collection</button><button on:click={() => { if (activeTab === 'passwords') { confirmationCollection = collectionContext; confirmation = 'collection'; dialogError = ''; } else removeCollectionFn(collectionContext); }}>Delete collection</button></div>{/if}

    {#if activeTab === 'passwords'}
      <button class="sidebar-lock" title={vaultRequirePassword ? 'Lock vault' : 'Enable password requirement in Settings to lock the vault'} disabled={!referenceMode && (!vaultRequirePassword || !vaultUnlocked || vaultBusy)} on:click={() => !referenceMode && lockVaultFn()}><IconLock size={22} strokeWidth={1.45} />Lock vault</button>
    {/if}

    <button class="ref-sidebar-footer" on:click={() => { if (activeTab !== 'images') activeTab = 'recent'; activeCategory = '全部'; recentFilter = 'all'; dateFilter = 'All time'; searchQuery = ''; closePopovers(); closeNavigation(); }}>
      <span>{activeTab === 'passwords' ? (referenceMode ? '120 items' : `${passwords.length} items`) : (referenceMode ? '328 clips' : `${collectionHistory.length} ${activeTab === 'images' ? 'images' : 'clips'}`)}</span>
      <IconChevronDown size={16} strokeWidth={1.45} />
    </button>
  </aside>

  <div class="window-controls" data-tauri-drag-region inert={sidebarOpen}>
    <button aria-label="Minimize" on:click={minimizeWindow}><IconMinus size={17} strokeWidth={1.35} /></button>
    <button aria-label="Maximize" on:click={maximizeWindow}><IconSquare size={15} strokeWidth={1.35} /></button>
    <button aria-label="Close" on:click={closeWindow}><IconX size={18} strokeWidth={1.35} /></button>
  </div>

  {#if activeTab !== 'images'}
    <header class="ref-topbar" data-tauri-drag-region inert={sidebarOpen}>
      <label class="ref-search">
        <IconSearch size={20} strokeWidth={1.45} />
        {#if activeTab === 'passwords'}
          <input bind:value={pwdSearchQuery} placeholder="Search passwords" />
        {:else}
          <input bind:value={searchQuery} placeholder="Search your clipboard" />
        {/if}
      </label>
    </header>
  {/if}

  {#if activeTab === 'recent'}
    <main class="ref-main recent-page" inert={sidebarOpen}>
      <section class="clip-column">
        <header class="column-title"><h2>Characters</h2><button class="filter-button" aria-label="Filter clips" on:click={() => (filterMenu = !filterMenu)}><IconFilter2 size={19} strokeWidth={1.45} /></button></header>
        {#if filterMenu}<div class="popover clip-filter" use:responsivePopover={'.filter-button'}><button on:click={() => { recentFilter = 'all'; filterMenu = false; }}>All clips</button><button on:click={() => { recentFilter = 'pinned'; filterMenu = false; }}>Pinned clips</button></div>{/if}
        <div class="clip-scroll">
          {#each clipRows as item, index}
            <button class="clip-row" class:selected={detailClip?.id === item.id} on:click={() => chooseClip(item)} on:dblclick={() => item.type === 'image' ? copyImageFn(item.content) : copyPlainText(item.content)}>
              <div class="clip-row-main">
                {#if item.thumb}{#if failedImages.has(item.thumb)}<span class="thumb-failed">Image unavailable</span>{:else}<img src={item.thumb} alt="" on:error={() => item.thumb && failImage(item.thumb)} />{/if}{/if}
                <span class:mono={item.code} class:masked-clip={item.tag === 'PASSWORD'}>{#if item.tag === 'MD'}<span class="md-heading">{item.content.split('\n')[0]}</span>{'\n' + item.content.split('\n').slice(1).join('\n')}{:else}{clipLabel(item)}{/if}</span>
                {#if item.tag}<small>{item.tag}</small>{/if}
              </div>
              <time>{item.time}</time>
            </button>
          {/each}
          {#if !clipRows.length}<p class="empty-state">{searchQuery || activeCategory !== '全部' || recentFilter === 'pinned' ? 'No matching clips' : 'No text clips yet. Copy text to get started.'}</p>{/if}
        </div>
        {#if referenceMode && !searchQuery && activeCategory === '全部'}<div class="reference-scrollbar clip-scrollbar" aria-hidden="true"></div>{/if}
      </section>

      <section class="clip-detail">
        <header class="detail-title">
          <h2>Selected clip</h2>
          <div class="icon-actions">
            <button aria-label="Pin clip" class:pinned={detailClip?.isPinned} on:click={pinClip}><IconPin size={21} strokeWidth={1.45} /></button>
            <button aria-label="More" on:click={() => (clipMenu = !clipMenu)}><IconDots size={22} strokeWidth={1.45} /></button>
          </div>
        </header>

        {#if detailClip}
          {#if clipMenu}<div class="popover clip-actions-menu" use:responsivePopover={'.clip-detail .icon-actions button:last-child'}><button on:click={pinClip}>{detailClip.isPinned ? 'Unpin clip' : 'Pin clip'}</button><button on:click={removeClip}>Delete clip</button></div>{/if}
          <h1 class="clip-heading">{clipLabel(detailClip)}</h1>
          <div class="clip-control-row">
            <label>Collection
              <button class="select-button" on:click={() => (collectionMenu = !collectionMenu)}><IconFolder size={19} strokeWidth={1.45} /><span>{referenceMode ? detailClip.category : detailClip.category === '全部' ? 'All clips' : detailClip.category || 'All clips'}</span><IconChevronDown size={16} strokeWidth={1.45} /></button>
              {#if collectionMenu}<div class="popover collection-picker" use:responsivePopover={'.select-button'}>{#each activeCollections as collection}<button on:click={() => assignCollection(collection.label)}>{collection.label}</button>{/each}</div>{/if}
            </label>
            <div class="copy-stack">
              <button class="primary-btn" on:click={copyClip}><IconCopy size={19} strokeWidth={1.45} />Copy all</button>
              <button class="outline-btn" disabled={detailClip.type === 'image'} on:mousedown|preventDefault on:click={copySelection}><IconCopy size={19} strokeWidth={1.45} />Copy selected</button>
            </div>
          </div>

          <div class="clip-content-box">
            <div class="clip-content">{#if detailClip.type === 'image'}{#if failedImages.has(getImgSrcFn(detailClip.content))}<p class="empty-state">Image unavailable. The original file may have been removed.</p>{:else}<img class="recent-image-preview" src={getImgSrcFn(detailClip.content)} alt="Selected clipboard item" on:error={() => failImage(getImgSrcFn(detailClip.content))} />{/if}{:else}<span>{referenceMode && detailClip.id === 1 ? detailClip.content.replace('feels like. ', 'feels like.\n') : detailClip.content}</span>{/if}</div>
            <footer><span>{detailClip.type === 'image' ? 'PNG image' : `Plain text · ${referenceMode ? 94 : detailClip.content.length} characters`}</span><span>Copied {referenceMode ? '10:24 AM' : displayTime(detailClip.timestamp)}</span></footer>
          </div>

          <fieldset class="quick-actions" disabled={detailClip.type === 'image'}>
            <legend>Quick actions</legend>
            <button on:click={() => copyFormatted('markdown')}><IconMarkdown size={18} stroke={1.45} />Copy as Markdown</button>
            <button on:click={() => copyFormatted('html')}><IconHtml size={18} strokeWidth={1.45} />Copy as HTML</button>
            <button on:click={() => copyFormatted('single-line')}><IconPilcrow size={18} strokeWidth={1.45} />Copy without line breaks</button>
            <button class="square-more" aria-label="More quick actions" on:click={() => (quickMenu = !quickMenu)}><IconDots size={20} strokeWidth={1.45} /></button>
            {#if quickMenu}<div class="popover quick-menu" use:responsivePopover={'.square-more'}><button on:click={() => { copyClip(); quickMenu = false; }}>Copy plain text</button></div>{/if}
          </fieldset>

          <button class="danger-btn delete-clip" on:click={removeClip}><IconTrash size={19} strokeWidth={1.45} />Delete clip</button>
        {:else}<p class="empty-state">Select a clip to view its contents.</p>{/if}
      </section>
    </main>
  {:else if activeTab === 'images'}
    <main class="ref-main images-page" inert={sidebarOpen}>
      <section class="image-library">
        <h1 data-tauri-drag-region>Copied images</h1>
        <div class="image-tools">
          <label class="ref-search image-search"><IconSearch size={20} strokeWidth={1.45} /><input bind:value={searchQuery} placeholder="Search images" /></label>
          <button class="date-filter" on:click={() => (dateMenu = !dateMenu)}><IconCalendar size={20} strokeWidth={1.45} /><span>{dateFilter}</span><IconChevronDown size={16} strokeWidth={1.45} /></button>
          {#if dateMenu}<div class="popover date-menu" use:responsivePopover={'.date-filter'}>{#each ['All time', 'Today', 'Yesterday'] as range}<button on:click={() => { dateFilter = range; dateMenu = false; }}>{range}</button>{/each}</div>{/if}
        </div>

        <div class="image-scroll">
          {#if todayImages.length || referenceMode}<h2>Today</h2>{/if}
          <div class="image-grid">
            {#each todayImages as item}
              <button class:selected={selectedImage?.id === item.id} title="Right-click to change collection" on:contextmenu|preventDefault={(event) => openImageCollection(item, event)} on:click={() => chooseImage(item)}>{#if failedImages.has(item.src)}<span class="image-failed">Image unavailable</span>{:else}<img src={item.src} alt="Clipboard item" on:error={() => failImage(item.src)} />{/if}</button>
            {/each}
          </div>
          {#if yesterdayImages.length}
            <h2 class="yesterday-title">Yesterday</h2>
            <div class="image-grid yesterday-grid">
              {#each yesterdayImages as item}
                <button class:selected={selectedImage?.id === item.id} title="Right-click to change collection" on:contextmenu|preventDefault={(event) => openImageCollection(item, event)} on:click={() => chooseImage(item)}>{#if failedImages.has(item.src)}<span class="image-failed">Image unavailable</span>{:else}<img src={item.src} alt="Clipboard item" on:error={() => failImage(item.src)} />{/if}</button>
              {/each}
            </div>
          {/if}
          {#if earlierImages.length}<h2 class="yesterday-title">Earlier</h2><div class="image-grid">{#each earlierImages as item}<button class:selected={selectedImage?.id === item.id} title="Right-click to change collection" on:contextmenu|preventDefault={(event) => openImageCollection(item, event)} on:click={() => chooseImage(item)}>{#if failedImages.has(item.src)}<span class="image-failed">Image unavailable</span>{:else}<img src={item.src} alt="Clipboard item" on:error={() => failImage(item.src)} />{/if}</button>{/each}</div>{/if}
          {#if !imageRows.length}<p class="empty-state">{searchQuery || dateFilter !== 'All time' ? 'No matching images' : 'No copied images yet.'}</p>{/if}
        </div>
      </section>

      <section class="image-detail">
        <h2>Selected image</h2>
        {#if selectedImage}
          {#if failedImages.has(selectedImage.src)}<div class="image-preview image-failed">Image unavailable. The original file may have been removed.</div>{:else}<img class="image-preview" src={referenceMode && selectedImage.id === 1 ? '/reference-assets/images/selected.png' : selectedImage.src} alt="Selected clipboard item" on:load={measureImage} on:error={() => selectedImage && failImage(selectedImage.src)} />{/if}
          <dl class="image-meta">
            <div><dt><IconClock size={19} strokeWidth={1.45} />Copied</dt><dd>{referenceMode ? 'Today, 10:24 AM' : displayTime(history.find((item) => item.id === selectedImage.id)?.timestamp)}</dd></div>
            <div><dt><IconFile size={19} strokeWidth={1.45} />Type</dt><dd>PNG image</dd></div>
            <div><dt><IconRulerMeasure size={19} strokeWidth={1.45} />Dimensions</dt><dd>{referenceMode ? '1200 × 1500' : imageDimensions}</dd></div>
            <div><dt><IconArchive size={19} strokeWidth={1.45} />Size</dt><dd>{referenceMode ? '1.2 MB' : imageSize}</dd></div>
          </dl>
          <button class="primary-btn image-action" disabled={failedImages.has(selectedImage.src)} on:click={copyCurrentImage}><IconCopy size={19} strokeWidth={1.45} />Copy image</button>
          <button class="danger-btn image-action" on:click={removeCurrentImage}><IconTrash size={19} strokeWidth={1.45} />Delete image</button>
          <div class="image-divider"></div>
          <button class="danger-btn image-action" on:click={() => { if (!referenceMode) { confirmation = 'images'; dialogError = ''; } }}><IconTrash size={19} strokeWidth={1.45} />Clear images</button>
        {:else}<p class="empty-state image-empty-detail">Select an image to preview it.</p>{/if}
      </section>
    </main>
  {:else if activeTab === 'passwords'}
    {#if !referenceMode && !vaultUnlocked}
      <main class="ref-main passwords-page vault-gate" inert={sidebarOpen}>
        <form on:submit|preventDefault={vaultExists ? unlockVaultFn : setupVaultFn}>
          <h1>{vaultExists ? 'Unlock password vault' : 'Create password vault'}</h1>
          {#if vaultExists}
            <label>Master password<input type="password" autocomplete="current-password" bind:value={unlockPassword} required /></label>
          {:else}
            <label>Master password<input type="password" autocomplete="new-password" placeholder="8–16 characters" title="8–16 characters" bind:value={masterPassword} required /></label>
            <label>Confirm master password<input type="password" autocomplete="new-password" placeholder="8–16 characters" title="8–16 characters" bind:value={masterPasswordConfirm} required /></label>
          {/if}
          {#if vaultError}<p class="interaction-error" role="alert">{vaultError}</p>{/if}
          <button class="primary-btn" disabled={vaultBusy}>{vaultBusy ? 'Please wait…' : vaultExists ? 'Unlock vault' : 'Create vault'}</button>
        </form>
      </main>
    {:else}
    <main class="ref-main passwords-page" inert={sidebarOpen}>
      <section class="password-list">
        <header class="password-list-head"><h2>Password vault</h2><button class="primary-btn add-password" disabled={vaultSaving || vaultBusy} on:click={beginPassword}>Add password</button><button class="password-filter-button" aria-label="Filter passwords" data-menu-trigger on:click={() => (passwordFilterMenu = !passwordFilterMenu)}><IconFilter2 size={19} strokeWidth={1.45} /></button></header>
        {#if passwordFilterMenu}<div class="popover password-filter-menu" use:responsivePopover={'.password-filter-button'}>
          {#each [{value:'all', label:'All items'}, {value:'with-username', label:'With username'}, {value:'without-username', label:'Without username'}] as option}<button aria-pressed={passwordFilter === option.value} on:click={() => { passwordFilter = option.value; passwordFilterMenu = false; }}>{option.label}</button>{/each}
          <hr />
          {#each [{value:'newest', label:'Newest first'}, {value:'oldest', label:'Oldest first'}, {value:'title', label:'Title A–Z'}] as option}<button aria-pressed={passwordSort === option.value} on:click={() => { passwordSort = option.value; passwordFilterMenu = false; }}>{option.label}</button>{/each}
        </div>{/if}
        <div class="password-scroll">
          {#each passwordRows as item}
            <div class="password-row" class:selected={!addingPassword && selectedPassword?.id === item.id}>
              <button class="password-select" aria-label={`Select ${item.title}`} on:click={() => { loadedPasswordKey = ''; selectedPasswordId = item.id; addingPassword = false; revealPassword = false; }}>
              <div><strong>{item.title}</strong><span>{item.username}</span><span class="masked">•••••••••••••</span></div>
              <time>{item.time}</time>
              </button>
              <button class="row-more" aria-label={`Actions for ${item.title}`} data-menu-trigger disabled={vaultSaving || vaultBusy} on:click={(event) => openPasswordMenu(item.id, event)}><IconDots size={20} strokeWidth={1.45} /></button>
            </div>
          {/each}
          {#if !passwordRows.length}<p class="empty-state">{pwdSearchQuery ? 'No matching passwords' : 'No saved passwords yet.'}</p>{/if}
        </div>
        {#if referenceMode && !pwdSearchQuery}<div class="reference-scrollbar password-scrollbar" aria-hidden="true"></div>{/if}
      </section>

      <section class="password-detail">
        <header class="detail-title">
          <h2>Item details</h2>
          <div class="icon-actions"><button aria-label="Edit" on:click={() => titleInput?.focus()}><IconPencil size={21} strokeWidth={1.45} /></button><button aria-label="More" disabled={!selectedPassword || addingPassword || vaultSaving || vaultBusy} data-menu-trigger on:click={() => { passwordMenu = !passwordMenu; choosingPasswordCollection = false; rowMenuId = null; }}><IconDots size={22} strokeWidth={1.45} /></button></div>
        </header>
        {#if passwordMenu && selectedPassword}<div class="popover password-menu" use:responsivePopover={'.password-detail .icon-actions button:last-child'}>
          {#if choosingPasswordCollection}<button on:click={() => assignPasswordCollection(selectedPassword.id, null)}>No collection</button>{#each vaultCollections as collection}<button on:click={() => assignPasswordCollection(selectedPassword.id, collection.id)}>{collection.name}</button>{/each}
          {:else}<button on:click={() => { titleInput?.focus(); passwordMenu = false; }}>Edit password</button><button on:click={() => { copyPlainText(selectedPassword.username); passwordMenu = false; }}>Copy username</button><button on:click={() => { copyPlainText(selectedPassword.password); passwordMenu = false; }}>Copy password</button><button on:click={() => (choosingPasswordCollection = true)}>Move to collection</button><button on:click={() => removePassword()}>Delete password</button>{/if}
        </div>{/if}
        {#if selectedPassword || addingPassword}
          <form id="password-details" class="password-form" on:submit|preventDefault={savePassword}>
            <label>Title<input bind:this={titleInput} bind:value={draftTitle} required /></label>
            <label>Username<div class="field-with-action"><input aria-label="Username" bind:value={draftUsername} /><button type="button" on:click={() => copyPlainText(draftUsername)}><IconCopy size={19} strokeWidth={1.45} />Copy username</button></div></label>
            <label>Password<div class="field-with-action password-field"><input aria-label="Password" type={revealPassword ? 'text' : 'password'} bind:value={draftPassword} required /><button class="eye-button" type="button" aria-label={revealPassword ? 'Hide password' : 'Show password'} on:click={() => (revealPassword = !revealPassword)}><IconEye size={20} strokeWidth={1.45} /></button><button type="button" on:click={() => copyPlainText(draftPassword)}><IconCopy size={19} strokeWidth={1.45} />Copy password</button></div></label>
          </form>
          {#if vaultError && !referenceMode}<p class="interaction-error" role="alert">{vaultError}</p>{/if}
        {/if}
        <footer class="password-footer"><button class="outline-btn lock-detail" title={vaultRequirePassword ? 'Lock vault' : 'Enable password requirement in Settings to lock the vault'} disabled={vaultBusy || (!referenceMode && !vaultRequirePassword)} on:click={() => !referenceMode && lockVaultFn()}><IconLock size={21} strokeWidth={1.45} />Lock vault</button><button class="primary-btn save-changes" disabled={savingPassword || vaultSaving || vaultBusy || (!selectedPassword && !addingPassword)} type="submit" form="password-details">{savingPassword ? 'Saving…' : 'Save changes'}</button></footer>
      </section>
    </main>
    {/if}
  {:else}
    <main class="ref-main settings-page" inert={sidebarOpen}>
      <section class="settings-content">
        <h1>General settings</h1>
        <div class="settings-row">
          <div><h2>Clipboard capture</h2><p>Automatically save everything you copy.</p></div>
          <div class="capture-control"><button aria-label="Toggle clipboard capture" aria-pressed={captureEnabled} disabled={!referenceMode && (!storeReady || settingsBusy || clearDataBusy || resetPending)} class:active={captureEnabled} class="toggle" on:click={() => referenceMode ? captureEnabled = !captureEnabled : toggleCaptureFn()}><span></span></button><span>{captureEnabled ? 'On' : 'Off'}</span></div>
        </div>
        <div class="settings-row">
          <div><h2>Global shortcut</h2><p>Press this shortcut to open Clipboard anywhere.</p></div>
          <button class="shortcut-control" disabled={shortcutBusy || clearDataBusy || resetPending} on:click={toggleShortcutEditor}>{#each shortcutKeys as key, index}{#if index}<span>+</span>{/if}<kbd>{key}</kbd>{/each}<IconChevronDown size={16} strokeWidth={1.45} /></button>
        </div>
        {#if editingShortcut}<form class="popover shortcut-editor" use:responsivePopover={'.shortcut-control'} on:submit|preventDefault={applyShortcut}><label>Global shortcut<input bind:this={shortcutInput} value={shortcutDraft} readonly placeholder="Press a shortcut…" on:keydown={recordShortcut} disabled={shortcutBusy || clearDataBusy || resetPending} title="Press the keys together to record a shortcut. Esc cancels; Tab moves to Apply." /></label><button class="outline-btn" disabled={!shortcutHasKey || shortcutBusy || clearDataBusy || resetPending}>Apply</button>{#if shortcutError}<p role="alert">{shortcutError}</p>{/if}</form>{/if}
        {#if !referenceMode}<div class="settings-row vault-access-setting">
          <div><h2>Password vault access</h2><p>{!vaultStatusReady ? 'Checking vault availability…' : !vaultExists ? 'Create a password vault first to change this option.' : !vaultAutoUnlockAvailable ? 'Password-free access is only available on Windows.' : vaultRequirePassword ? 'Require a master password to open your password vault.' : 'Open without a password on this Windows account.'}</p></div>
          <div class="capture-control"><button class="toggle" class:active={vaultRequirePassword} aria-label="Require password to open password vault" aria-pressed={vaultRequirePassword} disabled={!vaultStatusReady || !vaultExists || !vaultAutoUnlockAvailable || vaultBusy || vaultSaving || clearDataBusy || resetPending} on:click={openVaultAccessDialog}><span></span></button><span>{vaultRequirePassword ? 'On' : 'Off'}</span></div>
        </div>{/if}
        <div class="settings-row">
          <div><h2>Password transfer</h2><p>Import and export your saved passwords.</p></div>
          <div class="transfer-actions"><button class="outline-btn" disabled={clearDataBusy || resetPending || vaultSaving} on:click={requestPasswordImportFn}><IconUpload size={21} strokeWidth={1.45} />Import</button><button class="outline-btn" disabled={clearDataBusy || resetPending || vaultSaving} on:click={exportPasswordsFn}><IconDownload size={21} strokeWidth={1.45} />Export</button></div>
        </div>
        <div class="settings-row last-setting">
          <div><h2>Official website</h2><p>Visit our website to learn more and get help.</p></div>
          <button class="outline-btn website-button" on:click={openOfficialWebsiteFn}><IconExternalLink size={21} strokeWidth={1.45} />Open website</button>
        </div>

        <section class="danger-zone">
          <h2>Danger zone</h2>
          <div class="danger-content"><div><h3>Clear all local app data</h3><p>{referenceMode ? 'This permanently deletes all clips, collections, and settings from this device.' : 'Deletes clips, passwords, collections and app cache; resets settings and startup.'}</p></div><div class="danger-controls"><input bind:value={clearDataConfirmation} placeholder="Type DELETE to confirm" /><button disabled={clearDataConfirmation !== 'DELETE' || clearDataBusy} on:click={() => !referenceMode && clearDataFn()}>{clearDataBusy ? 'Deleting…' : 'Delete'}</button></div></div>
          {#if clearDataError}<p class="interaction-error" role="alert">{clearDataError}</p>{/if}
        </section>
        {#if !referenceMode}<details class="native-settings"><summary>Desktop & storage</summary><div class="native-setting"><span>Launch at startup</span><button aria-label="Toggle launch at startup" class="outline-btn" disabled={settingsBusy} on:click={toggleAutostartFn}>{autostartEnabled ? 'On' : 'Off'}</button></div>{#if autostartError}<p class="interaction-error">{autostartError}</p>{/if}<form class="native-setting" on:submit|preventDefault={saveRetentionFn}><label>Unpinned text retention (hours)<input type="number" min="1" max="8760" step="1" bind:value={retentionHours} /></label><button class="outline-btn" disabled={settingsBusy || !storeReady}>Save retention</button></form><p>Pinned and collected text is retained. Uncollected, unpinned text: 50 items. Images: 10 items / 80 MiB. Closing the window hides it to the tray.</p></details>{/if}
      </section>
    </main>
  {/if}
  {#if imageCollectionId !== null}<div class="popover image-collection-menu" aria-label="Image collection" style:top={`${imageMenuTop}px`} style:left={`${imageMenuLeft}px`}>
    {#each activeCollections as collection}<button aria-pressed={(history.find(item => item.id === imageCollectionId)?.category || '全部') === (collection.id || '全部')} on:click={() => assignImageCollection(collection.id)}>{collection.label}</button>{/each}
  </div>{/if}
  {#if menuPassword}<div class="popover password-row-menu" style:top={`${rowMenuTop}px`} style:left={`${rowMenuLeft}px`}>
    {#if choosingPasswordCollection}<button disabled={vaultSaving} on:click={() => assignPasswordCollection(menuPassword.id, null)}>No collection</button>{#each vaultCollections as collection}<button disabled={vaultSaving} on:click={() => assignPasswordCollection(menuPassword.id, collection.id)}>{collection.name}</button>{/each}
    {:else}<button on:click={() => { loadedPasswordKey = ''; selectedPasswordId = menuPassword.id; addingPassword = false; revealPassword = false; rowMenuId = null; queueMicrotask(() => titleInput?.focus()); }}>Edit password</button><button on:click={() => { copyPlainText(menuPassword.username); rowMenuId = null; }}>Copy username</button><button on:click={() => { copyPlainText(menuPassword.password); rowMenuId = null; }}>Copy password</button><button on:click={() => (choosingPasswordCollection = true)}>Move to collection</button><button on:click={() => removePassword(menuPassword.id)}>Delete password</button>{/if}
  </div>{/if}
  {#if addingCollection}<div class="dialog-backdrop"><form class="collection-dialog" on:submit|preventDefault={addCollection}><h2>{editingCollection ? 'Rename collection' : 'New collection'}</h2><input aria-label="Collection name" bind:value={collectionName} required />{#if dialogError}<p role="alert" class="interaction-error">{dialogError}</p>{/if}<div><button type="button" class="outline-btn" disabled={dialogBusy} on:click={() => { addingCollection = false; editingCollection = ''; }}>Cancel</button><button class="primary-btn" disabled={dialogBusy}>{dialogBusy ? 'Saving…' : editingCollection ? 'Save collection' : 'Create collection'}</button></div></form></div>{/if}
  {#if vaultAccessDialog}<div class="dialog-backdrop" role="dialog" aria-modal="true" aria-label="Password vault access"><form class="collection-dialog vault-access-dialog" on:submit|preventDefault={saveVaultAccess}>
    <h2>{vaultAccessTarget ? 'Require a password?' : 'Open without a password?'}</h2>
    <p>{vaultAccessTarget ? 'Confirm your current master password. The vault will lock immediately and ask for this password when opened.' : 'Anyone using your signed-in Windows account will be able to open this vault. Data stays encrypted. Keep your master password for backups and recovery.'}</p>
    <input bind:this={vaultAccessInput} type="password" autocomplete="current-password" aria-label="Current master password" placeholder="Current master password" bind:value={vaultAccessPassword} disabled={vaultBusy} required />
    {#if vaultAccessError}<p class="interaction-error" role="alert">{vaultAccessError}</p>{/if}
    <div><button type="button" class="outline-btn" disabled={vaultBusy} on:click={closeVaultAccessDialog}>Cancel</button><button class="primary-btn" disabled={vaultBusy || !vaultAccessPassword}>{vaultBusy ? 'Saving…' : 'Confirm'}</button></div>
  </form></div>{/if}
  {#if confirmation}<div class="dialog-backdrop"><div class="collection-dialog" role="dialog" aria-modal="true" aria-label={confirmation === 'images' ? 'Clear all images' : 'Delete collection'}><h2>{confirmation === 'images' ? 'Clear all images?' : 'Delete collection?'}</h2><p>{confirmation === 'images' ? 'This deletes all clipboard images, including images outside the current collection, search and date filter.' : 'Passwords will be kept in All items. Only this collection and its password associations will be removed.'}</p>{#if dialogError}<p class="interaction-error" role="alert">{dialogError}</p>{/if}<div><button class="outline-btn" disabled={dialogBusy} on:click={() => (confirmation = '')}>Cancel</button><button class="primary-btn" disabled={dialogBusy} on:click={confirmAction}>{dialogBusy ? 'Deleting…' : 'Delete'}</button></div></div></div>{/if}
  {#if showToast && !referenceMode && !vaultAccessDialog}<div class="status-toast" role="status">{toastMsg}</div>{/if}
  {#if !referenceMode && (storeError || captureError)}<div class="status-toast error-toast" role="alert">{storeError || captureError}</div>{/if}
</div>

<style>
  :global(html), :global(body) { width: 100%; height: 100%; margin: 0; overflow: hidden; }
  :global(*) { box-sizing: border-box; }
  :global(button), :global(input) { font: inherit; }
  :global(body) { font-weight: 400; }

  .ref-window {
    --serif: "Times New Roman", Georgia, serif;
    --window-bg: #faf9f7;
    --sidebar-bg: #f7f4f1;
    --surface: #fdfcfb;
    --active-bg: #f3e8e1;
    --selected-bg: #f6ebe3;
    --ink: #292824;
    --soft: #4f4b46;
    --muted: #77726b;
    --line: #e7e2dd;
    --line-strong: #d9d2cb;
    --accent: #c25437;
    --button-fill: #ba5d41;
    --danger: #c45538;
    position: relative;
    width: 100vw;
    height: 100vh;
    min-width: 0;
    overflow: hidden;
    border: 1px solid #cfc8c1;
    border-radius: 11px;
    background: var(--window-bg);
    color: var(--ink);
    font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", Arial, sans-serif;
  }

  button { color: inherit; border: 0; background: transparent; cursor: default; }
  h1, h2, h3, p { margin: 0; }

  .ref-sidebar {
    position: absolute;
    inset: 0 auto 0 0;
    width: 306px;
    background: var(--sidebar-bg);
    border-right: 1px solid var(--line);
  }

  .ref-brand {
    position: absolute;
    left: 31px;
    top: 44px;
    font-family: var(--serif);
    font-size: 43.5px;
    line-height: 46px;
    letter-spacing: -1.45px;
  }

  .ref-primary-nav { position: absolute; left: 14px; top: 129px; width: 278px; display: grid; gap: 9px; }
  .ref-primary-nav button {
    width: 278px;
    height: 46px;
    display: flex;
    align-items: center;
    gap: 17px;
    padding: 0 19px;
    border-radius: 8px;
    font-family: "Segoe UI", Arial, sans-serif;
    font-size: 18px;
    text-align: left;
  }
  .ref-primary-nav button :global(svg) { color: #706c66; }
  .ref-primary-nav button.active { color: var(--accent); background: var(--active-bg); }
  .ref-primary-nav button.active :global(svg) { color: var(--accent); }
  .ref-primary-nav button:first-child.active :global(svg) { color: #806e60; }

  .ref-collections-head {
    position: absolute;
    left: 30px;
    top: 378px;
    width: 237px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-family: var(--serif);
    font-size: 17px;
  }
  .ref-collections-head button { width: 25px; height: 25px; padding: 3px; color: #67635e; }
  .ref-collection-list { position: absolute; left: 14px; top: 421px; width: 278px; display: grid; }
  .ref-collection-list button {
    width: 278px;
    height: 51px;
    display: grid;
    grid-template-columns: 23px 1fr auto;
    align-items: center;
    column-gap: 14px;
    padding: 0 25px 0 19px;
    border-radius: 8px;
    font-family: var(--serif);
    font-size: 17px;
    text-align: left;
  }
  .ref-collection-list button :global(svg) { color: #706c66; }
  .ref-collection-list button small { color: #66615c; font-size: 14px; font-family: "Segoe UI", Arial, sans-serif; }
  .ref-collection-list button.active { position: relative; isolation: isolate; color: var(--accent); }
  .ref-collection-list button.active::before { content: ''; position: absolute; inset: 0 0 4px; z-index: -1; border-radius: 8px; background: var(--active-bg); }
  .ref-collection-list button > span { transform: translateY(-2px); }
  .ref-collection-list .new-collection { color: var(--accent); grid-template-columns: 23px 1fr; }
  .ref-collection-list .new-collection :global(svg) { color: var(--accent); }

  .sidebar-lock {
    position: absolute;
    left: 29px;
    bottom: 109px;
    width: 248px;
    height: 54px;
    display: flex;
    align-items: center;
    gap: 17px;
    padding: 0 18px;
    border: 1px solid var(--line-strong);
    border-radius: 7px;
    color: var(--danger);
    font-family: var(--serif);
    font-size: 17px;
  }
  .ref-sidebar-footer {
    position: absolute;
    left: 30px;
    right: 27px;
    bottom: 15px;
    height: 40px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-family: var(--serif);
    font-size: 15px;
  }

  .window-controls {
    position: absolute;
    z-index: 20;
    right: 1px;
    top: 1px;
    width: 174px;
    height: 45px;
    display: flex;
    justify-content: flex-end;
  }
  .window-controls button { width: 58px; height: 44px; display: grid; place-items: center; padding: 0; }

  .ref-topbar {
    position: absolute;
    left: 306px;
    right: 0;
    top: 0;
    height: 92px;
    border-bottom: 1px solid var(--line);
  }
  .ref-search {
    display: flex;
    align-items: center;
    border: 1px solid var(--line-strong);
    border-radius: 28px;
    background: rgba(253, 252, 251, .72);
    color: #6d6964;
  }
  .ref-topbar .ref-search { position: absolute; left: 32px; top: 21px; width: 430px; height: 53px; padding: 0 23px; gap: 15px; }
  .ref-search input { min-width: 0; flex: 1; border: 0; outline: 0; background: transparent; color: var(--ink); font-size: 17px; }
  .ref-topbar .ref-search { gap: 11px; }
  .ref-search input::placeholder { color: #797570; opacity: 1; }

  .ref-main { position: absolute; left: 306px; right: 0; bottom: 0; background: var(--window-bg); }
  .recent-page, .passwords-page, .settings-page { top: 92px; }
  .images-page { top: 0; }
  .column-title, .detail-title { display: flex; align-items: center; justify-content: space-between; }
  .column-title h2, .detail-title h2, .image-detail > h2, .password-list-head h2 {
    font-family: var(--serif);
    font-size: 21px;
    font-weight: 400;
    letter-spacing: -.18px;
  }

  .clip-column { position: absolute; left: 0; top: 0; bottom: 0; width: 477px; border-right: 1px solid var(--line); }
  .column-title { height: 67px; padding: 0 38px 0 34px; }
  .column-title h2 { transform: translateY(-2px); }
  .clip-scroll { position: absolute; left: 15px; right: 7px; top: 66px; bottom: 0; overflow-y: auto; overflow-x: hidden; padding-right: 13px; }
  .clip-scroll::-webkit-scrollbar, .password-scroll::-webkit-scrollbar, .image-scroll::-webkit-scrollbar { width: 7px; }
  .clip-scroll::-webkit-scrollbar-thumb, .password-scroll::-webkit-scrollbar-thumb, .image-scroll::-webkit-scrollbar-thumb { border-radius: 5px; background: #c9c3bd; }
  .reference-mode .clip-scroll, .reference-mode .password-scroll { scrollbar-width: none; }
  .reference-mode .clip-scroll::-webkit-scrollbar, .reference-mode .password-scroll::-webkit-scrollbar { display: none; }
  .reference-scrollbar { position: absolute; width: 7px; border-radius: 5px; background: #c9c3bd; }
  .clip-scrollbar { right: 9px; top: 68px; height: 169px; }
  .password-scrollbar { right: 7px; top: 77px; height: 100px; }
  .clip-row {
    color: var(--soft);
    position: relative;
    width: 100%;
    min-height: 78px;
    display: grid;
    grid-template-columns: 1fr 84px;
    padding: 15px 14px 11px 18px;
    text-align: left;
    font-size: 17px;
    line-height: 26px;
  }
  .clip-row.selected { min-height: 82px; background: var(--selected-bg); border-radius: 4px; }
  .clip-row:not(.selected)::after { content: ""; position: absolute; left: 8px; right: 7px; bottom: 0; border-bottom: 1px solid var(--line); }
  .reference-mode .clip-row { height: 77px; min-height: 0; }
  .reference-mode .clip-row:nth-child(1) { height: 82px; }
  .reference-mode .clip-row:nth-child(2) { height: 76px; }
  .reference-mode .clip-row:nth-child(6) { height: 89px; }
  .reference-mode .clip-row:nth-child(7) { height: 73px; line-height: 22px; }
  .reference-mode .clip-row:nth-child(7) .clip-row-main > span { max-width: 296px; }
  .reference-mode .clip-row:nth-child(8) { height: 103px; }
  .reference-mode .clip-row:nth-child(8) .mono { -webkit-line-clamp: 3; line-clamp: 3; }
  .reference-mode .clip-row:nth-child(10) { height: 84px; }
  .reference-mode .clip-row:has(img) { font-size: 16px; font-family: "Segoe UI", Arial, sans-serif; }
  .clip-row.selected::before, .password-row.selected::before { content: ""; position: absolute; left: 0; top: 0; bottom: 0; width: 2px; background: var(--accent); }
  .clip-row-main { min-width: 0; display: flex; flex-wrap: wrap; align-content: flex-start; column-gap: 9px; }
  .clip-row-main > span { display: -webkit-box; overflow: hidden; line-clamp: 2; -webkit-line-clamp: 2; -webkit-box-orient: vertical; white-space: pre-line; }
  .clip-row-main > img { width: 56px; height: 51px; border-radius: 4px; object-fit: cover; }
  .clip-row-main > img + span { flex: 1; min-width: 0; padding-top: 3px; display: block; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .clip-row-main small { position: absolute; left: 16px; bottom: 10px; width: fit-content; height: 20px; padding: 0 6px; border: 1px solid var(--line-strong); border-radius: 4px; color: #6d6862; font-size: 11px; line-height: 18px; }
  .clip-row-main:has(img) small { left: 83px; }
  .clip-row-main .mono { font-family: Consolas, monospace; font-size: 14px; line-height: 18px; }
  .clip-row-main .md-heading { font-family: "Segoe UI", Arial, sans-serif; font-size: 16px; }
  .clip-row-main .masked-clip { letter-spacing: 1px; color: #5f5b56; }
  .clip-row time { align-self: start; color: var(--muted); font-family: "Segoe UI", Arial, sans-serif; font-size: 14px; text-align: right; white-space: nowrap; }

  .clip-detail { position: absolute; left: 477px; right: 0; top: 0; bottom: 0; padding: 0 31px; }
  .detail-title { height: 67px; }
  .icon-actions { display: flex; gap: 8px; }
  .icon-actions button { width: 43px; height: 43px; display: grid; place-items: center; border: 1px solid var(--line-strong); border-radius: 7px; }
  .icon-actions button[aria-label='Pin clip'] :global(svg) { transform: rotate(40deg); }
  .icon-actions button.pinned { color: var(--accent); background: var(--active-bg); }
  .clip-heading {
    width: 510px;
    max-height: 81px;
    overflow: hidden;
    position: relative;
    left: 2px;
    margin-top: 15px;
    font-family: var(--serif);
    font-size: 32.5px;
    font-weight: 400;
    line-height: 40px;
    letter-spacing: -.6px;
    transform: scaleX(.96);
    transform-origin: left center;
  }
  .clip-control-row { height: 144.333px; display: flex; justify-content: space-between; align-items: end; padding-bottom: 18px; }
  .clip-control-row > label { display: grid; gap: 8px; font-family: var(--serif); font-size: 17px; }
  .select-button { width: 244px; height: 46px; display: grid; grid-template-columns: 20px 1fr 16px; align-items: center; gap: 9px; padding: 0 16px; border: 1px solid var(--line-strong); border-radius: 7px; text-align: left; font-size: 17px; }
  .copy-stack { display: grid; gap: 7px; width: 177px; }
  .clip-control-row > label { transform: translateY(-3px); }
  .primary-btn, .outline-btn, .danger-btn { display: flex; align-items: center; justify-content: center; gap: 11px; border-radius: 7px; font-family: var(--serif); font-size: 18px; }
  .primary-btn { color: #fff; background: var(--button-fill); border: 1px solid #bf5035; }
  .images-page { --button-fill: #c05a39; }
  .passwords-page { --button-fill: #b54d36; --selected-bg: #f5eae6; }
  .outline-btn, .danger-btn { height: 43px; border: 1px solid var(--line-strong); background: rgba(253, 252, 251, .55); }
  .danger-btn { color: var(--danger); }
  .copy-stack button { height: 43px; }
  .copy-stack .primary-btn { height: 44px; }
  .clip-content-box { height: 367px; border: 1px solid var(--line-strong); border-radius: 8px; overflow: hidden; font-family: var(--serif); }
  .clip-content { height: 318px; padding: 17px; white-space: pre-line; font-size: 22px; line-height: 28px; }
  .clip-content { overflow: auto; }
  .clip-content > span { display: block; transform: scaleX(.93); transform-origin: left top; }
  .clip-content-box footer { height: 49px; padding: 0 17px; display: flex; align-items: center; justify-content: space-between; border-top: 1px solid var(--line-strong); color: var(--muted); font-size: 15px; }
  .quick-actions { height: 84px; margin: 35px 0 0; padding: 17px 21px 16px; display: flex; align-items: center; gap: 10px; border: 1px solid var(--line-strong); border-radius: 8px; }
  .quick-actions legend { padding: 0 8px; color: var(--soft); font-family: var(--serif); font-size: 14px; }
  .quick-actions button { height: 40px; padding: 0 16px; display: flex; align-items: center; gap: 10px; border: 1px solid var(--line-strong); border-radius: 6px; background: var(--surface); font-family: var(--serif); font-size: 14px; white-space: nowrap; }
  .quick-actions .square-more { width: 42px; padding: 0; justify-content: center; }
  .quick-actions button:nth-of-type(1) { width: 190px; }
  .quick-actions button:nth-of-type(2) { width: 156px; }
  .quick-actions button:nth-of-type(3) { width: 209px; }
  .quick-actions :global(.tabler-icon-markdown path:first-child) { display: none; }
  .quick-actions :global(.tabler-icon-markdown) { transform: scale(1.5); }
  .delete-clip { position: absolute; right: 31px; bottom: 40px; width: 149px; }

  .image-library { position: absolute; left: 0; top: 0; bottom: 0; width: 795px; border-right: 1px solid var(--line); }
  .image-library > h1 { position: absolute; left: 35px; top: 41px; transform: scaleX(.92); transform-origin: left center; font-family: var(--serif); font-size: 30.25px; font-weight: 400; letter-spacing: -.45px; }
  .image-tools { position: absolute; left: 35px; top: 97px; display: flex; gap: 10px; }
  .image-search { width: 530px; height: 51px; padding: 0 23px; gap: 12px; }
  .date-filter { width: 191px; height: 51px; padding: 0 15px; display: grid; grid-template-columns: 21px 1fr 16px; align-items: center; gap: 11px; border: 1px solid var(--line-strong); border-radius: 7px; text-align: left; font-family: var(--serif); font-size: 16px; }
  .image-scroll { position: absolute; inset: 174.333px 14px 0 28.333px; padding: 0 8px 40px 4px; overflow-y: auto; }
  .image-scroll h2 { height: 43px; padding-top: 2px; font-family: var(--serif); color: var(--muted); font-size: 18px; font-weight: 400; }
  .image-grid { display: grid; grid-template-columns: repeat(4, 174px); grid-auto-rows: 215px; gap: 13px 13px; }
  .image-grid button { position: relative; width: 174px; height: 215px; padding: 0; border-radius: 9px; }
  .image-grid img { width: 100%; height: 100%; display: block; border-radius: 8px; object-fit: cover; }
  .image-grid button.selected::before { content: ""; position: absolute; inset: -4px; border: 2px solid var(--accent); border-radius: 11px; pointer-events: none; }
  .image-grid button.selected::after { content: ""; position: absolute; inset: -2px; border: 2px solid var(--window-bg); border-radius: 10px; pointer-events: none; }
  .yesterday-title { height: 36px !important; margin-top: 41px; }
  .yesterday-grid { grid-auto-rows: 213px; }
  .yesterday-grid button { height: 213px; }
  .image-detail { position: absolute; left: 795px; right: 0; top: 0; bottom: 0; padding: 0 26px; }
  .image-detail > h2 { position: absolute; left: 26px; top: 42px; }
  .image-preview { position: absolute; left: 25.333px; top: 97.333px; width: 427px; height: 480px; border-radius: 8px; object-fit: cover; }
  .image-meta { position: absolute; left: 26px; right: 30px; top: 599px; margin: 0; }
  .image-meta > div { height: 35px; display: flex; align-items: center; justify-content: space-between; }
  .image-meta dt { display: flex; align-items: center; gap: 17px; color: var(--muted); font-family: var(--serif); font-size: 18px; }
  .image-meta dd { margin: 0; color: var(--muted); font-family: "Segoe UI", Arial, sans-serif; font-size: 16px; }
  .image-action { position: absolute; left: 26px; width: 427px; height: 43px; }
  .image-detail .primary-btn { top: 757px; }
  .image-detail .danger-btn:nth-of-type(2) { top: 809px; }
  .image-divider { position: absolute; left: 26px; width: 427px; top: 877px; border-top: 1px solid var(--line); }
  .image-detail .danger-btn:nth-of-type(3) { top: 904px; }

  .password-list { position: absolute; left: 0; top: 0; bottom: 0; width: 501px; border-right: 1px solid var(--line); }
  .password-list-head { height: 76px; display: grid; grid-template-columns: 1fr 134px 26px; align-items: center; gap: 13px; padding: 0 25px 0 32px; }
  .add-password { width: 134px; height: 39px; }
  .password-scroll { position: absolute; left: 15px; right: 8px; top: 75px; bottom: 0; overflow-y: auto; overflow-x: hidden; padding-right: 6px; }
  .password-row { position: relative; width: 462px; height: 103px; padding: 15px 64px 10px 18px; display: block; text-align: left; }
  .password-row.selected { background: var(--selected-bg); border-radius: 4px; }
  .password-row:not(.selected)::after { content: ""; position: absolute; left: 8px; right: 7px; bottom: 0; border-bottom: 1px solid var(--line); }
  .password-select { position: absolute; inset: 0; width: 100%; height: 100%; text-align: left; }
  .password-select > div { position: absolute; left: 18px; top: 17px; display: grid; }
  .password-row strong { transform: translateY(-2px); font-family: "Segoe UI", Arial, sans-serif; font-size: 18px; font-weight: 400; line-height: 24px; }
  .password-row span, .password-row time { color: #69645f; font-size: 14px; line-height: 20px; }
  .password-select > div > span:not(.masked) { transform: translateY(-3px); font-size: 14.5px; }
  .password-row .masked { transform: translate(1px, 5px); letter-spacing: 3px; font-size: 16px; }
  .password-row time { position: absolute; right: 62px; bottom: 17px; font-family: "Segoe UI", Arial, sans-serif; }
  .password-row .row-more { position: absolute; right: 12px; top: 31px; width: 38px; height: 38px; display: grid; place-items: center; border: 1px solid var(--line-strong); border-radius: 7px; color: var(--ink); }
  .password-detail { position: absolute; left: 501px; right: 0; top: 0; bottom: 0; padding: 0 35px 0 30.333px; }
  .password-detail .detail-title h2 { transform: translate(-1px, 7px) scaleX(.95); transform-origin: left center; font-size: 20px; }
  .password-detail .detail-title .icon-actions { transform: translateY(6px); }
  .password-form { display: grid; gap: 28px; margin-top: 22px; }
  .password-form > label { display: grid; gap: 10px; font-family: var(--serif); font-size: 17px; }
  .password-form input { width: 100%; height: 53px; padding: 0 16px; border: 1px solid var(--line-strong); border-radius: 7px; outline: 0; background: rgba(253,252,251,.55); color: var(--ink); font-family: "Segoe UI", Arial, sans-serif; font-size: 20px; }
  .password-form > label:first-child input { height: 52px; }
  .password-form > label:nth-child(2) input { height: 55px; }
  .password-field input[type=password] { letter-spacing: 2.2px; }
  .field-with-action { position: relative; }
  .field-with-action input { padding-right: 210px; }
  .field-with-action button { position: absolute; right: 8px; top: 8px; width: 190px; height: 39px; padding: 0; display: flex; align-items: center; justify-content: center; gap: 13px; border: 1px solid var(--line-strong); border-radius: 6px; background: var(--surface); font-family: var(--serif); font-size: 17px; }
  .password-field input { padding-right: 245px; }
  .password-field .eye-button { right: 211px; top: 12px; width: 30px; height: 30px; padding: 5px; border: 0; background: transparent; color: #6c6862; }
  .password-footer { position: absolute; left: 0; right: 0; bottom: 0; height: 110px; padding: 21px 35px 0 30px; display: flex; align-items: flex-start; justify-content: space-between; border-top: 1px solid var(--line); }
  .lock-detail { width: 176px; height: 54px; color: var(--danger); }
  .save-changes { width: 191px; height: 56px; font-size: 19px; }

  .settings-content { position: absolute; left: 31px; top: 34px; width: 1111px; }
  .settings-content > h1 { position: relative; left: 1px; top: 2px; transform: scaleX(.9); transform-origin: left center; font-family: var(--serif); font-size: 44px; font-weight: 400; line-height: 48px; letter-spacing: -1px; }
  .settings-row { width: 1179px; padding-right: 68px; height: 110px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid var(--line); }
  .settings-row:first-of-type { height: 119px; margin-top: 14px; }
  .settings-row:first-of-type > * { position: relative; top: 3px; }
  .settings-row h2 { font-family: var(--serif); font-size: 22px; font-weight: 400; line-height: 29px; transform: scaleX(.965); transform-origin: left center; }
  .settings-row p { margin-top: 3px; color: var(--muted); font-size: 16px; line-height: 22px; }
  .capture-control { display: flex; align-items: center; gap: 14px; margin-right: 1px; font-family: var(--serif); font-size: 16px; }
  .toggle { width: 51px; height: 28px; padding: 3px; border-radius: 15px; background: #bbb5af; }
  .toggle span { display: block; width: 22px; height: 22px; border-radius: 50%; background: #fff; }
  .toggle.active { background: var(--accent); }
  .toggle.active span { margin-left: 23px; }
  .shortcut-control { width: 293px; height: 50px; padding: 0 15px; display: flex; align-items: center; justify-content: space-between; border: 1px solid var(--line-strong); border-radius: 7px; font-size: 17px; }
  .shortcut-control kbd { min-width: 32px; height: 35px; padding: 0 10px; display: grid; place-items: center; border: 1px solid var(--line-strong); border-radius: 6px; background: var(--surface); font-family: "Segoe UI", Arial, sans-serif; font-size: 16px; font-weight: 400; }
  .shortcut-control kbd:first-of-type { min-width: 46px; }
  .shortcut-control kbd:nth-of-type(2) { min-width: 53px; }
  .transfer-actions { display: flex; gap: 21px; }
  .transfer-actions button { width: 133px; height: 49px; font-size: 19px; }
  .website-button { width: 185px; height: 49px; font-size: 19px; }
  .last-setting { border-bottom: 0; }
  .danger-zone { height: 165px; margin-top: 28px; padding: 25px; border: 1px solid #ecd0c7; border-radius: 8px; background: #fbf7f4; }
  .danger-zone > h2 { transform: translateY(4px); color: var(--danger); font-family: var(--serif); font-size: 21px; font-weight: 400; }
  .danger-content { margin-top: 35px; display: flex; align-items: center; justify-content: space-between; }
  .danger-content h3 { color: var(--danger); font-family: var(--serif); font-size: 18px; font-weight: 400; }
  .danger-content p { margin-top: 7px; color: var(--muted); font-size: 16px; }
  .danger-controls { display: flex; gap: 12px; }
  .danger-controls input { width: 269px; height: 48px; padding: 0 16px; border: 1px solid #dfc8bf; border-radius: 6px; outline: 0; background: transparent; font-size: 14px; }
  .danger-controls button { width: 102px; height: 48px; border-radius: 6px; background: #e8bdaa; color: rgba(255,255,255,.72); font-family: var(--serif); font-size: 17px; }
  .danger-controls button:not(:disabled) { background: var(--danger); color: #fff; }

  .filter-button { padding: 0; display: grid; place-items: center; }
  .popover { position: absolute; z-index: 30; display: grid; padding: 6px; border: 1px solid var(--line-strong); border-radius: 7px; background: var(--surface); font-size: 16px; }
  .popover > button { padding: 10px 14px; text-align: left; }
  .popover > button:hover { background: var(--active-bg); }
  .clip-filter { top: 57px; right: 22px; }
  .collection-context { left: 100px; }
  /* 提升整个分类栏，避免正文的 transform 层遮挡菜单及点击。 */
  .clip-control-row > label { position: relative; z-index: 1; }
  .collection-picker { top: 78px; left: 0; width: 244px; }
  .password-menu { top: 61px; right: 35px; }
  .password-filter-button { display: flex; align-items: center; }
  .password-filter-menu { top: 63px; right: 18px; }
  .password-row-menu, .image-collection-menu { position: fixed; width: 248px; }
  .password-row-menu, .password-menu, .image-collection-menu { max-height: 320px; overflow-y: auto; }
  .popover > button[aria-pressed="true"] { background: var(--active-bg); }
  .date-menu { top: 58px; right: 0; width: 191px; }
  .shortcut-editor { top: 263px; right: 0; gap: 12px; padding: 16px; }
  .shortcut-editor label { display: grid; gap: 10px; }
  .shortcut-editor input, .collection-dialog input, .vault-gate input { height: 48px; padding: 0 15px; border: 1px solid var(--line-strong); border-radius: 7px; color: var(--ink); background: var(--surface); }
  .vault-gate { padding: 40px 32px; }
  .vault-gate form { width: 480px; display: grid; gap: 25px; }
  .vault-gate h1 { font-family: var(--serif); font-size: 29px; font-weight: 400; }
  .vault-gate label { display: grid; gap: 10px; }
  .vault-gate .primary-btn { height: 48px; }
  .interaction-error { color: var(--danger); font-size: 14px; margin-top: 10px; }
  .dialog-backdrop { position: absolute; inset: 0; z-index: 40; display: grid; place-items: center; background: #29282433; }
  .collection-dialog { width: 440px; padding: 25px; display: grid; gap: 22px; border: 1px solid var(--line-strong); border-radius: 8px; background: var(--window-bg); }
  .collection-dialog h2 { font-family: var(--serif); font-weight: 400; }
  .collection-dialog > div { display: flex; justify-content: flex-end; gap: 12px; }
  .collection-dialog button { height: 43px; padding: 0 18px; }
  .status-toast { position: absolute; z-index: 50; left: 50%; bottom: 22px; transform: translateX(-50%); padding: 12px 20px; border: 1px solid var(--line-strong); border-radius: 7px; background: var(--surface); color: var(--ink); }
  .empty-state { padding: 24px 18px; color: var(--muted); font-size: 16px; line-height: 24px; }
  .image-empty-detail { margin-top: 97px; }
  .image-failed { display: grid; place-content: center; padding: 20px; height: 100%; color: var(--muted); border: 1px solid var(--line); font: 16px/24px var(--serif); }
  .image-preview.image-failed { height: 480px; }
  .thumb-failed { width: 56px; font-size: 10px; line-height: 15px; }
  .recent-image-preview { display: block; width: 100%; height: 100%; object-fit: contain; }
  .clip-actions-menu { right: 31px; top: 63px; }
  .quick-actions { position: relative; }
  .quick-menu { bottom: 60px; right: 20px; }
  button:disabled { cursor: default; }
  button:focus-visible, input:focus-visible, summary:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .settings-page { overflow-y: auto; }
  .native-settings { margin: 24px 0 40px; padding: 20px 25px; border: 1px solid var(--line); border-radius: 7px; font-size: 16px; }
  .native-setting { display: flex; align-items: center; justify-content: space-between; margin: 20px 0; }
  .native-setting input { margin-left: 18px; width: 90px; height: 40px; padding: 0 10px; color: var(--ink); border: 1px solid var(--line-strong); background: var(--surface); }
  .native-setting button { padding: 0 20px; }
  .native-settings p { color: var(--muted); line-height: 24px; }
  .error-toast { color: var(--danger); max-width: 800px; }

  .navigation-toggle, .drawer-shade, .compact-titlebar { display: none; }
  .ref-collection-list { max-height: calc(100% - 491px); overflow-y: auto; }
  .ref-sidebar:has(.sidebar-lock) .ref-collection-list { max-height: calc(100% - 601px); }
  .collection-dialog { max-width: calc(100% - 32px); max-height: calc(100dvh - 32px); overflow-y: auto; }
  .collection-dialog input { min-width: 0; width: 100%; }
  .dialog-backdrop { padding: 16px 0; grid-template-columns: minmax(0, 1fr); justify-items: center; }
  .status-toast { max-width: calc(100% - 32px); overflow-wrap: anywhere; }

  /* 基准画布保留原始像素布局，其余尺寸按可用空间重排，不缩放字体。 */
  @media (max-width: 1585px), (max-height: 991px), (min-width: 1700px), (min-height: 1100px) {
    .ref-window { --sidebar-width: clamp(210px, calc(306px - (1586px - 100vw) * .2), 306px); --pane-padding: clamp(16px, 2vw, 31px); }
    .ref-sidebar { width: var(--sidebar-width); display: flex; flex-direction: column; padding: 24px 14px 15px; gap: 20px; }
    .ref-brand, .ref-primary-nav, .ref-collections-head, .ref-collection-list, .sidebar-lock, .ref-sidebar-footer { position: static; width: 100%; flex-shrink: 0; }
    .ref-brand { font-size: 36px; line-height: 46px; padding: 0 8px; }
    .ref-primary-nav { gap: 6px; }
    .ref-primary-nav button, .ref-collection-list button { width: 100%; padding-left: 12px; padding-right: 12px; }
    .ref-collections-head { padding: 0 8px; margin-top: 8px; }
    .ref-sidebar .ref-collection-list { display: block; flex: 1 1 0; min-height: 0; max-height: none; overflow-y: auto; }
    .ref-collection-list button > span { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .ref-sidebar-footer { margin-top: auto; padding: 0 8px; }
    .sidebar-lock { min-height: 48px; height: auto; padding: 10px 12px; }
    .ref-topbar, .ref-main { left: var(--sidebar-width); }
    .ref-topbar .ref-search { width: min(430px, calc(100% - 216px)); }
    .recent-page, .passwords-page:not(.vault-gate) { display: grid; grid-template-columns: minmax(260px, 38%) minmax(0, 1fr); overflow: hidden; }
    .clip-column, .password-list { position: relative; inset: auto; width: auto; min-width: 0; min-height: 0; display: flex; flex-direction: column; }
    .column-title, .password-list-head { flex-shrink: 0; padding-left: 18px; padding-right: 18px; }
    .clip-scroll, .password-scroll { position: relative; inset: auto; flex: 1; min-height: 0; padding: 0 8px; }
    .reference-scrollbar { display: none; }
    .reference-mode .clip-scroll, .reference-mode .password-scroll { scrollbar-width: thin; }
    .reference-mode .clip-scroll::-webkit-scrollbar, .reference-mode .password-scroll::-webkit-scrollbar { display: block; }
    .clip-row { grid-template-columns: minmax(0, 1fr) 74px; padding-left: 12px; padding-right: 10px; }
    .clip-row-main { overflow-wrap: anywhere; }
    .clip-detail, .password-detail { position: relative; inset: auto; min-width: 0; min-height: 0; padding: 0 var(--pane-padding); display: flex; flex-direction: column; overflow-y: auto; }
    .detail-title { flex-shrink: 0; }
    .clip-heading { width: 100%; flex-shrink: 0; overflow-wrap: anywhere; }
    .clip-control-row { height: auto; min-height: 126px; flex-shrink: 0; flex-wrap: wrap; gap: 18px; padding-top: 24px; }
    .clip-control-row > label, .select-button { max-width: 100%; }
    .select-button { grid-template-columns: 20px minmax(0, 1fr) 16px; }
    .select-button > span { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .copy-stack { margin-left: auto; flex-shrink: 0; }
    .clip-content-box { display: flex; flex-direction: column; flex: 1 0 300px; height: auto; min-height: 260px; }
    .clip-content { height: auto; min-height: 0; flex: 1; overflow-wrap: anywhere; }
    .clip-content-box footer { height: auto; min-height: 49px; flex-shrink: 0; flex-wrap: wrap; gap: 6px 16px; padding-top: 10px; padding-bottom: 10px; }
    .quick-actions { height: auto; min-width: 0; flex-shrink: 0; flex-wrap: wrap; margin-top: 24px; padding: 12px; }
    .quick-actions button:nth-of-type(n) { width: auto; max-width: 100%; white-space: normal; }
    .quick-actions .square-more { min-width: 40px; }
    .delete-clip { position: static; flex-shrink: 0; align-self: flex-end; margin: 24px 0; }
    .images-page { display: grid; grid-template-columns: minmax(0, 62%) minmax(0, 1fr); }
    .image-library { position: relative; inset: auto; width: auto; min-width: 0; min-height: 0; display: flex; flex-direction: column; padding: 36px var(--pane-padding) 0; }
    .image-library > h1 { position: static; flex-shrink: 0; margin: 0 0 22px; }
    .image-tools { position: relative; inset: auto; min-width: 0; flex-shrink: 0; gap: 10px; flex-wrap: wrap; }
    .image-search { width: auto; min-width: 140px; flex: 1; }
    .date-filter { width: 164px; }
    .image-scroll { position: relative; inset: auto; min-height: 0; flex: 1; margin-top: 25px; padding: 4px 5px 24px; }
    .image-grid { grid-template-columns: repeat(auto-fit, minmax(130px, 1fr)); grid-auto-rows: auto; }
    .image-grid button, .yesterday-grid button { width: 100%; height: auto; aspect-ratio: 174 / 215; }
    .image-grid img { position: absolute; inset: 0; }
    .image-detail { position: relative; inset: auto; min-width: 0; min-height: 0; padding: 41px var(--pane-padding) 24px; display: flex; flex-direction: column; overflow-y: auto; }
    .image-detail > h2 { position: static; flex-shrink: 0; margin-bottom: 29px; }
    .image-preview, .image-preview.image-failed { position: static; width: 100%; height: auto; aspect-ratio: 427 / 480; flex-shrink: 0; }
    .image-meta { position: static; margin: 22px 0; flex-shrink: 0; }
    .image-meta > div { height: auto; min-height: 35px; gap: 10px; flex-wrap: wrap; }
    .image-meta dt { gap: 10px; font-size: 17px; }
    .image-meta dd { margin-left: auto; text-align: right; overflow-wrap: anywhere; }
    .image-action { position: static; width: 100%; min-height: 43px; height: auto; flex-shrink: 0; margin-bottom: 10px; }
    .image-divider { position: static; width: 100%; margin: 14px 0 26px; flex-shrink: 0; }
    .password-list-head { grid-template-columns: minmax(0, 1fr) auto 26px; gap: 8px; }
    .password-list-head h2 { font-size: 19px; }
    .add-password { width: auto; padding: 0 10px; font-size: 16px; }
    .password-row { width: 100%; }
    .password-select > div { right: 62px; min-width: 0; }
    .password-select strong, .password-select span:not(.masked) { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .password-row time { font-size: 12px; }
    .password-form { margin-bottom: 32px; flex-shrink: 0; }
    .password-form > label { min-width: 0; }
    .field-with-action { display: flex; flex-wrap: wrap; gap: 8px; }
    .field-with-action input { padding-right: 16px; }
    .field-with-action button { position: static; margin-left: auto; }
    .password-field input { padding-right: 50px; }
    .password-field .eye-button { position: absolute; right: 10px; top: 12px; }
    .password-footer { position: sticky; left: auto; right: auto; bottom: 0; height: auto; min-height: 90px; margin: auto calc(-1 * var(--pane-padding)) 0; padding: 16px var(--pane-padding); gap: 12px; flex-shrink: 0; flex-wrap: wrap; background: var(--window-bg); z-index: 2; }
    .password-footer > button { width: auto; flex: 1; min-width: 130px; }
    .settings-content { position: relative; inset: auto; width: auto; margin: 34px var(--pane-padding) 40px; }
    .settings-row { width: 100%; height: auto; min-height: 110px; padding: 24px 0; gap: 20px; flex-wrap: wrap; }
    .settings-row:first-of-type { height: auto; min-height: 119px; }
    .settings-row > div:first-child { flex: 1 1 300px; }
    .settings-row .capture-control, .settings-row .transfer-actions { flex: 0 0 auto; margin-left: auto; }
    .shortcut-control, .website-button { margin-left: auto; max-width: 100%; flex-shrink: 0; }
    .danger-zone { height: auto; padding: 25px; }
    .danger-content { margin-top: 28px; gap: 20px; flex-wrap: wrap; }
    .danger-content > div:first-child { flex: 1 1 350px; }
    .danger-content p { overflow-wrap: anywhere; }
    .danger-controls { margin-left: auto; width: min(100%, 383px); }
    .danger-controls input { width: auto; min-width: 0; flex: 1; }
    .danger-controls button { flex-shrink: 0; }
    .native-setting { gap: 16px; flex-wrap: wrap; }
    .native-setting label { display: flex; flex-wrap: wrap; align-items: center; gap: 10px; }
    .native-setting input { margin-left: 0; }
    .vault-gate { padding: 32px var(--pane-padding); overflow-y: auto; }
    .vault-gate form { width: min(100%, 480px); }
    .vault-gate input { min-width: 0; width: 100%; }
    .popover { max-width: calc(100vw - 24px); max-height: min(320px, calc(100dvh - 80px)); overflow-y: auto; overflow-wrap: anywhere; }
    .popover > button, .shortcut-editor label, .shortcut-editor input { min-width: 0; }
  }

  @media (max-width: 1100px) {
    .ref-window { --sidebar-width: 0px; }
    .ref-sidebar { display: none; }
    .ref-sidebar.sidebar-open { display: flex; z-index: 35; width: min(306px, calc(100% - 58px)); padding-top: 52px; }
    .navigation-toggle { position: absolute; display: grid; place-items: center; top: 8px; left: 16px; z-index: 36; width: 36px; height: 36px; border: 1px solid var(--line-strong); border-radius: 7px; background: var(--surface); }
    .drawer-shade { display: block; position: absolute; inset: 0; z-index: 34; background: #29282433; }
    .compact-titlebar { display: block; position: absolute; top: 0; left: 0; right: 0; height: 46px; }
    .ref-topbar .ref-search { left: 68px; width: min(430px, calc(100% - 254px)); }
    .images-page { top: 46px; }
    .image-library { padding-top: 20px; }
    .image-detail { padding-top: 25px; }
  }

  @media (max-width: 760px) {
    .ref-topbar { height: 112px; }
    .ref-topbar .ref-search { left: 16px; right: 16px; top: 52px; width: auto; height: 45px; }
    .recent-page, .passwords-page, .settings-page { top: 112px; }
    .recent-page, .passwords-page:not(.vault-gate), .images-page { display: flex; flex-direction: column; overflow-y: auto; }
    .clip-column, .password-list { height: clamp(210px, 38dvh, 330px); flex: 0 0 auto; border-right: 0; border-bottom: 1px solid var(--line); }
    .clip-detail, .password-detail { flex: 1 0 auto; overflow: visible; }
    .clip-heading { font-size: 29px; line-height: 36px; max-height: 74px; }
    .clip-control-row { gap: 14px; }
    .select-button { width: min(244px, calc(100vw - 34px)); }
    .copy-stack { display: flex; width: 100%; gap: 10px; }
    .copy-stack button { flex: 1; min-width: 0; }
    .clip-content-box { flex: 0 0 auto; height: 340px; }
    .clip-content { font-size: 20px; line-height: 27px; }
    .quick-actions button:nth-of-type(n) { flex: 1 1 auto; padding: 0 10px; }
    .quick-actions button:last-of-type { flex: 0 0 40px; }
    .password-footer { position: static; margin-top: 20px; }
    .image-library { height: clamp(300px, 55dvh, 500px); flex: 0 0 auto; border-right: 0; border-bottom: 1px solid var(--line); }
    .image-detail { flex: 1 0 auto; overflow: visible; }
    .image-grid { grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); }
    .image-preview { max-height: 540px; object-fit: contain; }
    .image-detail > h2 { margin-bottom: 20px; }
    .settings-content > h1 { font-size: 36px; line-height: 42px; }
    .settings-row { gap: 16px; }
    .settings-row > div:first-child { flex-basis: 100%; }
    .settings-row .capture-control { margin-left: 0; }
    .settings-row .transfer-actions { margin-left: 0; max-width: 100%; gap: 12px; }
    .transfer-actions button { width: auto; padding: 0 18px; }
    .shortcut-control, .website-button { margin-left: 0; }
    .danger-zone { padding: 20px; }
    .danger-content { margin-top: 24px; }
    .native-settings { padding: 20px; }
    .collection-dialog > div { flex-wrap: wrap; }
    .collection-dialog { padding: 20px; }
  }

  @media (max-height: 650px) {
    .ref-sidebar { gap: 12px; padding-top: 12px; }
    .ref-sidebar.sidebar-open { padding-top: 48px; }
    .ref-brand { font-size: 31px; line-height: 38px; }
    .ref-primary-nav { gap: 3px; }
    .ref-primary-nav button { height: 38px; }
    .ref-collection-list button { height: 43px; }
    .ref-collections-head { margin-top: 0; }
  }

  @media (max-width: 480px) {
    .image-tools { flex-direction: column; align-items: stretch; }
    .image-search { flex: 0 0 53px; }
    .date-filter { width: 100%; }
    .image-library { height: auto; }
    .image-scroll { flex: none; height: clamp(220px, 40dvh, 360px); }
  }

  @media (max-height: 500px) {
    .ref-sidebar { overflow-y: auto; }
    .ref-sidebar .ref-collection-list { flex: 0 0 auto; overflow: visible; }
  }
</style>
