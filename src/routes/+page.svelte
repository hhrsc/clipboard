<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { fade, fly } from 'svelte/transition';

  let searchQuery = "";
  let history = []; // 存储完整的历史记录
  let lastData = "";
  let showToast = false;

  // 辅助函数：保存到本地存储
  function saveHistory() {
    localStorage.setItem('clip_v4_split', JSON.stringify(history));
  }

  onMount(() => {
    // 启动时读取本地存储
    const saved = localStorage.getItem('clip_v4_split');
    if (saved) {
      try { history = JSON.parse(saved); } catch (e) { history = []; }
    }

    // 定时监听剪贴板
    setInterval(async () => {
      try {
        const data = await invoke('get_clipboard_data');
        if (data) {
          const [type, content] = data;
          // 如果内容与上次不同，则添加新记录
          if (content !== lastData) {
            // 添加一个 timestamp 字段用于界面显示
            const newItem = { type, content, id: Date.now(), timestamp: new Date().toLocaleString() };
            // 新记录插到最前面，保留最近 60 条
            history = [newItem, ...history].slice(0, 60);
            lastData = content;
            saveHistory();
          }
        }
      } catch (error) {
        console.error("监听出错:", error);
      }
    }, 1000);
  });

  // --- 交互功能 ---

  // 触发复制成功弹窗
  function triggerToast() {
    showToast = true;
    setTimeout(() => { showToast = false; }, 800);
  }

  // 点击文本卡片进行复制
  async function copyText(text) {
    await invoke('set_clipboard_text', { text });
    lastData = text;
    triggerToast();
  }

  // 点击图片卡片（暂时只提示）
  function handleImageClick() {
    alert("图片暂不支持直接点击复制，请在原始位置右键复制。");
  }

  // 删除单个条目
  function deleteItem(id) {
    history = history.filter(item => item.id !== id);
    saveHistory();
  }

  // --- 新增：独立清空功能 ---

  // 清空所有图片
  function clearImages() {
    if (history.some(i => i.type !== 'text') && confirm("确定要清空所有图片记录吗？")) {
      history = history.filter(item => item.type === 'text');
      saveHistory();
    }
  }

  // 清空所有文本
  function clearText() {
    if (history.some(i => i.type === 'text') && confirm("确定要清空所有文本记录吗？")) {
      history = history.filter(item => item.type !== 'text');
      saveHistory();
    }
  }

  // 辅助函数：解析 Base64 图片
  function getImgSrc(content) {
    try {
        const base64 = content.split('|')[1];
        return `data:image/png;base64,${base64}`;
    } catch (e) { return ''; }
  }

  // 辅助函数：格式化简短时间 (模拟图中的格式)
  function formatTime(timestampStr) {
      const date = new Date(timestampStr);
      const month = (date.getMonth() + 1).toString().padStart(2, '0');
      const day = date.getDate().toString().padStart(2, '0');
      const hours = date.getHours().toString().padStart(2, '0');
      const minutes = date.getMinutes().toString().padStart(2, '0');
      return `${month}-${day} ${hours}:${minutes}`;
  }

  // --- 数据响应式过滤 ---

  // 图片列表通常不参与文本搜索，显示所有图片
  $: filteredImages = history.filter(item => item.type !== 'text');

  // 文本列表参与搜索过滤
  $: filteredText = searchQuery
    ? history.filter(item => item.type === 'text' && item.content.toLowerCase().includes(searchQuery.toLowerCase()))
    : history.filter(item => item.type === 'text');

</script>

<main class="app-container">
  <div class="search-bar-wrapper">
    <div class="search-bar">
      <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
      <input type="text" placeholder="Search clipboard..." bind:value={searchQuery} />
    </div>
  </div>

  <div class="content-split-view">

    <div class="column image-column">
      <div class="column-header">
        <h2>IMAGES</h2>
        <button class="clear-btn" on:click={clearImages}>Clear All</button>
      </div>
      <div class="scroll-area image-grid">
        {#each filteredImages as item (item.id)}
          <div class="image-card" on:click={handleImageClick} in:fly={{ y: 10, duration: 200 }}>
            <img src={getImgSrc(item.content)} alt="clip" loading="lazy" />
            <button class="delete-btn-overlay" on:click|stopPropagation={() => deleteItem(item.id)}>
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
            <div class="timestamp-overlay">{formatTime(item.timestamp)}</div>
          </div>
        {/each}
      </div>
    </div>

    <div class="column text-column">
      <div class="column-header">
        <h2>TEXT</h2>
        <button class="clear-btn" on:click={clearText}>Clear All</button>
      </div>
      <div class="scroll-area text-list">
        {#each filteredText as item (item.id)}
          <div class="text-card" on:click={() => copyText(item.content)} in:fly={{ x: 10, duration: 200 }}>
            <p class="text-content">{item.content}</p>
             <button class="delete-btn-inline" on:click|stopPropagation={() => deleteItem(item.id)}>
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#8E8E93" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>
          </div>
        {/each}
      </div>
    </div>

  </div>

  {#if showToast}
    <div class="toast-container" transition:fade={{ duration: 150 }}>
      <div class="toast">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#4CD964" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
        <span>Copied!</span>
      </div>
    </div>
  {/if}
</main>

<style>
  /* 全局样式重置与主题定义 */
  :global(body) {
    margin: 0;
    /* 深色背景渐变 */
    background: linear-gradient(135deg, #0a0f1d 0%, #060912 100%);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    color: #ffffff;
    overflow: hidden; /* 禁止 body 滚动 */
  }

  /* 主容器 */
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh; /* 占满全屏高度 */
    padding: 20px;
    box-sizing: border-box;
    /* 给整个应用加一个微妙的蓝色光晕背景 */
    background: radial-gradient(circle at 50% -20%, rgba(0, 122, 255, 0.15), transparent 70%);
  }

  /* --- 顶部搜索栏 --- */
  .search-bar-wrapper {
    margin-bottom: 20px;
  }
  .search-bar {
    display: flex;
    align-items: center;
    background: rgba(255, 255, 255, 0.08); /* 半透明背景 */
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 10px 16px;
    backdrop-filter: blur(10px); /* 毛玻璃效果 */
  }
  .search-icon {
    color: #8E8E93;
    margin-right: 10px;
  }
  .search-bar input {
    flex: 1;
    background: transparent;
    border: none;
    color: #fff;
    font-size: 16px;
    outline: none;
  }
  .search-bar input::placeholder {
    color: #8E8E93;
  }

  /* --- 主体内容双栏结构 --- */
  .content-split-view {
    flex: 1; /* 占据剩余高度 */
    display: flex;
    gap: 25px; /* 两栏之间的间距 */
    overflow: hidden; /* 防止自身滚动 */
  }

  /* 通用栏目样式 */
  .column {
    flex: 1; /* 两栏平分宽度 */
    display: flex;
    flex-direction: column;
    overflow: hidden;
    /* 给栏目加一个深色半透明背景框 */
    background: rgba(20, 25, 40, 0.6);
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    padding: 16px;
  }

  /* 栏目头部 (标题 + Clear All 按钮) */
  .column-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }
  .column-header h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    letter-spacing: 0.5px;
    color: #EBEBF5;
  }
  .clear-btn {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #EBEBF5;
    padding: 6px 12px;
    border-radius: 8px;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .clear-btn:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  /* 滚动区域容器 */
  .scroll-area {
    flex: 1;
    overflow-y: auto; /* 允许内部滚动 */
    /* 自定义滚动条样式 (Chrome/Safari) */
    &::-webkit-scrollbar { width: 6px; }
    &::-webkit-scrollbar-track { background: transparent; }
    &::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.2); border-radius: 3px; }
  }

  /* --- 左侧：图片宫格样式 --- */
  .image-grid {
    display: grid;
    /* 自动适应列数，最小宽度 130px */
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: 12px;
    padding-right: 4px; /* 留点空隙给滚动条 */
  }
  .image-card {
    position: relative;
    aspect-ratio: 16 / 10; /* 固定宽高比 */
    border-radius: 12px;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.1);
    cursor: default;
    transition: transform 0.2s;
  }
  .image-card:hover {
    transform: scale(1.02);
    border-color: rgba(0, 122, 255, 0.5);
  }
  .image-card img {
    width: 100%;
    height: 100%;
    object-fit: cover; /* 填满容器，保持比例 */
  }
  /* 图片卡片上的浮层按钮和文字 */
  .delete-btn-overlay {
    position: absolute;
    top: 8px;
    right: 8px;
    background: rgba(0, 0, 0, 0.5);
    border: none;
    width: 24px; height: 24px;
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
    color: white;
    cursor: pointer;
    opacity: 0.7; transition: opacity 0.2s;
  }
  .delete-btn-overlay:hover { opacity: 1; background: rgba(255, 59, 48, 0.8); }
  .timestamp-overlay {
    position: absolute;
    bottom: 0; left: 0; right: 0;
    padding: 4px 8px;
    background: linear-gradient(to top, rgba(0,0,0,0.7), transparent);
    color: rgba(255, 255, 255, 0.8);
    font-size: 11px;
    text-align: left;
  }

  /* --- 右侧：文本列表样式 --- */
  .text-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding-right: 4px;
  }
  .text-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: rgba(44, 44, 46, 0.6); /* 深灰色卡片背景 */
    border: 1px solid rgba(255, 255, 255, 0.05);
    padding: 14px 16px;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .text-card:hover {
    background: rgba(58, 58, 60, 0.8);
    border-color: rgba(0, 122, 255, 0.3);
  }
  .text-card:active {
      transform: scale(0.99);
  }
  .text-content {
    margin: 0;
    font-size: 15px;
    color: #EBEBF5;
    /* 单行显示，超出省略 */
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1; /* 占据剩余空间 */
    margin-right: 12px;
  }
  .delete-btn-inline {
    background: transparent;
    border: none;
    padding: 4px;
    display: flex; align-items: center; justify-content: center;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.2s;
  }
  .delete-btn-inline:hover svg {
    stroke: #FF3B30; /* hover 时变红 */
  }

  /* --- 复制成功提示弹窗 (保持不变) --- */
  .toast-container {
    position: fixed; top: 0; left: 0; right: 0; bottom: 0;
    display: flex; align-items: center; justify-content: center;
    pointer-events: none; z-index: 999;
  }
  .toast {
    background: rgba(0, 0, 0, 0.85); color: white;
    padding: 12px 24px; border-radius: 50px;
    display: flex; align-items: center; gap: 8px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255,255,255,0.1);
    font-weight: 600;
  }
</style>