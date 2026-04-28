<script>
  import { invoke } from '@tauri-apps/api/core';
  import { afterUpdate, onMount } from 'svelte';
  import { fade, fly, scale } from 'svelte/transition';

  const DEFAULT_CATEGORY = '全部';
  const HISTORY_KEY = 'clip_v5_split';
  const LAST_DATA_KEY = 'clip_v5_last_data';
  const PASSWORDS_KEY = 'clip_v5_passwords';
  const CATEGORIES_KEY = 'clip_v5_categories';

  let activeTab = 'clipboard';
  let showToast = false;
  let toastMsg = 'Copied!';

  let searchQuery = '';
  let history = [];
  let lastData = '';

  let pwdSearchQuery = '';
  let passwords = [];
  let newPwdTitle = '';
  let newPwdUser = '';
  let newPwdPass = '';

  let showNewPwd = false;
  let isAppCopying = false;

  let categories = [DEFAULT_CATEGORY];
  let activeCategory = DEFAULT_CATEGORY;
  let isAddingCat = false;
  let newCatName = '';

  let showContextMenu = false;
  let menuX = 0;
  let menuY = 0;
  let targetCategory = '';

  let isRenamingCat = false;
  let renameCatName = '';

  let clipboardTabEl;
  let passwordTabEl;
  let tabLineX = 0;
  let tabLineW = 0;

  function syncTabIndicator() {
    const activeTabEl = activeTab === 'clipboard' ? clipboardTabEl : passwordTabEl;
    if (!activeTabEl) return;

    const nextX = activeTabEl.offsetLeft;
    const nextW = activeTabEl.offsetWidth;
    if (nextX !== tabLineX) tabLineX = nextX;
    if (nextW !== tabLineW) tabLineW = nextW;
  }

  function handleCategoryWheel(event) {
    if (!event.currentTarget) return;
    const delta = Math.abs(event.deltaY) > Math.abs(event.deltaX) ? event.deltaY : event.deltaX;
    event.currentTarget.scrollLeft += delta;
  }

  afterUpdate(syncTabIndicator);

  function updateAndSaveHistory(newArray) {
    const EXPIRATION_MS = 24 * 60 * 60 * 1000;
    const now = Date.now();

    const valid = newArray.filter((item) => {
      if (item.type === 'text' && (!item.category || item.category === DEFAULT_CATEGORY)) {
        return now - item.id < EXPIRATION_MS;
      }
      return true;
    });

    const imgs = valid.filter((item) => item.type !== 'text');
    const categorizedTexts = valid.filter(
      (item) => item.type === 'text' && item.category && item.category !== DEFAULT_CATEGORY
    );
    const uncategorizedTexts = valid.filter(
      (item) => item.type === 'text' && (!item.category || item.category === DEFAULT_CATEGORY)
    );

    const merged = [...imgs.slice(0, 10), ...categorizedTexts, ...uncategorizedTexts.slice(0, 50)];
    merged.sort((a, b) => b.id - a.id);

    history = merged;
    localStorage.setItem(HISTORY_KEY, JSON.stringify(history));
    localStorage.setItem(LAST_DATA_KEY, lastData);
  }

  function savePasswords() {
    localStorage.setItem(PASSWORDS_KEY, JSON.stringify(passwords));
  }

  function saveCategories() {
    localStorage.setItem(CATEGORIES_KEY, JSON.stringify(categories));
  }

  onMount(() => {
    const saved = localStorage.getItem(HISTORY_KEY);
    if (saved) {
      try {
        history = JSON.parse(saved);
      } catch {
        history = [];
      }
    }

    const savedLastData = localStorage.getItem(LAST_DATA_KEY);
    if (savedLastData) lastData = savedLastData;

    const savedPwds = localStorage.getItem(PASSWORDS_KEY);
    if (savedPwds) {
      try {
        passwords = JSON.parse(savedPwds);
      } catch {
        passwords = [];
      }
    }

    const savedCats = localStorage.getItem(CATEGORIES_KEY);
    if (savedCats) {
      try {
        categories = JSON.parse(savedCats);
      } catch {
        categories = [DEFAULT_CATEGORY];
      }
    }

    if (!categories.includes(DEFAULT_CATEGORY)) {
      categories = [DEFAULT_CATEGORY, ...categories];
      saveCategories();
    }

    updateAndSaveHistory(history);

    const clipboardInterval = setInterval(async () => {
      const now = Date.now();
      const EXPIRATION_MS = 24 * 60 * 60 * 1000;
      if (
        history.some(
          (item) =>
            item.type === 'text' &&
            (!item.category || item.category === DEFAULT_CATEGORY) &&
            now - item.id > EXPIRATION_MS
        )
      ) {
        updateAndSaveHistory(history);
      }

      try {
        const data = await invoke('get_clipboard_data');
        if (data) {
          const [type, content] = data;
          if (content !== lastData) {
            if (isAppCopying && type === 'image') {
              lastData = content;
              isAppCopying = false;
              updateAndSaveHistory(history);
              return;
            }

            const defaultCat =
              type === 'text' && activeCategory !== DEFAULT_CATEGORY ? activeCategory : DEFAULT_CATEGORY;
            const newItem = {
              type,
              content,
              id: Date.now(),
              timestamp: new Date().toLocaleString(),
              category: defaultCat
            };

            lastData = content;
            updateAndSaveHistory([newItem, ...history]);
          }
        }
      } catch {
        // ignore clipboard poll errors
      }
    }, 2000);

    const handlePaste = (e) => {
      const active = document.activeElement;
      if (active && (active.tagName === 'INPUT' || active.tagName === 'TEXTAREA')) return;
      if (!e.clipboardData) return;

      const items = e.clipboardData.items;
      for (let i = 0; i < items.length; i++) {
        if (items[i].type.includes('image')) {
          const blob = items[i].getAsFile();
          if (!blob) continue;
          const reader = new FileReader();
          reader.onload = (event) => {
            const result = event?.target?.result;
            if (typeof result !== 'string') return;
            const base64 = result.split(',')[1];
            if (!base64) return;

            const content = `image|${base64}`;
            if (content !== lastData) {
              const newItem = {
                type: 'image',
                content,
                id: Date.now(),
                timestamp: new Date().toLocaleString(),
                category: DEFAULT_CATEGORY
              };
              lastData = content;
              updateAndSaveHistory([newItem, ...history]);
              triggerToast('Image Pasted!');
            }
          };
          reader.readAsDataURL(blob);
        } else if (items[i].type === 'text/plain') {
          items[i].getAsString((text) => {
            if (text !== lastData) {
              const defaultCat = activeCategory !== DEFAULT_CATEGORY ? activeCategory : DEFAULT_CATEGORY;
              const newItem = {
                type: 'text',
                content: text,
                id: Date.now(),
                timestamp: new Date().toLocaleString(),
                category: defaultCat
              };
              lastData = text;
              updateAndSaveHistory([newItem, ...history]);
              triggerToast('Text Pasted!');
            }
          });
        }
      }
    };

    window.addEventListener('paste', handlePaste);

    return () => {
      clearInterval(clipboardInterval);
      window.removeEventListener('paste', handlePaste);
    };
  });

  function triggerToast(msg = 'Copied!') {
    toastMsg = msg;
    showToast = true;
    setTimeout(() => {
      showToast = false;
    }, 900);
  }

  async function copyText(text) {
    if (!text) return;
    await invoke('set_clipboard_text', { text });
    lastData = text;
    updateAndSaveHistory(history);
    triggerToast('Copied');
  }

  async function copyImage(content) {
    if (!content) return;
    try {
      isAppCopying = true;
      const base64 = content.includes('|') ? content.split('|')[1] : content;
      await invoke('set_clipboard_image', { base64 });
      triggerToast('Image Copied');
      setTimeout(() => {
        isAppCopying = false;
      }, 3000);
    } catch {
      triggerToast('Failed to copy');
    }
  }

  async function deleteItem(id) {
    const item = history.find((it) => it.id === id);
    const newHistory = history.filter((it) => it.id !== id);
    if (item && item.content === lastData) {
      await invoke('set_clipboard_text', { text: '' });
      lastData = '';
    }
    updateAndSaveHistory(newHistory);
  }

  async function clearImages() {
    const hasLastData = history.some((it) => it.type !== 'text' && it.content === lastData);
    const newHistory = history.filter((item) => item.type === 'text');
    if (hasLastData) {
      await invoke('set_clipboard_text', { text: '' });
      lastData = '';
    }
    updateAndSaveHistory(newHistory);
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

  function changeCategory(id, newCat) {
    const newHistory = history.map((item) => (item.id === id ? { ...item, category: newCat } : item));
    updateAndSaveHistory(newHistory);
  }

  function handleContextMenu(e, cat) {
    if (cat === DEFAULT_CATEGORY) return;
    targetCategory = cat;
    menuX = e.clientX;
    menuY = e.clientY;
    showContextMenu = true;
  }

  function closeContextMenu() {
    showContextMenu = false;
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

  function addPassword() {
    if (!newPwdTitle.trim() || !newPwdPass.trim()) return;
    const newItem = {
      id: Date.now(),
      title: newPwdTitle.trim(),
      username: newPwdUser.trim(),
      password: newPwdPass,
      showPass: false
    };
    passwords = [newItem, ...passwords];
    savePasswords();
    newPwdTitle = '';
    newPwdUser = '';
    newPwdPass = '';
    showNewPwd = false;
    triggerToast('Saved');
  }

  function deletePassword(id) {
    passwords = passwords.filter((pwd) => pwd.id !== id);
    savePasswords();
  }

  function clearAllPasswords() {
    passwords = [];
    savePasswords();
  }

  function togglePassword(id) {
    passwords = passwords.map((pwd) => (pwd.id === id ? { ...pwd, showPass: !pwd.showPass } : pwd));
  }

  function getImgSrc(content) {
    try {
      const base64 = content.includes('|') ? content.split('|')[1] : content;
      return `data:image/png;base64,${base64}`;
    } catch {
      return '';
    }
  }

  $: filteredImages = history.filter((item) => item.type !== 'text');
  $: displayedText = history
    .filter((item) => item.type === 'text')
    .filter((item) => activeCategory === DEFAULT_CATEGORY || (item.category || DEFAULT_CATEGORY) === activeCategory)
    .filter((item) => !searchQuery || item.content.toLowerCase().includes(searchQuery.toLowerCase()));
  $: filteredPasswords = pwdSearchQuery
    ? passwords.filter((pwd) => pwd.title.toLowerCase().includes(pwdSearchQuery.toLowerCase()))
    : passwords;
</script>

<svelte:window on:click={closeContextMenu} on:resize={syncTabIndicator} />

<main class="app-shell">
  <nav class="top-tabs">
    <button
      class="tab-btn {activeTab === 'clipboard' ? 'active' : ''}"
      bind:this={clipboardTabEl}
      on:click={() => (activeTab = 'clipboard')}
    >
      Clipboard
    </button>
    <button
      class="tab-btn {activeTab === 'passwords' ? 'active' : ''}"
      bind:this={passwordTabEl}
      on:click={() => (activeTab = 'passwords')}
    >
      Password Book
    </button>
    <span
      class="tab-indicator"
      aria-hidden="true"
      style="width: {tabLineW}px; transform: translateX({tabLineX}px);"
    ></span>
  </nav>

  <div class="search-wrap">
    <div class="search-bar">
      <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      {#if activeTab === 'clipboard'}
        <input type="text" placeholder="Search stored clipboard..." bind:value={searchQuery} />
      {:else}
        <input type="text" placeholder="Search stored passwords..." bind:value={pwdSearchQuery} />
      {/if}
      <kbd>Ctrl K</kbd>
    </div>
  </div>

  {#if activeTab === 'clipboard'}
    <div class="workspace" in:fade={{ duration: 170 }}>
      <section class="panel panel-image">
        <header class="panel-head">
          <h3>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="3" />
              <circle cx="8.6" cy="8.6" r="1.6" />
              <polyline points="21 15 15.7 9.8 5 21" />
            </svg>
            Image History
          </h3>
          <button class="clear-btn" on:click={clearImages}>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6" />
              <path d="M8 6v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
            </svg>
            Clear All
          </button>
        </header>

        {#if filteredImages.length === 0}
          <div class="empty-state" in:fade={{ duration: 260 }}>
            <div class="empty-illustration">
              <svg viewBox="0 0 100 100" fill="none">
                <rect x="23" y="24" width="54" height="52" rx="11" stroke="#7fb4f3" stroke-width="3.4" />
                <path d="M34 60l11-16 11 12 9-9 11 13" stroke="#6da5ea" stroke-width="4" stroke-linecap="round" />
                <circle cx="63.5" cy="40.5" r="4.5" fill="#7fb4f3" />
              </svg>
            </div>
            <p class="empty-title">No images yet</p>
            <p class="empty-sub">Images you copy will appear here for quick access.</p>
          </div>
        {:else}
          <div class="scroll-v image-grid">
            {#each filteredImages as item (item.id)}
              <div
                class="image-card card-pop"
                role="button"
                tabindex="0"
                on:click={() => copyImage(item.content)}
                on:keydown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    copyImage(item.content);
                  }
                }}
                in:fly={{ y: 12, duration: 260 }}
              >
                <img src={getImgSrc(item.content)} alt="clipboard" />
                <span class="image-card-overlay">Copy</span>
                <button class="icon-btn danger image-del" on:click|stopPropagation={() => deleteItem(item.id)} aria-label="Delete image">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18" />
                    <line x1="6" y1="6" x2="18" y2="18" />
                  </svg>
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </section>

      <section class="panel panel-text">
        <header class="panel-head">
          <h3>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="4" y1="6" x2="20" y2="6" />
              <line x1="4" y1="12" x2="20" y2="12" />
              <line x1="4" y1="18" x2="12" y2="18" />
            </svg>
            Text History
          </h3>
          <button class="clear-btn" on:click={clearText}>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6" />
              <path d="M8 6v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
            </svg>
            Clear All
          </button>
        </header>

        <div class="category-row">
          <div class="category-scroll" on:wheel|preventDefault={handleCategoryWheel}>
            {#each categories as cat}
              {#if isRenamingCat && targetCategory === cat}
                <input
                  class="cat-editor"
                  bind:value={renameCatName}
                  on:keydown={(e) => {
                    if (e.key === 'Enter') confirmRename();
                    else if (e.key === 'Escape') isRenamingCat = false;
                  }}
                  on:blur={confirmRename}
                />
              {:else}
                <button class="chip {activeCategory === cat ? 'active' : ''}" on:click={() => (activeCategory = cat)} on:contextmenu|preventDefault={(e) => handleContextMenu(e, cat)}>
                  {cat}
                </button>
              {/if}
            {/each}

            {#if isAddingCat}
              <input
                class="cat-editor"
                bind:value={newCatName}
                on:keydown={(e) => {
                  if (e.key === 'Enter') addCategory();
                  else if (e.key === 'Escape') isAddingCat = false;
                }}
                on:blur={addCategory}
                placeholder="新分类..."
              />
            {:else}
              <button class="chip add-chip" on:click={() => (isAddingCat = true)} aria-label="Add category">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="5" x2="12" y2="19" />
                  <line x1="5" y1="12" x2="19" y2="12" />
                </svg>
              </button>
            {/if}
          </div>
        </div>

        <div class="scroll-v list-stack">
          {#if displayedText.length === 0}
            <div class="empty-list">No text records in this category.</div>
          {/if}

          {#each displayedText as item (item.id)}
            <div
              class="text-item card-pop"
              role="button"
              tabindex="0"
              on:click={() => copyText(item.content)}
              on:keydown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  copyText(item.content);
                }
              }}
              in:fly={{ x: 14, duration: 260 }}
            >
              <div class="text-main">
                <div class="text-title" title={item.content}>{item.content}</div>
                <div class="text-meta">
                  <select class="cat-select" value={item.category || DEFAULT_CATEGORY} on:change={(e) => changeCategory(item.id, e.target.value)} on:click|stopPropagation>
                    {#each categories as cat}
                      <option value={cat}>{cat}</option>
                    {/each}
                  </select>
                </div>
              </div>
              <div class="text-actions">
                <button class="icon-btn" on:click|stopPropagation={() => copyText(item.content)} aria-label="Copy text">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
                  </svg>
                </button>
                <button class="icon-btn danger" on:click|stopPropagation={() => deleteItem(item.id)} aria-label="Delete text">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18" />
                    <line x1="6" y1="6" x2="18" y2="18" />
                  </svg>
                </button>
              </div>
            </div>
          {/each}
        </div>
      </section>
    </div>
  {:else}
    <div class="workspace password-workspace" in:fade={{ duration: 170 }}>
      <section class="panel panel-form">
        <header class="panel-head single">
          <h3>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="11" width="18" height="10" rx="2" />
              <path d="M7 11V7a5 5 0 0 1 10 0v4" />
            </svg>
            Add New
          </h3>
        </header>

        <div class="form-body">
          <label for="pwd-title">For what</label>
          <div class="input-shell">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <polyline points="14 2 14 8 20 8" />
              <line x1="16" y1="13" x2="8" y2="13" />
            </svg>
            <input id="pwd-title" bind:value={newPwdTitle} placeholder="Enter purpose or website" />
          </div>

          <label for="pwd-username">Username</label>
          <div class="input-shell">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
              <circle cx="12" cy="7" r="4" />
            </svg>
            <input id="pwd-username" bind:value={newPwdUser} placeholder="Enter username or email" />
          </div>

          <label for="pwd-password">Password</label>
          <div class="input-shell">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 12V8a4 4 0 0 0-8 0v4" />
              <rect x="4" y="12" width="16" height="9" rx="2" />
            </svg>
            <input id="pwd-password" bind:value={newPwdPass} placeholder="Enter password" type={showNewPwd ? 'text' : 'password'} />
            <button class="inline-btn" on:click={() => (showNewPwd = !showNewPwd)} aria-label="Toggle password">
              {#if showNewPwd}
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M17.94 17.94A10.94 10.94 0 0 1 12 20C5 20 1 12 1 12a18.4 18.4 0 0 1 5-6" />
                  <path d="M9.9 4.2A9.1 9.1 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.2 3.2" />
                  <line x1="1" y1="1" x2="23" y2="23" />
                </svg>
              {:else}
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
                  <circle cx="12" cy="12" r="3" />
                </svg>
              {/if}
            </button>
          </div>

          <button class="save-btn" on:click={addPassword}>Save</button>
        </div>
      </section>

      <section class="panel panel-passwords">
        <header class="panel-head">
          <h3>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M15 7h3a5 5 0 0 1 0 10h-3" />
              <path d="M9 17H6a5 5 0 1 1 0-10h3" />
              <line x1="8" y1="12" x2="16" y2="12" />
            </svg>
            My Passwords
          </h3>
          <button class="clear-btn" on:click={clearAllPasswords}>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="3 6 5 6 21 6" />
              <path d="M8 6v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
            </svg>
            Clear All
          </button>
        </header>

        <div class="scroll-v list-stack">
          {#if filteredPasswords.length === 0}
            <div class="empty-list">No password items yet.</div>
          {/if}

          {#each filteredPasswords as item (item.id)}
            <div class="password-item card-pop" in:fly={{ x: 14, duration: 260 }}>
              <div class="password-head">
                <div class="password-title" title={item.title}>{item.title}</div>
                <button class="icon-btn danger" on:click={() => deletePassword(item.id)} aria-label="Delete password">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18" />
                    <line x1="6" y1="6" x2="18" y2="18" />
                  </svg>
                </button>
              </div>

              {#if item.username}
                <div class="password-line">
                  <span class="line-label">User:</span>
                  <span class="line-value">{item.username}</span>
                </div>
              {/if}

              <div class="password-line">
                <span class="line-label">Pass:</span>
                <span class="line-value">{item.showPass ? item.password : '••••'}</span>
              </div>

              <div class="password-actions">
                <button class="icon-btn" on:click={() => togglePassword(item.id)} aria-label="Toggle password">
                  {#if item.showPass}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M17.94 17.94A10.94 10.94 0 0 1 12 20C5 20 1 12 1 12a18.4 18.4 0 0 1 5-6" />
                      <path d="M9.9 4.2A9.1 9.1 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.2 3.2" />
                      <line x1="1" y1="1" x2="23" y2="23" />
                    </svg>
                  {:else}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
                      <circle cx="12" cy="12" r="3" />
                    </svg>
                  {/if}
                </button>
                {#if item.username}
                  <button class="icon-btn" on:click={() => copyText(item.username)} aria-label="Copy username">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
                      <circle cx="12" cy="7" r="4" />
                    </svg>
                  </button>
                {/if}
                <button class="icon-btn" on:click={() => copyText(item.password)} aria-label="Copy password">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
                  </svg>
                </button>
                <button class="icon-btn danger" on:click={() => deletePassword(item.id)} aria-label="Delete password">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="3 6 5 6 21 6" />
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
                  </svg>
                </button>
              </div>
            </div>
          {/each}
        </div>
      </section>
    </div>
  {/if}

  {#if showToast}
    <div class="toast-tip" transition:scale={{ duration: 120 }}>{toastMsg}</div>
  {/if}

  {#if showContextMenu}
    <div
      class="context-menu"
      style="left: {menuX}px; top: {menuY}px;"
      role="menu"
      tabindex="-1"
      aria-label="Category menu"
      in:fade={{ duration: 100 }}
      on:mousedown|stopPropagation
    >
      <button class="menu-item" on:click={startRename}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 20h9" />
          <path d="M16.5 3.5a2.1 2.1 0 0 1 3 3L7 19l-4 1 1-4Z" />
        </svg>
        重命名
      </button>
      <div class="menu-divider"></div>
      <button class="menu-item danger" on:click={deleteCategoryFromMenu}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="3 6 5 6 21 6" />
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" />
          <path d="M8 6v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
        </svg>
        删除分类
      </button>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    height: 100vh;
    overflow: hidden;
    font-family: 'Segoe UI', 'PingFang SC', 'Microsoft YaHei', sans-serif;
    background: linear-gradient(180deg, #f8fbff 0%, #f4f8ff 100%);
    color: #16233f;
  }

  :global(*) { box-sizing: border-box; }
  :global(button), :global(input), :global(select) { font-family: inherit; }

  .app-shell {
    width: 100%;
    height: 100%;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    animation: shell-reveal 0.34s ease-out;
  }

  .top-tabs {
    position: relative;
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 4px 4px 0;
    border-bottom: 1px solid #dce6f4;
    min-height: 38px;
  }

  .tab-btn {
    border: none;
    background: transparent;
    color: #5b6d8c;
    font-size: clamp(17px, 1.7vw, 20px);
    padding: 8px 4px 10px;
    line-height: 1;
    cursor: pointer;
    position: relative;
    transition: color 0.22s ease;
    font-weight: 500;
    z-index: 1;
  }

  .tab-indicator {
    position: absolute;
    left: 0;
    bottom: -1px;
    height: 3px;
    border-radius: 999px;
    background: #2f84e6;
    transition: transform 0.28s cubic-bezier(0.22, 1, 0.36, 1), width 0.28s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .tab-btn.active { color: #14233f; font-weight: 700; }

  .search-wrap { padding: 0 4px; }

  .search-bar {
    display: flex;
    align-items: center;
    gap: 10px;
    border: 1px solid #d8e3f2;
    background: #ffffff;
    border-radius: 12px;
    padding: 9px 11px;
    box-shadow: 0 6px 16px rgba(120, 149, 191, 0.08);
    transition: box-shadow 0.25s ease, border-color 0.25s ease, transform 0.25s ease;
  }

  .search-bar:focus-within {
    border-color: #8abcf8;
    box-shadow: 0 12px 24px rgba(82, 136, 213, 0.16);
    transform: translateY(-1px);
  }

  .search-icon { width: 20px; height: 20px; color: #7b8baa; flex-shrink: 0; }

  .search-bar input {
    border: none;
    outline: none;
    background: transparent;
    color: #2b3a56;
    font-size: 14px;
    flex: 1;
  }

  .search-bar input::placeholder { color: #8a99b7; }

  kbd {
    border: 1px solid #dae4f2;
    background: #f6f9ff;
    border-radius: 7px;
    color: #8a97ae;
    font-size: 12px;
    padding: 4px 8px;
    min-width: 48px;
    text-align: center;
    flex-shrink: 0;
  }

  .workspace {
    flex: 1;
    min-height: 0;
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1.08fr);
    gap: 10px;
  }

  .panel {
    border: 1px solid #d8e3f2;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 8px 22px rgba(104, 136, 184, 0.08);
    padding: 12px;
    min-height: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: panel-in 0.36s ease;
  }

  .panel-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
    gap: 10px;
  }

  .panel-head h3 {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 0;
    font-size: clamp(17px, 1.8vw, 22px);
    line-height: 1.1;
    color: #1a2746;
    font-weight: 700;
  }

  .panel-head h3 svg { width: 20px; height: 20px; color: #21314f; opacity: 0.92; }

  .clear-btn {
    border: 1px solid #bfe0ff;
    background: #f2f8ff;
    color: #2d81e4;
    border-radius: 8px;
    padding: 6px 10px;
    font-size: 13px;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    transition: all 0.22s ease;
    white-space: nowrap;
  }

  .clear-btn svg { width: 14px; height: 14px; }
  .clear-btn:hover { background: #e8f3ff; transform: translateY(-1px); box-shadow: 0 8px 18px rgba(72,136,213,.16); }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    text-align: center;
    gap: 8px;
    color: #8b9ab5;
    padding-bottom: 16px;
  }

  .empty-illustration {
    width: 128px;
    height: 128px;
    border-radius: 50%;
    background: radial-gradient(circle at 30% 30%, #e8f2ff, #f7fbff 68%);
    display: grid;
    place-items: center;
    position: relative;
    animation: float-y 4.6s ease-in-out infinite;
  }

  .empty-illustration::before, .empty-illustration::after {
    content: '';
    position: absolute;
    width: 14px;
    height: 14px;
    background: radial-gradient(circle, #b2d2fb 5%, #e7f2ff 72%);
    border-radius: 50%;
    animation: twinkle 2.8s ease-in-out infinite;
  }

  .empty-illustration::before { top: 24px; right: 20px; }
  .empty-illustration::after { bottom: 20px; left: 16px; animation-delay: .8s; }
  .empty-illustration svg { width: 72px; height: 72px; filter: drop-shadow(0 10px 14px rgba(88,144,216,.2)); }

  .empty-title { margin: 6px 0 0; font-size: 20px; color: #1d2946; font-weight: 700; line-height: 1.15; }
  .empty-sub { margin: 0; max-width: 320px; font-size: 14px; color: #7f8da8; line-height: 1.45; }

  .scroll-v {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding-right: 6px;
  }

  .scroll-v::-webkit-scrollbar { width: 6px; }
  .scroll-v::-webkit-scrollbar-thumb { background: #c6d8f1; border-radius: 999px; }

  .image-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(96px, 1fr)); gap: 10px; }

  .image-card {
    position: relative;
    aspect-ratio: 1;
    border: 1px solid #d6e4f4;
    border-radius: 12px;
    overflow: hidden;
    padding: 0;
    cursor: pointer;
    background: #f6faff;
    transition: transform .22s ease, box-shadow .22s ease, border-color .22s ease;
  }

  .image-card img { width: 100%; height: 100%; object-fit: cover; display: block; }
  .image-card-overlay {
    position: absolute;
    inset: auto 0 0 0;
    padding: 7px 10px;
    color: #e7f3ff;
    font-size: 14px;
    text-align: center;
    background: linear-gradient(180deg, transparent, rgba(21, 42, 75, 0.72));
    opacity: 0;
    transform: translateY(12px);
    transition: all .24s ease;
  }
  .image-card:hover .image-card-overlay { opacity: 1; transform: translateY(0); }
  .image-card:hover { transform: translateY(-2px); border-color: #93bff4; box-shadow: 0 14px 22px rgba(80, 128, 196, .22); }
  .image-del { position: absolute; top: 6px; right: 6px; background: rgba(255, 255, 255, .92); }

  .category-row { margin-bottom: 10px; }
  .category-scroll { display: flex; align-items: center; gap: 10px; overflow-x: auto; overflow-y: hidden; padding-bottom: 2px; scroll-behavior: smooth; }
  .category-scroll::-webkit-scrollbar { height: 4px; }
  .category-scroll::-webkit-scrollbar-thumb { background: #d4e3f5; border-radius: 999px; }

  .chip {
    border: 1px solid #e2eaf6;
    border-radius: 999px;
    background: #f3f7fc;
    color: #677897;
    font-size: 13px;
    padding: 6px 12px;
    white-space: nowrap;
    cursor: pointer;
    transition: all .2s ease;
    flex: 0 0 auto;
  }
  .chip:hover { transform: translateY(-1px); background: #edf4fe; color: #365986; }
  .chip.active { border-color: transparent; color: #fff; background: linear-gradient(135deg, #2d83e6, #2d78d9); box-shadow: 0 8px 16px rgba(62,128,213,.3); }
  .add-chip { width: 32px; height: 32px; min-width: 32px; min-height: 32px; padding: 0; border-radius: 50%; display: grid; place-items: center; flex: 0 0 auto; }
  .add-chip svg { width: 14px; height: 14px; }

  .cat-editor {
    width: 96px;
    min-width: 96px;
    border: 1px solid #87b9f5;
    border-radius: 999px;
    padding: 6px 10px;
    font-size: 13px;
    background: #f8fbff;
    outline: none;
    color: #21324f;
    flex: 0 0 auto;
  }

  .list-stack { display: flex; flex-direction: column; gap: 10px; }
  .empty-list { border: 1px dashed #d4e2f3; border-radius: 12px; color: #8393ae; padding: 16px; text-align: center; background: #f9fbff; font-size: 13px; }

  .card-pop { transition: transform .22s ease, box-shadow .22s ease, border-color .22s ease; }
  .card-pop:hover { transform: translateY(-2px); box-shadow: 0 10px 22px rgba(82,130,199,.16); }

  .text-item { border: 1px solid #d8e4f4; border-radius: 12px; padding: 11px 10px; background: #fff; display: flex; align-items: center; gap: 8px; cursor: pointer; }
  .text-main { flex: 1; min-width: 0; }
  .text-title { font-size: 15px; color: #1b2a48; font-weight: 700; line-height: 1.35; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .text-meta { margin-top: 8px; }
  .cat-select { border: 1px solid #cfe3fc; border-radius: 8px; background: #f2f8ff; color: #2f7dde; font-size: 13px; padding: 5px 24px 5px 10px; outline: none; cursor: pointer; max-width: 132px; }
  .text-actions { display: flex; align-items: center; gap: 6px; flex-shrink: 0; }

  .icon-btn {
    width: 28px;
    height: 28px;
    border-radius: 7px;
    border: 1px solid transparent;
    background: transparent;
    color: #6d7d9a;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    cursor: pointer;
    transition: all .2s ease;
  }
  .icon-btn svg { width: 16px; height: 16px; }
  .icon-btn:hover { color: #2a4f80; background: #eef4fd; border-color: #d6e4f6; }
  .icon-btn.danger:hover { color: #d84f5f; background: #fff1f3; border-color: #ffd5dc; }

  .panel-form { max-width: 440px; }
  .form-body { display: flex; flex-direction: column; gap: 8px; margin-top: 2px; flex: 1; min-height: 0; overflow-y: auto; padding-right: 2px; }
  .form-body label { color: #3e4d68; font-size: 13px; margin-top: 2px; }

  .input-shell {
    display: flex;
    align-items: center;
    gap: 8px;
    border: 1px solid #d6e4f5;
    border-radius: 10px;
    background: #fff;
    padding: 8px 10px;
    transition: all .22s ease;
  }
  .input-shell:focus-within { border-color: #8fbef7; box-shadow: 0 8px 18px rgba(72,136,213,.16); transform: translateY(-1px); }
  .input-shell svg { width: 18px; height: 18px; color: #6f809f; flex-shrink: 0; }
  .input-shell input { border: none; outline: none; background: transparent; color: #293957; font-size: 14px; flex: 1; }
  .input-shell input::placeholder { color: #90a0ba; }

  .inline-btn { width: 26px; height: 26px; border: none; background: transparent; color: #6d7e9e; border-radius: 8px; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; transition: all .2s ease; }
  .inline-btn svg { width: 16px; height: 16px; }
  .inline-btn:hover { color: #2c4f80; background: #eef4fd; }

  .save-btn { margin-top: 12px; border: none; border-radius: 10px; padding: 10px 12px; color: #fff; font-weight: 700; font-size: 15px; background: linear-gradient(120deg, #2f87ea, #286ed8); cursor: pointer; transition: transform .22s ease, box-shadow .22s ease, filter .22s ease; }
  .save-btn:hover { transform: translateY(-1px); filter: brightness(1.03); box-shadow: 0 12px 20px rgba(53,122,211,.28); }

  .password-item { border: 1px solid #d8e4f4; border-radius: 12px; background: #fff; padding: 11px 12px; }
  .password-head { display: flex; align-items: center; justify-content: space-between; gap: 10px; }
  .password-title { font-size: 16px; color: #1a2b49; font-weight: 700; line-height: 1.25; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .password-line { margin-top: 8px; display: flex; align-items: baseline; gap: 8px; }
  .line-label { color: #7a8ba8; min-width: 42px; font-size: 13px; }
  .line-value { color: #2e3d5b; font-size: 14px; line-height: 1.35; word-break: break-all; }
  .password-actions { margin-top: 10px; display: flex; justify-content: flex-end; gap: 6px; }

  .toast-tip { position: fixed; left: 50%; bottom: 18px; transform: translateX(-50%); background: #1f2f4d; color: #fff; font-size: 12px; padding: 7px 12px; border-radius: 999px; box-shadow: 0 12px 20px rgba(20,33,58,.28); z-index: 9999; }

  .context-menu { position: fixed; min-width: 140px; border: 1px solid #d5e3f5; background: #fff; border-radius: 10px; box-shadow: 0 14px 26px rgba(80,118,173,.24); padding: 6px; z-index: 10000; }
  .menu-item { width: 100%; border: none; background: transparent; border-radius: 8px; display: flex; align-items: center; gap: 8px; padding: 8px 10px; color: #4d607f; cursor: pointer; transition: all .2s ease; font-size: 14px; }
  .menu-item svg { width: 16px; height: 16px; }
  .menu-item:hover { background: #f0f6ff; color: #20426f; }
  .menu-item.danger { color: #d05567; }
  .menu-item.danger:hover { background: #fff1f3; color: #cc4256; }
  .menu-divider { height: 1px; background: #e1eaf6; margin: 4px 0; }

  input::-ms-reveal,
  input::-ms-clear,
  input::-webkit-credentials-auto-fill-button { display: none !important; }

  @keyframes shell-reveal { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }
  @keyframes panel-in { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }
  @keyframes float-y { 0%,100% { transform: translateY(0); } 50% { transform: translateY(-8px); } }
  @keyframes twinkle { 0%,100% { opacity: .4; transform: scale(1); } 50% { opacity: 1; transform: scale(1.2); } }

  @media (max-width: 760px), (max-height: 700px) {
    .app-shell {
      padding: 10px;
      gap: 8px;
    }
    .top-tabs {
      gap: 10px;
      padding-top: 2px;
    }
    .tab-btn {
      font-size: 16px;
      padding: 7px 2px 9px;
    }
    .panel {
      padding: 10px;
      border-radius: 10px;
    }
    .panel-head {
      margin-bottom: 8px;
    }
    .panel-head h3 {
      font-size: 16px;
      gap: 7px;
    }
    .panel-head h3 svg {
      width: 16px;
      height: 16px;
    }
    .empty-illustration {
      width: 98px;
      height: 98px;
    }
    .empty-illustration svg {
      width: 54px;
      height: 54px;
    }
    .empty-title {
      font-size: 16px;
    }
    .empty-sub {
      font-size: 12px;
      max-width: 240px;
    }
    .search-bar input {
      font-size: 13px;
    }
    kbd {
      font-size: 11px;
      min-width: 42px;
      padding: 3px 6px;
    }
    .workspace,
    .password-workspace { gap: 8px; }
    .text-title {
      font-size: 14px;
    }
    .password-title {
      font-size: 15px;
    }
    .line-value {
      font-size: 13px;
    }
  }
</style>
