<script lang="ts">
  import { IconArrowLeft as ArrowLeft, IconBrandGithub as BrandGithub, IconClipboard as Clipboard } from '@tabler/icons-svelte';
</script>

<svelte:head>
  <title>隐私说明 | Clipboard</title>
  <meta
    name="description"
    content="了解 Clipboard Windows 应用与官方网站如何处理剪贴板历史、密码库和基础访问数据。"
  />
  <link rel="canonical" href="https://hhrsc.github.io/website/privacy-policy/" />
  <meta name="robots" content="index,follow" />
</svelte:head>

<div class="policy-shell">
  <header>
    <a class="brand" href="/website/" aria-label="返回 Clipboard 官网">
      <span><Clipboard size={19} stroke={1.7} /></span>
      Clipboard
    </a>
    <a class="back-link" href="/website/"><ArrowLeft size={17} stroke={1.8} />返回官网</a>
  </header>

  <main>
    <p class="eyebrow">隐私说明</p>
    <h1>你的内容，<br />留在你的电脑里。</h1>
    <p class="lede">本说明适用于 Clipboard Windows 应用及其官方网站。生效日期：2026 年 9 月 1 日。</p>

    <div class="policy-grid">
      <section>
        <h2>应用保存什么</h2>
        <p>应用会在你的 Windows 设备上保存剪贴板文字、图片、分类、设置以及你主动添加到密码库的内容。</p>
        <p>密码库数据在本地加密保存。导出密码时，应用会明确提醒导出文件包含明文内容。</p>
      </section>

      <section>
        <h2>应用不会做什么</h2>
        <p>当前版本不要求账户，不提供云同步，也不在应用内投放广告或接入行为分析服务。</p>
        <p>除非你主动导出、复制或分享内容，应用不会把剪贴板历史和密码库上传到我们的服务器。</p>
      </section>

      <section>
        <h2>你可以控制什么</h2>
        <p>你可以在应用内删除单条记录、清空历史、调整保留时间、停止剪贴板监听，或导出和导入支持的数据。</p>
        <p>卸载应用前，如需保留数据，请先使用应用内提供的导出功能。</p>
      </section>

      <section>
        <h2>官网与下载</h2>
        <p>官网由 GitHub Pages 托管，安装包由 GitHub Releases 托管。GitHub 可能按照其隐私声明处理标准访问日志和下载请求。</p>
        <p>本官网目前没有接入广告、用户画像或第三方访问分析脚本。</p>
      </section>

      <section>
        <h2>问题与反馈</h2>
        <p>如果你对隐私、数据处理或产品行为有疑问，请在公开仓库提交 Issue。请勿在 Issue 中粘贴密码或私密剪贴板内容。</p>
        <a href="https://github.com/hhrsc/clipboard/issues/new/choose" target="_blank" rel="noreferrer">
          <BrandGithub size={18} stroke={1.8} />提交问题
        </a>
      </section>
    </div>
  </main>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(html),
  :global(body) {
    margin: 0;
    background: #f7f5f0;
    color: #22211f;
    font-family: 'Segoe UI Variable Text', 'Segoe UI', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  }

  :global(a) {
    color: inherit;
  }

  .policy-shell {
    min-height: 100dvh;
    background: radial-gradient(circle at 86% 8%, rgba(191, 80, 51, 0.12), transparent 25rem);
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 72px;
    padding: 0 4.5vw;
    border-bottom: 1px solid rgba(34, 33, 31, 0.12);
  }

  .brand,
  .back-link {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    text-decoration: none;
  }

  .brand {
    font-family: Georgia, 'Times New Roman', 'Songti SC', serif;
    font-size: 23px;
    font-weight: 600;
  }

  .brand span {
    display: grid;
    width: 32px;
    height: 32px;
    place-items: center;
    border: 1px solid rgba(34, 33, 31, 0.14);
    border-radius: 10px;
  }

  .back-link {
    font-size: 14px;
    font-weight: 650;
  }

  main {
    width: min(100% - 9vw, 1120px);
    margin: 0 auto;
    padding: 104px 0 140px;
  }

  .eyebrow {
    margin: 0 0 24px;
    color: #bd4f33;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.16em;
  }

  h1,
  h2,
  p {
    margin-top: 0;
  }

  h1,
  h2 {
    font-family: Georgia, 'Times New Roman', 'Songti SC', serif;
    font-weight: 500;
  }

  h1 {
    max-width: 820px;
    margin-bottom: 28px;
    font-size: clamp(48px, 7vw, 88px);
    line-height: 0.98;
    letter-spacing: -0.045em;
  }

  .lede {
    max-width: 660px;
    margin-bottom: 92px;
    color: #68645f;
    font-size: 18px;
    line-height: 1.75;
  }

  .policy-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    border-top: 1px solid rgba(34, 33, 31, 0.15);
  }

  section {
    padding: 42px 7vw 48px 0;
    border-bottom: 1px solid rgba(34, 33, 31, 0.15);
  }

  section:nth-child(even) {
    padding-right: 0;
    padding-left: 7vw;
    border-left: 1px solid rgba(34, 33, 31, 0.15);
  }

  section:last-child {
    grid-column: 1 / -1;
    padding-right: 0;
    padding-left: 0;
    border-left: 0;
  }

  h2 {
    margin-bottom: 18px;
    font-size: 30px;
  }

  section p {
    max-width: 520px;
    margin-bottom: 14px;
    color: #68645f;
    line-height: 1.75;
  }

  section a {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    margin-top: 12px;
    font-weight: 650;
    text-underline-offset: 4px;
  }

  :global(:focus-visible) {
    outline: 3px solid rgba(189, 79, 51, 0.42);
    outline-offset: 4px;
  }

  @media (max-width: 720px) {
    header {
      padding: 0 20px;
    }

    main {
      width: auto;
      padding: 72px 20px 96px;
    }

    .back-link {
      font-size: 0;
    }

    .lede {
      margin-bottom: 64px;
    }

    .policy-grid {
      grid-template-columns: 1fr;
    }

    section,
    section:nth-child(even),
    section:last-child {
      grid-column: auto;
      padding: 34px 0 38px;
      border-left: 0;
    }
  }

  @media (prefers-color-scheme: dark) {
    :global(html),
    :global(body) {
      background: #201f1d;
      color: #f1eee8;
    }

    .policy-shell {
      background: radial-gradient(circle at 86% 8%, rgba(205, 96, 65, 0.1), transparent 25rem);
    }

    header,
    .brand span,
    .policy-grid,
    section,
    section:nth-child(even) {
      border-color: rgba(255, 255, 255, 0.12);
    }

    .lede,
    section p {
      color: #bdb7af;
    }
  }
</style>
