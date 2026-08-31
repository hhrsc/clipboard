<script>
  import { onMount } from 'svelte';
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
    Minus as IconMinus, Pencil as IconPencil, Image as IconPhoto,
    Pilcrow as IconPilcrow, Pin as IconPin, Plus as IconPlus,
    PencilRuler as IconRulerMeasure, Search as IconSearch, Settings as IconSettings,
    ShoppingBag as IconShoppingBag, Square as IconSquare, Trash2 as IconTrash,
    Upload as IconUpload, UserRound as IconUser, X as IconX
  } from '@lucide/svelte';

  export let activeTab = 'recent';
  /** @type {Array<{id: number, type?: string, content: string, timestamp?: string, time?: string, category?: string, tag?: string, code?: boolean, thumb?: string, isPinned?: boolean}>} */
  export let history = [];
  /** @type {Array<{id: number, title: string, username: string, password: string, time?: string, showPass?: boolean}>} */
  export let passwords = [];
  export let activeCategory = '全部';
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
  export let updateShortcutFn = async () => {};
  /** @type {(id: number | null, title: string, username: string, password: string) => Promise<number | boolean | undefined>} */
  export let savePasswordFn = async (id, title, username, password) => undefined;
  /** @type {(id: number) => Promise<void>} */
  export let deletePasswordFn = async (id) => {};
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
    { id: 9, content: 'IMG_2024_0521_1830.png', time: 'Yesterday', tag: 'PNG', thumb: '/reference-assets/recent-thumbnail.png', category: 'All clips' },
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
  let collectionMenu = false;
  let addingCollection = false;
  let collectionName = '';
  let editingCollection = '';
  let collectionContext = '';
  let collectionContextY = 0;
  let editingShortcut = false;
  let filterMenu = false;
  let dateFilter = 'All time';
  let dateMenu = false;
  let imageDimensions = '';
  let imageSize = '';
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
    referenceMode = import.meta.env.DEV && !isTauri() && ['recent', 'images', 'passwords', 'settings'].includes(page || '');
    if (referenceMode) activeTab = page || 'recent';
  });

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
    ? referenceClips.filter((item) => item.content.toLowerCase().includes(searchQuery.toLowerCase()))
        .filter((item) => activeCategory === '全部' || item.category === activeCategory)
        .filter((item) => recentFilter !== 'pinned' || item.isPinned)
    : liveClips;
  $: activeReferenceClip = referenceClips.find((item) => item.id === selectedReferenceClipId) || referenceClips[0];
  $: detailClip = referenceMode ? activeReferenceClip : selectedClip;
  $: livePasswordRows = filteredPasswords.map((item) => ({ ...item, time: displayTime(new Date(item.id).toISOString()) }));
  $: passwordRows = referenceMode
    ? referencePasswords.filter((item) => `${item.title} ${item.username}`.toLowerCase().includes(pwdSearchQuery.toLowerCase()))
    : livePasswordRows;
  $: selectedPassword = passwordRows.find((item) => item.id === selectedPasswordId) || passwordRows[0] || null;
  $: if (!addingPassword) {
    draftTitle = selectedPassword?.title || '';
    draftUsername = selectedPassword?.username || '';
    draftPassword = selectedPassword?.password || '';
  }
  $: liveImages = filteredImages.map((item) => ({ ...item, src: getImgSrcFn(item.content), group: imageGroup(item) }));
  $: imageRows = (referenceMode ? referenceImages : liveImages)
    .filter((item) => dateFilter === 'All time' || item.group === dateFilter)
    .filter((item) => !referenceMode || !searchQuery || `${item.src} ${item.group}`.toLowerCase().includes(searchQuery.toLowerCase()));
  $: selectedImage = imageRows.find((item) => item.id === selectedImageId) || imageRows[0] || null;
  $: todayImages = imageRows.filter((item) => item.group === 'Today');
  $: yesterdayImages = imageRows.filter((item) => item.group === 'Yesterday');
  $: earlierImages = imageRows.filter((item) => item.group === 'Earlier');
  $: activeCollections = referenceMode
    ? (activeTab === 'passwords' ? passwordCollections : recentCollections)
    : activeTab === 'passwords'
      ? [{ label: 'All items', count: passwords.length, icon: IconFolder }]
      : categories.map((category) => ({ label: category === '全部' ? 'All clips' : category, count: category === '全部' ? history.length : history.filter((item) => item.category === category).length, icon: category === '全部' ? IconArchive : IconFolder }));
  $: shortcutKeys = (referenceMode ? referenceShortcut : shortcutValue).split('+');
  $: if (!vaultUnlocked && !referenceMode) {
    addingPassword = false;
    draftPassword = '';
    revealPassword = false;
  }

  /** @param {{id: number}} item */
  function chooseClip(item) {
    if (referenceMode) selectedReferenceClipId = item.id;
    else selectClipFn(item.id);
  }

  /** @param {{id: number, src: string, content?: string, group?: string}} item */
  function chooseImage(item) {
    selectedImageId = item.id;
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

  function removePassword() {
    if (!selectedPassword) return;
    if (referenceMode) referencePasswords = referencePasswords.filter((item) => item.id !== selectedPassword.id);
    else deletePasswordFn(selectedPassword.id);
    passwordMenu = false;
  }

  function addCollection() {
    if (!collectionName.trim()) return;
    if (referenceMode) {
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
    const result = format === 'html'
      ? `<p>${content.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('\n', '<br>')}</p>`
      : format === 'single-line' ? content.replace(/\s*\n\s*/g, ' ') : content;
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
    { id: 'recent', label: 'Recent', icon: IconClock },
    { id: 'images', label: 'Images', icon: IconPhoto },
    { id: 'passwords', label: 'Passwords', icon: IconLock },
    { id: 'settings', label: 'Settings', icon: IconSettings }
  ];
</script>

<svelte:window on:click={() => (collectionContext = '')} on:keydown={(event) => { if (event.key === 'Escape') { collectionContext = ''; addingCollection = false; editingCollection = ''; } }} />

<div class="ref-window" class:reference-mode={referenceMode}>
  <aside class="ref-sidebar">
    <div class="ref-brand" data-tauri-drag-region>Clipboard</div>

    <nav class="ref-primary-nav" aria-label="Primary">
      {#each navItems as item}
        <button class:active={activeTab === item.id} on:click={() => (activeTab = item.id)}>
          <svelte:component this={item.icon} size={24} strokeWidth={1.55} />
          <span>{item.label}</span>
        </button>
      {/each}
    </nav>

    <div class="ref-collections-head">
      <span>Collections</span>
      <button aria-label="Add collection" disabled={!referenceMode && activeTab === 'passwords'} title={!referenceMode && activeTab === 'passwords' ? 'Password collections are not supported by the existing vault' : 'Add collection'} on:click={() => (addingCollection = true)}><IconPlus size={19} strokeWidth={1.45} /></button>
    </div>

    <nav class="ref-collection-list" aria-label="Collections">
      {#each activeCollections as item, index}
        <button
          class:active={activeTab === 'recent' && (activeCategory === '全部' ? index === 0 : activeCategory === item.label)}
          on:click={() => (activeCategory = item.label === 'All clips' || item.label === 'All items' ? '全部' : item.label)}
          on:contextmenu|preventDefault={(event) => { if (!referenceMode && activeTab !== 'passwords' && index > 0) { collectionContext = item.label; collectionContextY = Math.min(event.clientY, innerHeight - 150); } }}
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

    {#if collectionContext}<div class="popover collection-context" style:top={`${collectionContextY}px`}><button on:click={() => { editingCollection = collectionContext; collectionName = collectionContext; addingCollection = true; }}>Rename collection</button><button on:click={() => removeCollectionFn(collectionContext)}>Delete collection</button></div>{/if}

    {#if activeTab === 'passwords'}
      <button class="sidebar-lock" on:click={() => !referenceMode && lockVaultFn()}><IconLock size={22} strokeWidth={1.45} />Lock vault</button>
    {/if}

    <button class="ref-sidebar-footer" on:click={() => { activeTab = 'recent'; activeCategory = '全部'; recentFilter = 'all'; searchQuery = ''; }}>
      <span>{activeTab === 'passwords' ? (referenceMode ? '120 items' : `${passwords.length} items`) : (referenceMode ? '328 clips' : `${history.length} clips`)}</span>
      <IconChevronDown size={16} strokeWidth={1.45} />
    </button>
  </aside>

  <div class="window-controls" data-tauri-drag-region>
    <button aria-label="Minimize" on:click={minimizeWindow}><IconMinus size={17} strokeWidth={1.35} /></button>
    <button aria-label="Maximize" on:click={maximizeWindow}><IconSquare size={15} strokeWidth={1.35} /></button>
    <button aria-label="Close" on:click={closeWindow}><IconX size={18} strokeWidth={1.35} /></button>
  </div>

  {#if activeTab !== 'images'}
    <header class="ref-topbar" data-tauri-drag-region>
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
    <main class="ref-main recent-page">
      <section class="clip-column">
        <header class="column-title"><h2>Recent clips</h2><button class="filter-button" aria-label="Filter clips" on:click={() => (filterMenu = !filterMenu)}><IconFilter2 size={19} strokeWidth={1.45} /></button></header>
        {#if filterMenu}<div class="popover clip-filter"><button on:click={() => { recentFilter = 'all'; filterMenu = false; }}>All clips</button><button on:click={() => { recentFilter = 'pinned'; filterMenu = false; }}>Pinned clips</button></div>{/if}
        <div class="clip-scroll">
          {#each clipRows as item, index}
            <button class="clip-row" class:selected={(referenceMode ? selectedReferenceClipId : selectedClip?.id) === item.id} on:click={() => chooseClip(item)} on:dblclick={() => item.type === 'image' ? copyImageFn(item.content) : copyPlainText(item.content)}>
              <div class="clip-row-main">
                {#if item.thumb}{#if failedImages.has(item.thumb)}<span class="thumb-failed">Image unavailable</span>{:else}<img src={item.thumb} alt="" on:error={() => item.thumb && failImage(item.thumb)} />{/if}{/if}
                <span class:mono={item.code} class:masked-clip={item.tag === 'PASSWORD'}>{#if item.tag === 'MD'}<span class="md-heading">{item.content.split('\n')[0]}</span>{'\n' + item.content.split('\n').slice(1).join('\n')}{:else}{clipLabel(item)}{/if}</span>
                {#if item.tag}<small>{item.tag}</small>{/if}
              </div>
              <time>{item.time}</time>
            </button>
          {/each}
          {#if !clipRows.length}<p class="empty-state">{searchQuery || activeCategory !== '全部' || recentFilter === 'pinned' ? 'No matching clips' : 'No clips yet. Copy text or an image to get started.'}</p>{/if}
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
          {#if clipMenu}<div class="popover clip-actions-menu"><button on:click={pinClip}>{detailClip.isPinned ? 'Unpin clip' : 'Pin clip'}</button><button on:click={removeClip}>Delete clip</button></div>{/if}
          <h1 class="clip-heading">{clipLabel(detailClip)}</h1>
          <div class="clip-control-row">
            <label>Collection
              <button class="select-button" on:click={() => (collectionMenu = !collectionMenu)}><IconFolder size={19} strokeWidth={1.45} /><span>{referenceMode ? detailClip.category : detailClip.category === '全部' ? 'All clips' : detailClip.category || 'All clips'}</span><IconChevronDown size={16} strokeWidth={1.45} /></button>
              {#if collectionMenu}<div class="popover collection-picker">{#each activeCollections as collection}<button on:click={() => assignCollection(collection.label)}>{collection.label}</button>{/each}</div>{/if}
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
            {#if quickMenu}<div class="popover quick-menu"><button on:click={() => { copyClip(); quickMenu = false; }}>Copy plain text</button></div>{/if}
          </fieldset>

          <button class="danger-btn delete-clip" on:click={removeClip}><IconTrash size={19} strokeWidth={1.45} />Delete clip</button>
        {:else}<p class="empty-state">Select a clip to view its contents.</p>{/if}
      </section>
    </main>
  {:else if activeTab === 'images'}
    <main class="ref-main images-page">
      <section class="image-library">
        <h1 data-tauri-drag-region>Copied images</h1>
        <div class="image-tools">
          <label class="ref-search image-search"><IconSearch size={20} strokeWidth={1.45} /><input bind:value={searchQuery} placeholder="Search images" /></label>
          <button class="date-filter" on:click={() => (dateMenu = !dateMenu)}><IconCalendar size={20} strokeWidth={1.45} /><span>{dateFilter}</span><IconChevronDown size={16} strokeWidth={1.45} /></button>
          {#if dateMenu}<div class="popover date-menu">{#each ['All time', 'Today', 'Yesterday'] as range}<button on:click={() => { dateFilter = range; dateMenu = false; }}>{range}</button>{/each}</div>{/if}
        </div>

        <div class="image-scroll">
          {#if todayImages.length || referenceMode}<h2>Today</h2>{/if}
          <div class="image-grid">
            {#each todayImages as item}
              <button class:selected={selectedImage?.id === item.id} on:click={() => chooseImage(item)}>{#if failedImages.has(item.src)}<span class="image-failed">Image unavailable</span>{:else}<img src={item.src} alt="Clipboard item" on:error={() => failImage(item.src)} />{/if}</button>
            {/each}
          </div>
          {#if yesterdayImages.length}
            <h2 class="yesterday-title">Yesterday</h2>
            <div class="image-grid yesterday-grid">
              {#each yesterdayImages as item}
                <button class:selected={selectedImage?.id === item.id} on:click={() => chooseImage(item)}>{#if failedImages.has(item.src)}<span class="image-failed">Image unavailable</span>{:else}<img src={item.src} alt="Clipboard item" on:error={() => failImage(item.src)} />{/if}</button>
              {/each}
            </div>
          {/if}
          {#if earlierImages.length}<h2 class="yesterday-title">Earlier</h2><div class="image-grid">{#each earlierImages as item}<button class:selected={selectedImage?.id === item.id} on:click={() => chooseImage(item)}>{#if failedImages.has(item.src)}<span class="image-failed">Image unavailable</span>{:else}<img src={item.src} alt="Clipboard item" on:error={() => failImage(item.src)} />{/if}</button>{/each}</div>{/if}
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
          <button class="danger-btn image-action" on:click={() => !referenceMode && clearImagesFn()}><IconTrash size={19} strokeWidth={1.45} />Clear images</button>
        {:else}<p class="empty-state image-empty-detail">Select an image to preview it.</p>{/if}
      </section>
    </main>
  {:else if activeTab === 'passwords'}
    {#if !referenceMode && !vaultUnlocked}
      <main class="ref-main passwords-page vault-gate">
        <form on:submit|preventDefault={vaultExists ? unlockVaultFn : setupVaultFn}>
          <h1>{vaultExists ? 'Unlock password vault' : 'Create password vault'}</h1>
          {#if vaultExists}
            <label>Master password<input type="password" autocomplete="current-password" bind:value={unlockPassword} required /></label>
          {:else}
            <label>Master password<input type="password" autocomplete="new-password" bind:value={masterPassword} required /></label>
            <label>Confirm master password<input type="password" autocomplete="new-password" bind:value={masterPasswordConfirm} required /></label>
          {/if}
          {#if vaultError}<p class="interaction-error" role="alert">{vaultError}</p>{/if}
          <button class="primary-btn" disabled={vaultBusy}>{vaultBusy ? 'Please wait…' : vaultExists ? 'Unlock vault' : 'Create vault'}</button>
        </form>
      </main>
    {:else}
    <main class="ref-main passwords-page">
      <section class="password-list">
        <header class="password-list-head"><h2>Password vault</h2><button class="primary-btn add-password" on:click={beginPassword}>Add password</button><IconFilter2 size={19} strokeWidth={1.45} /></header>
        <div class="password-scroll">
          {#each passwordRows as item}
            <button class="password-row" class:selected={!addingPassword && selectedPassword?.id === item.id} on:click={() => { selectedPasswordId = item.id; addingPassword = false; revealPassword = false; }}>
              <div><strong>{item.title}</strong><span>{item.username}</span><span class="masked">•••••••••••••</span></div>
              <time>{item.time}</time>
              <span class="row-more"><IconDots size={20} strokeWidth={1.45} /></span>
            </button>
          {/each}
          {#if !passwordRows.length}<p class="empty-state">{pwdSearchQuery ? 'No matching passwords' : 'No saved passwords yet.'}</p>{/if}
        </div>
        {#if referenceMode && !pwdSearchQuery}<div class="reference-scrollbar password-scrollbar" aria-hidden="true"></div>{/if}
      </section>

      <section class="password-detail">
        <header class="detail-title">
          <h2>Item details</h2>
          <div class="icon-actions"><button aria-label="Edit" on:click={() => titleInput?.focus()}><IconPencil size={21} strokeWidth={1.45} /></button><button aria-label="More" on:click={() => (passwordMenu = !passwordMenu)}><IconDots size={22} strokeWidth={1.45} /></button></div>
        </header>
        {#if passwordMenu}<div class="popover password-menu"><button on:click={removePassword}>Delete password</button></div>{/if}
        {#if selectedPassword || addingPassword}
          <form id="password-details" class="password-form" on:submit|preventDefault={savePassword}>
            <label>Title<input bind:this={titleInput} bind:value={draftTitle} required /></label>
            <label>Username<div class="field-with-action"><input aria-label="Username" bind:value={draftUsername} /><button type="button" on:click={() => copyPlainText(draftUsername)}><IconCopy size={19} strokeWidth={1.45} />Copy username</button></div></label>
            <label>Password<div class="field-with-action password-field"><input aria-label="Password" type={revealPassword ? 'text' : 'password'} bind:value={draftPassword} required /><button class="eye-button" type="button" aria-label={revealPassword ? 'Hide password' : 'Show password'} on:click={() => (revealPassword = !revealPassword)}><IconEye size={20} strokeWidth={1.45} /></button><button type="button" on:click={() => copyPlainText(draftPassword)}><IconCopy size={19} strokeWidth={1.45} />Copy password</button></div></label>
          </form>
          {#if vaultError && !referenceMode}<p class="interaction-error" role="alert">{vaultError}</p>{/if}
        {/if}
        <footer class="password-footer"><button class="outline-btn lock-detail" on:click={() => !referenceMode && lockVaultFn()}><IconLock size={21} strokeWidth={1.45} />Lock vault</button><button class="primary-btn save-changes" disabled={savingPassword || (!selectedPassword && !addingPassword)} type="submit" form="password-details">{savingPassword ? 'Saving…' : 'Save changes'}</button></footer>
      </section>
    </main>
    {/if}
  {:else}
    <main class="ref-main settings-page">
      <section class="settings-content">
        <h1>General settings</h1>
        <div class="settings-row">
          <div><h2>Clipboard capture</h2><p>Automatically save everything you copy.</p></div>
          <div class="capture-control"><button aria-label="Toggle clipboard capture" aria-pressed={captureEnabled} disabled={!referenceMode && (!storeReady || settingsBusy)} class:active={captureEnabled} class="toggle" on:click={() => referenceMode ? captureEnabled = !captureEnabled : toggleCaptureFn()}><span></span></button><span>{captureEnabled ? 'On' : 'Off'}</span></div>
        </div>
        <div class="settings-row">
          <div><h2>Global shortcut</h2><p>Press this shortcut to open Clipboard anywhere.</p></div>
          <button class="shortcut-control" on:click={() => (editingShortcut = !editingShortcut)}>{#each shortcutKeys as key, index}{#if index}<span>+</span>{/if}<kbd>{key}</kbd>{/each}<IconChevronDown size={16} strokeWidth={1.45} /></button>
        </div>
        {#if editingShortcut}<form class="popover shortcut-editor" on:submit|preventDefault={async () => { if (referenceMode) referenceShortcut = shortcutValue; else await updateShortcutFn(); if (!shortcutError) editingShortcut = false; }}><label>Global shortcut<input bind:value={shortcutValue} /></label><button class="outline-btn" disabled={shortcutBusy}>Apply</button>{#if shortcutError}<p role="alert">{shortcutError}</p>{/if}</form>{/if}
        <div class="settings-row">
          <div><h2>Password transfer</h2><p>Import and export your saved passwords.</p></div>
          <div class="transfer-actions"><button class="outline-btn" on:click={requestPasswordImportFn}><IconUpload size={21} strokeWidth={1.45} />Import</button><button class="outline-btn" on:click={exportPasswordsFn}><IconDownload size={21} strokeWidth={1.45} />Export</button></div>
        </div>
        <div class="settings-row last-setting">
          <div><h2>Official website</h2><p>Visit our website to learn more and get help.</p></div>
          <button class="outline-btn website-button" on:click={openOfficialWebsiteFn}><IconExternalLink size={21} strokeWidth={1.45} />Open website</button>
        </div>

        <section class="danger-zone">
          <h2>Danger zone</h2>
          <div class="danger-content"><div><h3>Clear all local app data</h3><p>This permanently deletes all clips, collections, and settings from this device.</p></div><div class="danger-controls"><input bind:value={clearDataConfirmation} placeholder="Type DELETE to confirm" /><button disabled={clearDataConfirmation !== 'DELETE' || clearDataBusy} on:click={() => !referenceMode && clearDataFn()}>{clearDataBusy ? 'Deleting…' : 'Delete'}</button></div></div>
          {#if clearDataError}<p class="interaction-error" role="alert">{clearDataError}</p>{/if}
        </section>
        {#if !referenceMode}<details class="native-settings"><summary>Desktop & storage</summary><div class="native-setting"><span>Launch at startup</span><button aria-label="Toggle launch at startup" class="outline-btn" disabled={settingsBusy} on:click={toggleAutostartFn}>{autostartEnabled ? 'On' : 'Off'}</button></div>{#if autostartError}<p class="interaction-error">{autostartError}</p>{/if}<form class="native-setting" on:submit|preventDefault={saveRetentionFn}><label>Unpinned text retention (hours)<input type="number" min="1" max="8760" step="1" bind:value={retentionHours} /></label><button class="outline-btn" disabled={settingsBusy || !storeReady}>Save retention</button></form><p>Pinned and collected text is retained. Recent text: 50 items. Images: 10 items / 80 MiB. Closing the window hides it to the tray.</p></details>{/if}
      </section>
    </main>
  {/if}
  {#if addingCollection}<div class="dialog-backdrop"><form class="collection-dialog" on:submit|preventDefault={addCollection}><h2>{editingCollection ? 'Rename collection' : 'New collection'}</h2><input aria-label="Collection name" bind:value={collectionName} required /><div><button type="button" class="outline-btn" on:click={() => { addingCollection = false; editingCollection = ''; }}>Cancel</button><button class="primary-btn">{editingCollection ? 'Save collection' : 'Create collection'}</button></div></form></div>{/if}
  {#if showToast && !referenceMode}<div class="status-toast" role="status">{toastMsg}</div>{/if}
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
    min-width: 1100px;
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
  .password-row > div { position: absolute; left: 18px; top: 17px; display: grid; }
  .password-row strong { transform: translateY(-2px); font-family: "Segoe UI", Arial, sans-serif; font-size: 18px; font-weight: 400; line-height: 24px; }
  .password-row span, .password-row time { color: #69645f; font-size: 14px; line-height: 20px; }
  .password-row > div > span:not(.masked) { transform: translateY(-3px); font-size: 14.5px; }
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
  .clip-control-row > label { position: relative; }
  .collection-picker { top: 78px; left: 0; width: 244px; }
  .password-menu { top: 61px; right: 35px; }
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

  @media (max-width: 1200px) {
    .ref-window { transform-origin: left top; }
  }
</style>
