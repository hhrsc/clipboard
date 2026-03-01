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

  function saveHistory() {
    localStorage.setItem('clip_v4_split', JSON.stringify(history));
    localStorage.setItem('clip_v4_last_data', lastData);
  }

  function savePasswords() {
    localStorage.setItem('clip_v4_passwords', JSON.stringify(passwords));
  }

  onMount(() => {
    const saved = localStorage.getItem('clip_v4_split');
    if (saved) { try { history = JSON.parse(saved); } catch (e) { history = []; } }
    const savedLastData = localStorage.getItem('clip_v4_last_data');
    if (savedLastData) lastData = savedLastData;

    const savedPwds = localStorage.getItem('clip_v4_passwords');
    if (savedPwds) { try { passwords = JSON.parse(savedPwds); } catch (e) { passwords = []; } }

    const clipboardInterval = setInterval(async () => {
      try {
        const data = await invoke('get_clipboard_data');
        if (data) {
          const [type, content] = data;
          if (content !== lastData) {
            const newItem = { type, content, id: Date.now(), timestamp: new Date().toLocaleString() };
            history = [newItem, ...history].slice(0, 60);
            lastData = content;
            saveHistory();
          }
        }
      } catch (error) { }
    }, 1000);

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
              history = [newItem, ...history].slice(0, 60);
              lastData = content;
              saveHistory();
              triggerToast("Image Pasted!");
            }
          };
          reader.readAsDataURL(blob);
        } else if (items[i].type === 'text/plain') {
          items[i].getAsString((text) => {
            if (text !== lastData) {
              const newItem = { type: 'text', content: text, id: Date.now(), timestamp: new Date().toLocaleString() };
              history = [newItem, ...history].slice(0, 60);
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
      const base64 = content.includes('|') ? content.split('|')[1] : content;
      await invoke('set_clipboard_image', { base64 });
      lastData = content;
      saveHistory();
      triggerToast("Image Copied!");
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
      localStorage.setItem('clip_v4_last_data', "");
    }
  }

  async function clearImages() {
    const hasLastData = history.some(i => i.type !== 'text' && i.content === lastData);
    history = history.filter(item => item.type === 'text');
    saveHistory();
    
    if (hasLastData) {
      await invoke('set_clipboard_text', { text: "" });
      lastData = "";
      localStorage.setItem('clip_v4_last_data', "");
    }
  }

  async function clearText() {
    const hasLastData = history.some(i => i.type === 'text' && i.content === lastData);
    history = history.filter(item => item.type !== 'text');
    saveHistory();
    
    if (hasLastData) {
      await invoke('set_clipboard_text', { text: "" });
      lastData = "";
      localStorage.setItem('clip_v4_last_data', "");
    }
  }
  
  function addPassword() {
    if (!newPwdTitle || !newPwdPass) return;
    const newItem = { id: Date.now(), title: newPwdTitle, username: newPwdUser, password: newPwdPass, showPass: false };
    passwords = [newItem, ...passwords];
    savePasswords();
    newPwdTitle = ""; newPwdUser = ""; newPwdPass = "";
    triggerToast();
  }

  function deletePassword(id) {
    passwords = passwords.filter(p => p.id !== id);
    savePasswords();
  }

  function clearAllPasswords() {
    passwords = [];
    savePasswords();
  }

  function togglePassword(id) {
    passwords = passwords.map(p => {
      if (p.id === id) {
        return { ...p, showPass: !p.showPass };
      }
      return p;
    });
  }

  function getImgSrc(content) { try { return `data:image/png;base64,${content.split('|')[1]}`; } catch (e) { return ''; } }

  $: filteredImages = history.filter(item => item.type !== 'text');
  $: filteredText = searchQuery ? history.filter(item => item.type === 'text' && item.content.toLowerCase().includes(searchQuery.toLowerCase())) : history.filter(item => item.type === 'text');
  $: filteredPasswords = pwdSearchQuery ? passwords.filter(p => p.title.toLowerCase().includes(pwdSearchQuery.toLowerCase())) : passwords;
</script>

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
              <div class="image-card" on:click={() => copyImage(item.content)} in:fly={{ y: 10 }}>
                <img src={getImgSrc(item.content)} alt="" />
                <button class="img-del" on:click|stopPropagation={() => deleteItem(item.id)}>
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
          <div class="scroll-v list-container">
            {#each filteredText as item (item.id)}
              <div class="data-card pwd-card" on:click={() => copyText(item.content)} in:fly={{ x: 10 }}>
                <div class="pwd-info text-info">
                  <div class="pwd-title text-ellipsis">{item.content}</div>
                  <div class="pwd-detail">
                    <span class="detail-label">Text Clip</span>
                  </div>
                </div>
                <div class="pwd-actions">
                  <button class="action-icon" on:click|stopPropagation={() => copyText(item.content)}>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
                  </button>
                  <button class="action-icon" on:click|stopPropagation={() => deleteItem(item.id)}>
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
              <input bind:value={newPwdTitle} placeholder="Gmail" />
            </div>
            <div class="input-wrapper">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
              <input bind:value={newPwdUser} placeholder="Username" />
            </div>
            <div class="input-wrapper">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
              <input bind:value={newPwdPass} placeholder="Password" type="password" />
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
              <div class="data-card pwd-card">
                <div class="pwd-info">
                  <div class="pwd-title">{item.title}</div>
                  <div class="pwd-detail">
                    {#if item.username}
                      <span class="detail-label">Username</span>
                      <span class="detail-value">{item.username}</span>
                    {:else}
                      <span class="detail-label">Password</span>
                      <span class="detail-value">{item.showPass ? item.password : '********'}</span>
                    {/if}
                  </div>
                </div>
                <div class="pwd-actions">
                  <button class="action-icon" on:click={() => togglePassword(item.id)}>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                  </button>
                  <button class="action-icon" on:click={() => copyText(item.password)}>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
                  </button>
                  <button class="action-icon" on:click={() => deletePassword(item.id)}>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                  </button>
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

  .app-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    padding: 10px 20px 20px;
  }

  .header-nav {
    padding: 0 0 10px 0;
    border-bottom: 1px solid #f1f5f9;
  }

  .tabs {
    display: flex;
    gap: 20px;
  }

  .tabs button {
    background: none;
    border: none;
    color: #64748b;
    padding: 10px 4px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    border-bottom: 3px solid transparent;
    transition: all 0.2s;
  }

  .tabs button.active {
    color: #0f172a;
    border-bottom: 3px solid #7cb3f2;
    font-weight: 600;
  }

  .search-section {
    padding: 15px 0;
  }

  .search-box {
    display: flex;
    align-items: center;
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 6px;
    padding: 8px 12px;
  }

  .search-box .icon {
    width: 16px;
    height: 16px;
    color: #94a3b8;
  }

  .search-box input {
    flex: 1;
    background: none;
    border: none;
    outline: none;
    margin-left: 10px;
    font-size: 13px;
    color: #334155;
  }

  .content-area {
    flex: 1;
    overflow: hidden;
    display: flex;
  }

  .clipboard-layout, .password-layout {
    display: flex;
    width: 100%;
    gap: 25px;
  }

  .left-pane {
    width: 240px;
    display: flex;
    flex-direction: column;
  }

  .right-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .divider {
    width: 1px;
    background: #f1f5f9;
  }

  .pane-title {
    font-size: 15px;
    font-weight: 500;
    color: #0f172a;
    margin-bottom: 15px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .pane-title .icon {
    width: 18px;
    height: 18px;
    color: #334155;
  }

  .pane-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
  }

  .pane-header .pane-title {
    margin-bottom: 0;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .input-wrapper {
    display: flex;
    align-items: center;
    border: 1px solid #e2e8f0;
    border-radius: 6px;
    padding: 8px 12px;
    background: #ffffff;
  }

  .input-wrapper .icon {
    width: 16px;
    height: 16px;
    color: #64748b;
    margin-right: 10px;
  }

  .input-wrapper input {
    flex: 1;
    border: none;
    outline: none;
    font-size: 13px;
    color: #334155;
    background: none;
  }

  .btn-primary {
    background: #2b74ba;
    color: white;
    border: none;
    padding: 10px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    margin-top: 5px;
    transition: background 0.2s;
  }

  .btn-primary:hover {
    background: #1d4ed8;
  }

  .btn-secondary {
    background: #e0f2fe;
    color: #0284c7;
    border: none;
    padding: 6px 12px;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-secondary:hover {
    background: #bae6fd;
  }

  .scroll-v {
    flex: 1;
    overflow-y: auto;
    padding-right: 8px;
  }

  .scroll-v::-webkit-scrollbar { width: 4px; }
  .scroll-v::-webkit-scrollbar-thumb { background: #cbd5e1; border-radius: 4px; }

  .list-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .data-card {
    background: #ffffff;
    border: 1px solid #7cb3f2;
    border-radius: 12px;
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    transition: all 0.2s;
    cursor: pointer;
  }

  .data-card:hover {
    background: #f8fafc;
  }

  .pwd-card {
    padding: 10px 16px;
    cursor: default;
  }

  .pwd-card:hover {
    background: #ffffff;
  }

  .pwd-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0; 
  }

  .text-info {
    margin-right: 10px;
  }

  .pwd-title {
    font-size: 13px;
    font-weight: 500;
    color: #0f172a;
  }

  .text-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
  }

  .pwd-detail {
    font-size: 12px;
    color: #64748b;
    display: flex;
    gap: 6px;
  }

  .detail-label {
    color: #94a3b8;
  }

  .detail-value {
    letter-spacing: 0.5px;
  }

  .pwd-actions {
    display: flex;
    gap: 8px;
    align-items: center;
    flex-shrink: 0; 
  }

  .action-icon {
    background: none;
    border: none;
    color: #64748b;
    cursor: pointer;
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: all 0.2s;
    padding: 0;
    flex-shrink: 0;
  }

  .action-icon:hover {
    color: #0f172a;
    background: #f1f5f9;
  }

  .action-icon svg {
    width: 16px;
    height: 16px;
  }

  .image-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 10px;
  }

  .image-card {
    position: relative;
    border: 1px solid #7cb3f2;
    border-radius: 8px;
    overflow: hidden;
    aspect-ratio: 1;
    cursor: pointer;
  }

  .image-card:hover {
    border-color: #2b74ba;
  }

  .image-card img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .img-del {
    position: absolute;
    top: 4px;
    right: 4px;
    background: rgba(255,255,255,0.9);
    border: none;
    border-radius: 4px;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    color: #64748b;
  }

  .img-del svg {
    width: 12px;
    height: 12px;
  }

  .img-del:hover {
    background: #fee2e2;
    color: #ef4444;
  }

  .toast-tip {
    position: fixed;
    top: 30px;
    left: 50%;
    transform: translateX(-50%);
    background: #1e293b;
    color: white;
    padding: 10px 24px;
    border-radius: 30px;
    font-size: 13px;
    font-weight: 500;
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
    z-index: 100;
  }
</style>