<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { fade, fly } from 'svelte/transition';

  let activeTab = 'clipboard'; 
  let showToast = false;
  let toastMsg = "Copied!";

  let searchQuery = "";
  let history = []; 
  let lastData = "";

  let pwdSearchQuery = "";
  let passwords = [];
  let newPwdTitle = "";
  let newPwdUser = "";
  let newPwdPass = "";
  
  let showNewPwd = false;

  let isAppCopying = false; 

  let categories = ['全部']; 
  let activeCategory = '全部';
  let isAddingCat = false;
  let newCatName = "";

  let showContextMenu = false;
  let menuX = 0;
  let menuY = 0;
  let targetCategory = '';
  
  let isRenamingCat = false;
  let renameCatName = "";

  function saveHistory() {
    localStorage.setItem('clip_v5_split', JSON.stringify(history));
    localStorage.setItem('clip_v5_last_data', lastData);
  }

  function savePasswords() {
    localStorage.setItem('clip_v5_passwords', JSON.stringify(passwords));
  }

  function saveCategories() {
    localStorage.setItem('clip_v5_categories', JSON.stringify(categories));
  }

  onMount(() => {
    const saved = localStorage.getItem('clip_v5_split');
    if (saved) { try { history = JSON.parse(saved); } catch (e) { history = []; } }
    const savedLastData = localStorage.getItem('clip_v5_last_data');
    if (savedLastData) lastData = savedLastData;

    const savedPwds = localStorage.getItem('clip_v5_passwords');
    if (savedPwds) { try { passwords = JSON.parse(savedPwds); } catch (e) { passwords = []; } }

    const savedCats = localStorage.getItem('clip_v5_categories');
    if (savedCats) { try { categories = JSON.parse(savedCats); } catch (e) { categories = ['全部']; } }

    const clipboardInterval = setInterval(async () => {
      try {
        const data = await invoke('get_clipboard_data');
        if (data) {
          const [type, content] = data;
          if (content !== lastData) {
            
            if (isAppCopying && type === 'image') {
              lastData = content; 
              isAppCopying = false; 
              saveHistory();
              return; 
            }

            const defaultCat = (type === 'text' && activeCategory !== '全部') ? activeCategory : '全部';
            const newItem = { type, content, id: Date.now(), timestamp: new Date().toLocaleString(), category: defaultCat };
            
            const images = history.filter(i => i.type !== 'text');
            const texts = history.filter(i => i.type === 'text');
            if (type === 'image') {
              images.unshift(newItem);
            } else {
              texts.unshift(newItem);
            }
            history = [...images.slice(0, 10), ...texts.slice(0, 50)];
            
            lastData = content;
            saveHistory();
          }
        }
      } catch (error) { }
    }, 2000); 

    const handlePaste = async (e) => {
      if (document.activeElement.tagName === 'INPUT') return;

      const items = e.clipboardData.items;
      for (let i = 0; i < items.length; i++) {
        if (items[i].type.indexOf('image') !== -1) {
          const blob = items[i].getAsFile();
          const reader = new FileReader();
          reader.onload = (event) => {
            const base64 = event.target.result.split(',')[1];
            const content = `image|${base64}`;
            if (content !== lastData) {
              const newItem = { type: 'image', content, id: Date.now(), timestamp: new Date().toLocaleString() };
              const images = history.filter(item => item.type !== 'text');
              const texts = history.filter(item => item.type === 'text');
              images.unshift(newItem);
              history = [...images.slice(0, 10), ...texts.slice(0, 50)];
              lastData = content;
              saveHistory();
              triggerToast("Image Pasted!");
            }
          };
          reader.readAsDataURL(blob);
        } else if (items[i].type === 'text/plain') {
          items[i].getAsString((text) => {
            if (text !== lastData) {
              const defaultCat = activeCategory !== '全部' ? activeCategory : '全部';
              const newItem = { type: 'text', content: text, id: Date.now(), timestamp: new Date().toLocaleString(), category: defaultCat };
              const images = history.filter(item => item.type !== 'text');
              const texts = history.filter(item => item.type === 'text');
              texts.unshift(newItem);
              history = [...images.slice(0, 10), ...texts.slice(0, 50)];
              lastData = text;
              saveHistory();
              triggerToast("Text Pasted!");
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

  function triggerToast(msg = "Copied!") {
    toastMsg = msg;
    showToast = true;
    setTimeout(() => { showToast = false; }, 800);
  }

  async function copyText(text) {
    if (!text) return;
    await invoke('set_clipboard_text', { text });
    lastData = text;
    saveHistory();
    triggerToast();
  }

  async function copyImage(content) {
    if (!content) return;
    try {
      isAppCopying = true; 
      const base64 = content.includes('|') ? content.split('|')[1] : content;
      await invoke('set_clipboard_image', { base64 });
      triggerToast("Image Copied!");
      setTimeout(() => { isAppCopying = false; }, 3000);
    } catch (e) {
      triggerToast("Failed to copy");
    }
  }

  async function deleteItem(id) {
    const item = history.find(i => i.id === id);
    history = history.filter(i => i.id !== id);
    saveHistory();
    if (item && item.content === lastData) {
      await invoke('set_clipboard_text', { text: "" });
      lastData = "";
      localStorage.setItem('clip_v5_last_data', "");
    }
  }

  async function clearImages() {
    const hasLastData = history.some(i => i.type !== 'text' && i.content === lastData);
    history = history.filter(item => item.type === 'text');
    saveHistory();
    if (hasLastData) {
      await invoke('set_clipboard_text', { text: "" });
      lastData = "";
      localStorage.setItem('clip_v5_last_data', "");
    }
  }

  async function clearText() {
    const toDelete = history.filter(item => item.type === 'text' && (activeCategory === '全部' || (item.category || '全部') === activeCategory));
    const hasLastData = toDelete.some(i => i.content === lastData);
    
    history = history.filter(item => !toDelete.includes(item));
    saveHistory();
    
    if (hasLastData) {
      await invoke('set_clipboard_text', { text: "" });
      lastData = "";
      localStorage.setItem('clip_v5_last_data', "");
    }
  }
  
  function addCategory() {
    if (newCatName && !categories.includes(newCatName)) {
      categories = [...categories, newCatName];
      saveCategories();
    }
    isAddingCat = false;
    newCatName = "";
  }

  function changeCategory(id, newCat) {
    history = history.map(item => item.id === id ? { ...item, category: newCat } : item);
    saveHistory();
  }

  function handleContextMenu(e, cat) {
    if (cat === '全部') return;
    targetCategory = cat;
    menuX = e.clientX;
    menuY = e.clientY;
    showContextMenu = true;
  }

  function closeContextMenu() { showContextMenu = false; }

  function startRename() {
    renameCatName = targetCategory;
    isRenamingCat = true;
    closeContextMenu();
  }

  function confirmRename() {
    if (renameCatName && renameCatName !== targetCategory && !categories.includes(renameCatName)) {
      categories = categories.map(c => c === targetCategory ? renameCatName : c);
      history = history.map(item => item.category === targetCategory ? { ...item, category: renameCatName } : item);
      if (activeCategory === targetCategory) activeCategory = renameCatName;
      saveCategories();
      saveHistory();
    }
    isRenamingCat = false;
  }

  function deleteCategoryFromMenu() {
    const cat = targetCategory;
    categories = categories.filter(c => c !== cat);
    history = history.map(item => item.category === cat ? { ...item, category: '全部' } : item);
    saveHistory();
    saveCategories();
    if (activeCategory === cat) activeCategory = '全部';
    closeContextMenu();
  }

  function addPassword() {
    if (!newPwdTitle || !newPwdPass) return;
    const newItem = { id: Date.now(), title: newPwdTitle, username: newPwdUser, password: newPwdPass, showPass: false };
    passwords = [newItem, ...passwords];
    savePasswords();
    newPwdTitle = ""; newPwdUser = ""; newPwdPass = "";
    showNewPwd = false; 
    triggerToast();
  }

  function deletePassword(id) { passwords = passwords.filter(p => p.id !== id); savePasswords(); }
  function clearAllPasswords() { passwords = []; savePasswords(); }
  function togglePassword(id) { passwords = passwords.map(p => p.id === id ? { ...p, showPass: !p.showPass } : p); }

  function getImgSrc(content) { try { return `data:image/png;base64,${content.split('|')[1]}`; } catch (e) { return ''; } }

  $: filteredImages = history.filter(item => item.type !== 'text');
  $: displayedText = history
      .filter(item => item.type === 'text')
      .filter(item => activeCategory === '全部' || (item.category || '全部') === activeCategory)
      .filter(item => !searchQuery || item.content.toLowerCase().includes(searchQuery.toLowerCase()));
      
  $: filteredPasswords = pwdSearchQuery ? passwords.filter(p => p.title.toLowerCase().includes(pwdSearchQuery.toLowerCase())) : passwords;
</script>

<svelte:window on:click={closeContextMenu} />

<main class="app-container">
  <div class="header-nav">
    <div class="tabs">
      <button class={activeTab === 'clipboard' ? 'active' : ''} on:click={() => activeTab = 'clipboard'}>Clipboard</button>
      <button class={activeTab === 'passwords' ? 'active' : ''} on:click={() => activeTab = 'passwords'}>Password Book</button>
    </div>
  </div>

  <div class="search-section">
    <div class="search-box">
      <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
      {#if activeTab === 'clipboard'}
        <input type="text" placeholder="Search stored clipboard..." bind:value={searchQuery} />
      {:else}
        <input type="text" placeholder="Search stored passwords..." bind:value={pwdSearchQuery} />
      {/if}
    </div>
  </div>

  <div class="content-area">
    {#if activeTab === 'clipboard'}
      <div class="clipboard-layout" in:fade>
        <div class="left-pane">
          <div class="pane-header">
            <div class="pane-title">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>
              Image History
            </div>
            <button class="btn-secondary" on:click={clearImages}>Clear All</button>
          </div>
          <div class="scroll-v image-grid">
            {#each filteredImages as item (item.id)}
              <div class="image-card" on:click={() => copyImage(item.content)} on:keydown={(e) => e.key === 'Enter' && copyImage(item.content)} tabindex="0" role="button" in:fly={{ y: 10 }}>
                <img src={getImgSrc(item.content)} alt="Clipboard snapshot" />
                <button class="img-del" on:click|stopPropagation={() => deleteItem(item.id)} aria-label="Delete image">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                </button>
              </div>
            {/each}
          </div>
        </div>

        <div class="divider"></div>

        <div class="right-pane">
          <div class="pane-header">
            <div class="pane-title">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="4" y1="6" x2="20" y2="6"/><line x1="4" y1="12" x2="20" y2="12"/><line x1="4" y1="18" x2="12" y2="18"/></svg>
              Text History
            </div>
            <button class="btn-secondary" on:click={clearText}>Clear All</button>
          </div>

          <div class="category-bar">
            <div class="category-scroll">
              {#each categories as cat}
                {#if isRenamingCat && targetCategory === cat}
                  <input 
                    class="cat-input" 
                    bind:value={renameCatName} 
                    on:keydown={(e) => { if(e.key === 'Enter') confirmRename(); else if(e.key === 'Escape') isRenamingCat = false; }}
                    on:blur={confirmRename}
                    autofocus />
                {:else}
                  <div 
                    class="cat-tab {activeCategory === cat ? 'active' : ''}" 
                    on:click={() => activeCategory = cat}
                    on:keydown={(e) => e.key === 'Enter' && (activeCategory = cat)}
                    on:contextmenu|preventDefault={(e) => handleContextMenu(e, cat)}
                    tabindex="0"
                    role="tab">
                    {cat}
                  </div>
                {/if}
              {/each}
              
              {#if isAddingCat}
                <input 
                  class="cat-input" 
                  bind:value={newCatName} 
                  on:keydown={(e) => { if(e.key === 'Enter') addCategory(); else if(e.key === 'Escape') isAddingCat = false; }}
                  on:blur={addCategory}
                  placeholder="新分类..."
                  autofocus />
              {:else}
                <button class="add-cat-btn" on:click={() => isAddingCat = true} aria-label="Add category">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                </button>
              {/if}
            </div>
          </div>

          <div class="scroll-v list-container" style="padding-top: 5px;">
            {#each displayedText as item (item.id)}
              <div class="data-card pwd-card" on:click={() => copyText(item.content)} on:keydown={(e) => e.key === 'Enter' && copyText(item.content)} tabindex="0" role="button" in:fly={{ x: 10 }}>
                <div class="pwd-info text-info">
                  <div class="pwd-title text-ellipsis">{item.content}</div>
                  <div class="pwd-detail">
                    <select class="cat-select" value={item.category || '全部'} on:change={(e) => changeCategory(item.id, e.target.value)} on:click|stopPropagation>
                      {#each categories as cat}
                        <option value={cat}>{cat}</option>
                      {/each}
                    </select>
                  </div>
                </div>
                <div class="pwd-actions">
                  <button class="action-icon" on:click|stopPropagation={() => copyText(item.content)} aria-label="Copy text">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
                  </button>
                  <button class="action-icon" on:click|stopPropagation={() => deleteItem(item.id)} aria-label="Delete text">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {:else}
      <div class="password-layout" in:fade>
        <div class="left-pane">
          <div class="pane-title">
            <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
            Add New
          </div>
          <div class="input-group">
            <div class="input-wrapper">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>
              <input bind:value={newPwdTitle} placeholder="For what" />
            </div>
            <div class="input-wrapper">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
              <input bind:value={newPwdUser} placeholder="Username" />
            </div>
            <div class="input-wrapper">
              <button class="icon-btn" on:click={() => showNewPwd = !showNewPwd} aria-label={showNewPwd ? "Hide password" : "Show password"}>
                {#if showNewPwd}
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                {:else}
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
                {/if}
              </button>
              <input bind:value={newPwdPass} placeholder="Password" type={showNewPwd ? 'text' : 'password'} />
            </div>
            <button class="btn-primary" on:click={addPassword}>Save</button>
          </div>
        </div>

        <div class="divider"></div>

        <div class="right-pane">
          <div class="pane-header">
            <div class="pane-title">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/></svg>
              My Passwords
            </div>
            <button class="btn-secondary" on:click={clearAllPasswords}>Clear All</button>
          </div>
          <div class="scroll-v list-container">
            {#each filteredPasswords as item (item.id)}
              <div class="data-card pass-card">
                
                <div class="pass-row pass-header">
                  <div class="pass-title">{item.title}</div>
                  <button class="action-icon danger-icon" on:click={() => deletePassword(item.id)} aria-label="Delete password">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                  </button>
                </div>
                
                {#if item.username}
                  <div class="pass-row">
                    <div class="pass-detail">
                      <span class="detail-label">User:</span>
                      <span class="detail-value">{item.username}</span>
                    </div>
                    <button class="action-icon" on:click={() => copyText(item.username)} aria-label="Copy username">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                    </button>
                  </div>
                {/if}

                <div class="pass-row">
                  <div class="pass-detail">
                    <span class="detail-label">Pass:</span>
                    <span class="detail-value">{item.showPass ? item.password : '****'}</span>
                  </div>
                  <div class="pass-actions">
                    <button class="action-icon" on:click={() => togglePassword(item.id)} aria-label={item.showPass ? "Hide password" : "Show password"}>
                      {#if item.showPass}
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
                      {:else}
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                      {/if}
                    </button>
                    <button class="action-icon" on:click={() => copyText(item.password)} aria-label="Copy password">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
                    </button>
                  </div>
                </div>

              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>

  {#if showToast}
    <div class="toast-tip" transition:fade>{toastMsg}</div>
  {/if}

  {#if showContextMenu}
    <div 
      class="context-menu" 
      style="left: {menuX}px; top: {menuY}px;"
      in:fade={{ duration: 100 }}>
      <button class="menu-item" on:click|stopPropagation={startRename}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
        重命名
      </button>
      <div class="menu-divider"></div>
      <button class="menu-item danger" on:click|stopPropagation={deleteCategoryFromMenu}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
        删除分类
      </button>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    background: #ffffff;
    color: #1a1a1a;
    font-family: 'Inter', 'Segoe UI', system-ui, sans-serif;
    overflow: hidden;
    height: 100vh;
  }

  input::-ms-reveal,
  input::-ms-clear,
  input::-webkit-credentials-auto-fill-button {
    display: none !important;
  }

  .app-container { width: 100%; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; padding: 10px 20px 20px; }
  .header-nav { padding: 0 0 10px 0; border-bottom: 1px solid #f1f5f9; }
  .tabs { display: flex; gap: 20px; }
  .tabs button { background: none; border: none; color: #64748b; padding: 10px 4px; cursor: pointer; font-size: 14px; font-weight: 500; border-bottom: 3px solid transparent; transition: all 0.2s; }
  .tabs button.active { color: #0f172a; border-bottom: 3px solid #7cb3f2; font-weight: 600; }
  
  .search-section { padding: 15px 0; }
  .search-box { display: flex; align-items: center; background: #ffffff; border: 1px solid #e2e8f0; border-radius: 6px; padding: 8px 12px; }
  .search-box .icon { width: 16px; height: 16px; color: #94a3b8; }
  .search-box input { flex: 1; background: none; border: none; outline: none; margin-left: 10px; font-size: 13px; color: #334155; }

  .content-area { flex: 1; overflow: hidden; display: flex; }
  .clipboard-layout, .password-layout { display: flex; width: 100%; gap: 25px; }
  .left-pane { width: 240px; display: flex; flex-direction: column; }
  .right-pane { flex: 1; display: flex; flex-direction: column; min-width: 0; }
  .divider { width: 1px; background: #f1f5f9; }

  .pane-title { font-size: 15px; font-weight: 500; color: #0f172a; margin-bottom: 15px; display: flex; align-items: center; gap: 8px; }
  .pane-title .icon { width: 18px; height: 18px; color: #334155; }
  .pane-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; }
  .pane-header .pane-title { margin-bottom: 0; }

  .category-bar { margin-bottom: 10px; border-bottom: 1px solid #e2e8f0; padding-bottom: 5px; }
  .category-scroll { display: flex; gap: 8px; overflow-x: auto; padding-bottom: 4px; align-items: center; }
  .category-scroll::-webkit-scrollbar { height: 2px; }
  .category-scroll::-webkit-scrollbar-thumb { background: #cbd5e1; border-radius: 2px; }

  .cat-tab { padding: 4px 12px; border-radius: 12px; font-size: 12px; font-weight: 500; background: #f1f5f9; color: #64748b; cursor: context-menu; white-space: nowrap; transition: all 0.2s; user-select: none; }
  .cat-tab:hover { background: #e2e8f0; color: #0f172a; }
  .cat-tab.active { background: #2b74ba; color: white; cursor: default; }

  .add-cat-btn { background: none; border: 1px dashed #cbd5e1; border-radius: 12px; width: 26px; height: 24px; display: flex; align-items: center; justify-content: center; cursor: pointer; color: #94a3b8; flex-shrink: 0; }
  .add-cat-btn:hover { border-color: #2b74ba; color: #2b74ba; }
  .add-cat-btn svg { width: 14px; height: 14px; }
  .cat-input { border: 1px solid #7cb3f2; border-radius: 12px; padding: 2px 8px; font-size: 12px; width: 80px; outline: none; background: #f8fafc; color: #0f172a;}

  .context-menu { position: fixed; background: #ffffff; border: 1px solid #e2e8f0; box-shadow: 0 4px 12px rgba(0,0,0,0.1); border-radius: 8px; padding: 6px; z-index: 9999; display: flex; flex-direction: column; min-width: 120px; }
  .menu-item { background: none; border: none; text-align: left; padding: 8px 12px; font-size: 13px; color: #334155; cursor: pointer; border-radius: 4px; display: flex; align-items: center; gap: 8px; }
  .menu-item:hover { background: #f1f5f9; color: #0f172a; }
  .menu-item svg { width: 14px; height: 14px; }
  .menu-divider { height: 1px; background: #e2e8f0; margin: 4px 0; }
  .menu-item.danger { color: #ef4444; }
  .menu-item.danger:hover { background: #fef2f2; }
  .cat-select { font-size: 11px; padding: 2px 4px; border-radius: 4px; border: 1px solid #e2e8f0; background: #f8fafc; color: #64748b; outline: none; cursor: pointer; }
  
  .input-group { display: flex; flex-direction: column; gap: 12px; }
  .input-wrapper { display: flex; align-items: center; border: 1px solid #e2e8f0; border-radius: 6px; padding: 8px 12px; background: #ffffff; }
  .input-wrapper .icon { width: 16px; height: 16px; color: #64748b; margin-right: 10px; }
  
  .icon-btn { background: none; border: none; padding: 0; margin-right: 10px; color: #64748b; cursor: pointer; display: flex; align-items: center; justify-content: center; }
  .icon-btn:hover { color: #0f172a; }
  .icon-btn svg { width: 16px; height: 16px; }

  .input-wrapper input { flex: 1; border: none; outline: none; font-size: 13px; color: #334155; background: none; }

  .btn-primary { background: #2b74ba; color: white; border: none; padding: 10px; border-radius: 6px; font-size: 14px; font-weight: 500; cursor: pointer; margin-top: 5px; transition: background 0.2s; }
  .btn-primary:hover { background: #1d4ed8; }
  .btn-secondary { background: #e0f2fe; color: #0284c7; border: none; padding: 6px 12px; border-radius: 4px; font-size: 12px; font-weight: 500; cursor: pointer; transition: all 0.2s; }
  .btn-secondary:hover { background: #bae6fd; }

  .scroll-v { flex: 1; overflow-y: auto; padding-right: 8px; }
  .scroll-v::-webkit-scrollbar { width: 4px; }
  .scroll-v::-webkit-scrollbar-thumb { background: #cbd5e1; border-radius: 4px; }
  .list-container { display: flex; flex-direction: column; gap: 10px; }

  .data-card { background: #ffffff; border: 1px solid #7cb3f2; border-radius: 12px; padding: 12px 16px; display: flex; align-items: center; justify-content: space-between; transition: all 0.2s; cursor: pointer; }
  .data-card:hover { background: #f8fafc; }

  /* 密码本卡片排版 */
  .pass-card { display: flex; flex-direction: column; align-items: stretch; gap: 8px; padding: 12px 14px; cursor: default; }
  .pass-row { display: flex; justify-content: space-between; align-items: center; width: 100%; }
  .pass-header { padding-bottom: 8px; border-bottom: 1px solid #f1f5f9; margin-bottom: 2px; }
  .pass-title { font-size: 14px; font-weight: 600; color: #0f172a; }
  .pass-detail { font-size: 13px; color: #334155; display: flex; align-items: center; gap: 6px; }
  .pass-actions { display: flex; gap: 8px; align-items: center; }

  .pwd-card { padding: 10px 16px; cursor: default; }
  .pwd-card:hover { background: #ffffff; }

  .pwd-info { display: flex; flex-direction: column; gap: 4px; flex: 1; min-width: 0; }
  .text-info { margin-right: 10px; }
  .pwd-title { font-size: 13px; font-weight: 500; color: #0f172a; }
  .text-ellipsis { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; width: 100%; }
  
  .detail-label { color: #94a3b8; margin-right: 2px; font-size: 12px; }
  .detail-value { letter-spacing: 0.5px; }

  .pwd-actions { display: flex; gap: 8px; align-items: center; flex-shrink: 0; }
  .action-icon { background: none; border: none; color: #64748b; cursor: pointer; width: 26px; height: 26px; display: flex; align-items: center; justify-content: center; border-radius: 4px; transition: all 0.2s; padding: 0; flex-shrink: 0; }
  .action-icon:hover { color: #0f172a; background: #f1f5f9; }
  
  .action-icon.danger-icon:hover { color: #ef4444; background: #fef2f2; }
  .action-icon svg { width: 16px; height: 16px; }

  .image-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px; }
  .image-card { position: relative; border: 1px solid #7cb3f2; border-radius: 8px; overflow: hidden; aspect-ratio: 1; cursor: pointer; }
  .image-card:hover { border-color: #2b74ba; }
  .image-card img { width: 100%; height: 100%; object-fit: cover; }
  .img-del { position: absolute; top: 4px; right: 4px; background: rgba(255,255,255,0.9); border: none; border-radius: 4px; width: 20px; height: 20px; display: flex; align-items: center; justify-content: center; cursor: pointer; box-shadow: 0 2px 4px rgba(0,0,0,0.1); color: #64748b; }
  .img-del svg { width: 12px; height: 12px; }
  .img-del:hover { background: #fee2e2; color: #ef4444; }

  .toast-tip { position: fixed; top: 30px; left: 50%; transform: translateX(-50%); background: #1e293b; color: white; padding: 10px 24px; border-radius: 30px; font-size: 13px; font-weight: 500; box-shadow: 0 4px 12px rgba(0,0,0,0.15); z-index: 100; }
</style>