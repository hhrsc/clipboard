<script lang="ts">
  import { animate, hover, inView, press, stagger } from 'motion';
  import { quintOut } from 'svelte/easing';
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import {
    IconArrowDown as ArrowDown,
    IconArrowRight as ArrowRight,
    IconBrandGithub as BrandGithub,
    IconCheck as Check,
    IconChecks as Checks,
    IconClipboard as Clipboard,
    IconCopy as Copy,
    IconExternalLink as ExternalLink,
    IconPhoto as Image,
    IconKeyboard as Keyboard,
    IconLock as Lock,
    IconMenu2 as Menu2,
    IconShieldLock as ShieldLock,
    IconX as X
  } from '@tabler/icons-svelte';

  const downloadHref =
    'https://github.com/hhrsc/clipboard/releases/download/v0.1.0/Clipboard_0.1.0_x64-setup.exe';
  const releaseHref = 'https://github.com/hhrsc/clipboard/releases/tag/v0.1.0';
  const sourceHref = 'https://github.com/hhrsc/clipboard';
  const feedbackHref = 'https://github.com/hhrsc/clipboard/issues/new/choose';
  const checksum = 'C5BE35E575E698696C376DF46247D4A670D0C8880098F5F2EAD5B7F921B8AE4C';
  const softwareStructuredData = JSON.stringify({
    '@context': 'https://schema.org',
    '@type': 'SoftwareApplication',
    name: 'Clipboard',
    description: '一款本地优先的 Windows 剪贴板工具，支持文字、图片、分类、搜索、多格式复制与本地加密密码库。',
    applicationCategory: 'UtilitiesApplication',
    operatingSystem: 'Windows 10, Windows 11',
    softwareVersion: '0.1.0',
    downloadUrl: downloadHref,
    url: 'https://hhrsc.github.io/website/',
    isAccessibleForFree: true
  });
  const faqStructuredData = JSON.stringify({
    '@context': 'https://schema.org',
    '@type': 'FAQPage',
    mainEntity: [
      {
        '@type': 'Question',
        name: 'Clipboard 会把数据上传到云端吗？',
        acceptedAnswer: {
          '@type': 'Answer',
          text: '不会。当前版本没有云同步，剪贴板历史和密码库都保存在本机。'
        }
      },
      {
        '@type': 'Question',
        name: 'Clipboard 支持哪些内容？',
        acceptedAnswer: {
          '@type': 'Answer',
          text: '支持文字、富文本格式和图片历史，也提供独立的本地加密密码库。'
        }
      },
      {
        '@type': 'Question',
        name: 'Clipboard 能自定义快捷键吗？',
        acceptedAnswer: {
          '@type': 'Answer',
          text: '可以。你可以在设置中录制全局快捷键，具体可用组合受 Windows 注册规则约束。'
        }
      },
      {
        '@type': 'Question',
        name: 'Clipboard 安装包适用于什么平台？',
        acceptedAnswer: {
          '@type': 'Answer',
          text: '当前安装包面向 64 位 Windows 桌面系统。'
        }
      },
      {
        '@type': 'Question',
        name: 'Windows 提示“已保护你的电脑”怎么办？',
        acceptedAnswer: {
          '@type': 'Answer',
          text: '当前安装包尚未进行代码签名，新应用可能触发 SmartScreen。请只从官网或 GitHub Releases 下载，并核对本页 SHA-256；如无法确认来源，请不要运行。'
        }
      }
    ]
  });
  let menuOpen = false;
  let checksumState: 'idle' | 'copied' | 'error' = 'idle';
  const motionEase = [0.16, 1, 0.3, 1] as const;

  function shouldReduceMotion() {
    return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  }

  function fallbackCopy(text: string) {
    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';
    textarea.style.opacity = '0';
    document.body.appendChild(textarea);
    textarea.select();
    const copied = document.execCommand('copy');
    textarea.remove();
    return copied;
  }

  async function copyChecksum() {
    try {
      if (navigator.clipboard?.writeText) {
        await navigator.clipboard.writeText(checksum);
      } else if (!fallbackCopy(checksum)) {
        throw new Error('copy unavailable');
      }
      checksumState = 'copied';
      window.setTimeout(() => (checksumState = 'idle'), 3500);
    } catch {
      checksumState = fallbackCopy(checksum) ? 'copied' : 'error';
    }
  }

  function interactiveMedia(node: HTMLElement) {
    const reduceMotion = shouldReduceMotion();
    const coarsePointer = window.matchMedia('(pointer: coarse)').matches;
    if (reduceMotion || coarsePointer) return {};

    // 截图只提供轻微的悬浮反馈，避免连续追赶指针造成迟滞和眩晕感。
    const stopHover = hover(node, (element) => {
      const enter = animate(element, { y: -3, scale: 1.004 }, { duration: 0.18, ease: motionEase });
      return () => {
        enter.stop();
        animate(element, { y: 0, scale: 1 }, { duration: 0.24, ease: motionEase });
      };
    });

    return {
      destroy() {
        stopHover();
      }
    };
  }

  function interactiveControl(node: HTMLElement) {
    if (shouldReduceMotion()) return {};

    let hovered = false;
    let pressed = false;
    let controls: ReturnType<typeof animate> | undefined;
    const render = (duration: number) => {
      controls?.stop();
      controls = animate(
        node,
        { y: pressed ? 1 : hovered ? -2 : 0, scale: pressed ? 0.985 : 1 },
        { duration, ease: motionEase }
      );
    };

    // hover/press 会过滤触摸设备上的伪悬停，并让键盘触发保持与指针一致。
    const stopHover = hover(node, () => {
      hovered = true;
      render(0.16);
      return () => {
        hovered = false;
        render(0.2);
      };
    });
    const stopPress = press(node, () => {
      pressed = true;
      render(0.09);
      return () => {
        pressed = false;
        render(0.16);
      };
    });

    return {
      destroy() {
        controls?.stop();
        stopHover();
        stopPress();
      }
    };
  }

  function staggerMenu(node: HTMLElement) {
    if (shouldReduceMotion()) return {};
    const controls = animate(
      node.querySelectorAll('a'),
      { opacity: [0, 1], y: [-6, 0] },
      { duration: 0.2, delay: stagger(0.03), ease: motionEase }
    );
    return { destroy: () => controls.stop() };
  }

  function stateMotion(node: HTMLElement) {
    if (shouldReduceMotion()) return {};
    const controls = animate(node, { opacity: [0, 1], scale: [0.96, 1] }, { duration: 0.16, ease: motionEase });
    return { destroy: () => controls.stop() };
  }

  function animateFaq(event: Event) {
    const details = event.currentTarget as HTMLDetailsElement;
    if (!details.open || shouldReduceMotion()) return;
    const answer = details.querySelector<HTMLElement>('.faq-answer');
    if (answer) animate(answer, { opacity: [0, 1], y: [-4, 0] }, { duration: 0.22, ease: motionEase });
  }

  onMount(() => {
    if (shouldReduceMotion()) return;

    // 首屏只编排一次，后续区块保持短促，避免用户滚动时反复等待页面表演。
    const heroControls = animate(
      document.querySelectorAll('.hero-copy > *'),
      { opacity: [0, 1], y: [12, 0] },
      { duration: 0.36, delay: stagger(0.045), ease: motionEase }
    );
    const productControls = animate(
      '.hero-product',
      { opacity: [0, 1], scale: [0.985, 1] },
      { duration: 0.42, delay: 0.08, ease: motionEase }
    );
    const stopInView = inView(
      '[data-motion="section"]',
      (element) => {
        animate(element, { opacity: [0.84, 1], y: [8, 0] }, { duration: 0.3, ease: motionEase });
      },
      { amount: 0.14, margin: '0px 0px -6% 0px' }
    );

    return () => {
      heroControls.stop();
      productControls.stop();
      stopInView();
    };
  });
</script>

<svelte:head>
  <title>Clipboard | 本地优先的 Windows 剪贴板工具</title>
  <meta
    name="description"
    content="Clipboard 是一款本地优先的 Windows 剪贴板工具，集中保存文字、图片与密码，支持分类、搜索、自由选取与多格式复制。"
  />
  <meta name="theme-color" content="#f7f5f0" />
  <link rel="canonical" href="https://hhrsc.github.io/website/" />
  <meta property="og:type" content="website" />
  <meta property="og:locale" content="zh_CN" />
  <meta property="og:title" content="Clipboard | 本地优先的 Windows 剪贴板工具" />
  <meta
    property="og:description"
    content="保存文字与图片历史，自由选取内容并复制为纯文本、Markdown、HTML 等格式。"
  />
  <meta property="og:url" content="https://hhrsc.github.io/website" />
  <meta property="og:image" content="https://hhrsc.github.io/marketing/clipboard-recent.webp" />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content="Clipboard | 本地优先的 Windows 剪贴板工具" />
  <meta
    name="twitter:description"
    content="保存文字与图片历史，自由选取内容并复制为多种格式。"
  />
  <meta name="twitter:image" content="https://hhrsc.github.io/marketing/clipboard-recent.webp" />
  {@html `<script type="application/ld+json">${softwareStructuredData}</script>`}
  {@html `<script type="application/ld+json">${faqStructuredData}</script>`}
</svelte:head>

<div class="site-shell">
  <header class="site-header">
    <a class="brand" href="#top" aria-label="Clipboard 首页">
      <span class="brand-mark"><Clipboard size={19} stroke={1.7} /></span>
      <span>Clipboard</span>
    </a>

    <nav class="desktop-nav" aria-label="主要导航">
      <a href="#features">功能</a>
      <a href="#privacy">隐私</a>
      <a href="#faq">常见问题</a>
    </nav>

    <a class="header-download" href={downloadHref} use:interactiveControl>
      下载 Windows 版
      <ArrowDown size={17} stroke={1.8} />
    </a>

    <button
      class="menu-button"
      type="button"
      aria-label={menuOpen ? '关闭菜单' : '打开菜单'}
      aria-expanded={menuOpen}
      onclick={() => (menuOpen = !menuOpen)}
    >
      {#if menuOpen}<X size={21} />{:else}<Menu2 size={21} />{/if}
    </button>
  </header>

  {#if menuOpen}
    <nav
      class="mobile-nav"
      aria-label="移动端导航"
      use:staggerMenu
      in:fly={{ y: -8, duration: 220, easing: quintOut }}
      out:fly={{ y: -4, duration: 150, easing: quintOut }}
    >
      <a href="#features" onclick={() => (menuOpen = false)}>功能</a>
      <a href="#privacy" onclick={() => (menuOpen = false)}>隐私</a>
      <a href="#faq" onclick={() => (menuOpen = false)}>常见问题</a>
      <a href={downloadHref}>下载 Windows 版</a>
    </nav>
  {/if}

  <main id="top">
    <section class="hero">
      <div class="hero-copy">
        <p class="eyebrow">专为 Windows 设计</p>
        <h1>复制以后，<br />依然找得到。</h1>
        <p class="hero-lede">在本地保存文字、图片与密码，让常用内容不再消失在下一次复制之后。</p>
        <div class="hero-actions">
          <a class="primary-button" href={downloadHref} use:interactiveControl>
            <span class="windows-symbol" aria-hidden="true">⊞</span>
            下载 Windows 版
          </a>
          <a class="text-link" href="#features">
            看看它怎么工作
            <ArrowRight size={18} stroke={1.8} />
          </a>
        </div>
      </div>

      <figure class="hero-product">
        <div class="hero-halo" aria-hidden="true"></div>
        <div class="interactive-media" use:interactiveMedia>
          <img
            src="/marketing/clipboard-recent.webp"
            alt="Clipboard 的文字历史和详情界面"
            width="1586"
            height="992"
            fetchpriority="high"
          />
        </div>
      </figure>
    </section>

    <section class="fact-strip" aria-label="产品特点" data-motion="section">
      <div><Check size={18} stroke={1.8} /><span>本地保存</span></div>
      <div><Keyboard size={18} stroke={1.8} /><span>全局快捷键</span></div>
      <div><Image size={18} stroke={1.8} /><span>图片历史</span></div>
      <div><ShieldLock size={18} stroke={1.8} /><span>加密密码库</span></div>
    </section>

    <section class="trust-section" aria-labelledby="trust-heading">
      <div class="trust-copy" data-motion="section">
        <h2 id="trust-heading">下载前，把该知道的写清楚。</h2>
        <p>安装包由 GitHub Releases 托管。版本、系统要求、源码与文件校验值都公开，不让你靠猜。</p>
        <div class="trust-links">
          <a href={releaseHref} target="_blank" rel="noreferrer">
            查看版本说明
            <ExternalLink size={17} stroke={1.8} />
          </a>
          <a href={sourceHref} target="_blank" rel="noreferrer">
            <BrandGithub size={18} stroke={1.8} />
            查看源码
          </a>
        </div>
      </div>
      <div class="trust-facts" data-motion="section">
        <div>
          <span>当前版本</span>
          <strong>0.1.0</strong>
        </div>
        <div>
          <span>适用系统</span>
          <strong>64 位 Windows</strong>
        </div>
        <div>
          <span>安装包大小</span>
          <strong>3.52 MB</strong>
        </div>
        <div class="checksum-row">
          <span>SHA-256</span>
          <code title={checksum}>{checksum}</code>
          <button type="button" onclick={copyChecksum} aria-live="polite" use:interactiveControl>
            {#if checksumState === 'copied'}
              <span class="checksum-status" use:stateMotion><Checks size={17} stroke={1.8} /> 已复制</span>
            {:else if checksumState === 'error'}
              <span class="checksum-status" use:stateMotion>复制失败</span>
            {:else}
              <span class="checksum-status" use:stateMotion><Copy size={17} stroke={1.8} /> 复制</span>
            {/if}
          </button>
        </div>
      </div>
    </section>

    <section class="story-section" id="features">
      <div class="story-copy" data-motion="section">
        <h2>不是多记一点，<br />是少找很多次。</h2>
        <p>搜索、置顶、分类和格式化复制放在同一个窗口里。打开它，找到它，继续手上的事。</p>
      </div>
      <figure class="story-visual" data-motion="section">
        <div class="interactive-media" use:interactiveMedia>
          <img
            src="/marketing/paper-clip-editorial.webp"
            alt="层叠纸张与陶土色回形针组成的安静桌面画面"
            width="1536"
            height="1024"
            loading="lazy"
          />
        </div>
      </figure>
    </section>

    <section class="format-section" aria-labelledby="format-heading">
      <div class="format-heading" data-motion="section">
        <p class="eyebrow">精确复制</p>
        <h2 id="format-heading">只复制你真正需要的那一段。</h2>
        <p>在详情中自由选取文字，再复制为纯文本、Markdown、HTML，或去掉换行。内容和格式，都由你决定。</p>
        <div class="format-list" aria-label="支持的复制方式">
          <span>自由选取</span>
          <span>纯文本</span>
          <span>Markdown</span>
          <span>HTML</span>
          <span>单行文本</span>
        </div>
      </div>
      <figure class="format-shot" data-motion="section">
        <div class="interactive-media" use:interactiveMedia>
          <img
            src="/marketing/clipboard-recent.webp"
            alt="Clipboard 详情面板中的文字选择和多格式复制操作"
            width="1586"
            height="992"
            loading="lazy"
          />
        </div>
      </figure>
    </section>

    <section class="feature-row feature-images">
      <div class="feature-copy" data-motion="section">
        <Image size={28} stroke={1.5} />
        <h2>图片，也有自己的历史。</h2>
        <p>浏览缩略图，按时间和分类筛选，再把需要的图片一键复制回去。</p>
      </div>
      <figure class="app-shot wide-shot" data-motion="section">
        <div class="interactive-media" use:interactiveMedia>
          <img
            src="/marketing/clipboard-images.webp"
            alt="Clipboard 的图片历史、筛选和预览界面"
            width="1586"
            height="992"
            loading="lazy"
          />
        </div>
      </figure>
    </section>

    <section class="feature-row feature-vault" id="privacy">
      <figure class="app-shot" data-motion="section">
        <div class="interactive-media" use:interactiveMedia>
          <img
            src="/marketing/clipboard-passwords.webp"
            alt="Clipboard 的本地加密密码库界面"
            width="1586"
            height="992"
            loading="lazy"
          />
        </div>
      </figure>
      <div class="feature-copy" data-motion="section">
        <Lock size={28} stroke={1.5} />
        <h2>密码留在本机，<br />不去云端绕路。</h2>
        <p>密码库在本地加密保存，默认使用主密码解锁。分组、编辑、导入和导出都在应用内完成。</p>
      </div>
    </section>

    <section class="control-section">
      <div class="control-heading" data-motion="section">
        <h2>把工具调成你顺手的样子。</h2>
        <p>设置监听、自启动、保留时长和快捷键。需要时出现，不需要时安静待着。</p>
      </div>
      <figure class="settings-shot" data-motion="section">
        <div class="interactive-media" use:interactiveMedia>
          <img
            src="/marketing/clipboard-settings.webp"
            alt="Clipboard 的常规设置与快捷键界面"
            width="1586"
            height="992"
            loading="lazy"
          />
        </div>
      </figure>
    </section>

    <section class="faq-section" id="faq">
      <h2 data-motion="section">常见问题</h2>
      <div class="faq-list" data-motion="section">
        <details open ontoggle={animateFaq}>
          <summary>Clipboard 会把数据上传到云端吗？</summary>
          <p class="faq-answer">不会。当前版本没有云同步，剪贴板历史和密码库都保存在本机。</p>
        </details>
        <details ontoggle={animateFaq}>
          <summary>支持哪些内容？</summary>
          <p class="faq-answer">支持文字、富文本格式和图片历史，也提供独立的本地加密密码库。</p>
        </details>
        <details ontoggle={animateFaq}>
          <summary>能自定义快捷键吗？</summary>
          <p class="faq-answer">可以。你可以在设置中录制全局快捷键，具体可用组合受 Windows 注册规则约束。</p>
        </details>
        <details ontoggle={animateFaq}>
          <summary>安装包适用于什么平台？</summary>
          <p class="faq-answer">当前安装包面向 64 位 Windows 桌面系统。</p>
        </details>
        <details ontoggle={animateFaq}>
          <summary>Windows 提示“已保护你的电脑”怎么办？</summary>
          <p class="faq-answer">当前安装包尚未进行代码签名，新应用可能触发 SmartScreen。请只从官网或 GitHub Releases 下载，并核对本页 SHA-256；如无法确认来源，请不要运行。</p>
        </details>
        <details ontoggle={animateFaq}>
          <summary>在哪里提交问题或建议？</summary>
          <p class="faq-answer">可以前往 <a href={feedbackHref} target="_blank" rel="noreferrer">GitHub Issues</a>，描述你的使用场景和期望结果。</p>
        </details>
      </div>
    </section>

    <section class="final-cta" data-motion="section">
      <div>
        <h2>让下一次复制，<br />不再覆盖上一次。</h2>
        <p>下载 Clipboard，把经常用到的内容留在手边。</p>
      </div>
      <a class="primary-button light-button" href={downloadHref} use:interactiveControl>
        <span class="windows-symbol" aria-hidden="true">⊞</span>
        下载 Windows 版
      </a>
    </section>
  </main>

  <footer>
    <a class="brand footer-brand" href="#top" aria-label="返回顶部">
      <span class="brand-mark"><Clipboard size={19} stroke={1.7} /></span>
      <span>Clipboard</span>
    </a>
    <p>一款安静、直接的 Windows 本地剪贴板工具。</p>
    <p class="footer-links">
      <a href="/website/guides/windows-clipboard-history/">使用指南</a>
      <a href="/website/guides/copy-text-as-markdown-html/">多格式复制</a>
      <a href="/website/privacy-policy">隐私说明</a>
      <a href={sourceHref} target="_blank" rel="noreferrer">源码</a>
      <a href={feedbackHref} target="_blank" rel="noreferrer">反馈</a>
    </p>
  </footer>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(html) {
    scroll-behavior: smooth;
    background: #f7f5f0;
  }

  :global(body) {
    margin: 0;
    background: #f7f5f0;
    color: #22211f;
    font-family: 'Segoe UI Variable Text', 'Segoe UI', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  }

  :global(button),
  :global(a) {
    font: inherit;
  }

  :global(a) {
    color: inherit;
  }

  .site-shell {
    min-height: 100dvh;
    overflow: clip;
    background:
      radial-gradient(circle at 84% 5%, rgba(196, 81, 50, 0.06), transparent 22rem),
      #f7f5f0;
  }

  .site-header {
    position: sticky;
    top: 0;
    z-index: 20;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    min-height: 72px;
    padding: 0 4.5vw;
    border-bottom: 1px solid rgba(34, 33, 31, 0.11);
    background: rgba(247, 245, 240, 0.88);
    backdrop-filter: blur(18px) saturate(130%);
  }

  .brand {
    display: inline-flex;
    align-items: center;
    justify-self: start;
    gap: 10px;
    width: fit-content;
    text-decoration: none;
    font-family: Georgia, 'Times New Roman', 'Songti SC', serif;
    font-size: 24px;
    line-height: 1;
  }

  .brand-mark {
    display: grid;
    width: 33px;
    height: 33px;
    place-items: center;
    border: 1px solid rgba(34, 33, 31, 0.15);
    border-radius: 12px;
    color: #bd4f33;
  }

  .desktop-nav {
    display: flex;
    align-items: center;
    gap: 34px;
    color: #68645f;
    font-size: 14px;
  }

  .desktop-nav a,
  .text-link {
    text-decoration: none;
    transition: color 180ms ease;
  }

  .desktop-nav a:hover,
  .text-link:hover {
    color: #bd4f33;
  }

  .header-download {
    display: inline-flex;
    align-items: center;
    justify-self: end;
    gap: 8px;
    min-height: 42px;
    padding: 0 18px;
    border-radius: 12px;
    background: #242320;
    color: #f8f6f1;
    text-decoration: none;
    font-size: 14px;
    font-weight: 600;
    transition: background 180ms ease, color 180ms ease;
  }

  .header-download:hover {
    background: #bd4f33;
  }

  .menu-button,
  .mobile-nav {
    display: none;
  }

  main {
    width: min(100%, 1600px);
    margin: 0 auto;
  }

  .hero {
    position: relative;
    display: grid;
    grid-template-columns: minmax(0, 0.78fr) minmax(520px, 1.22fr);
    align-items: center;
    min-height: calc(100dvh - 72px);
    padding: 64px 4.5vw 72px;
  }

  .hero-copy {
    position: relative;
    z-index: 2;
    max-width: 640px;
    padding-right: 5vw;
  }

  .eyebrow {
    margin: 0 0 24px;
    color: #bd4f33;
    font-size: 12px;
    font-weight: 650;
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
    font-weight: 400;
    letter-spacing: -0.04em;
  }

  h1 {
    margin-bottom: 26px;
    font-size: clamp(50px, 5.35vw, 82px);
    line-height: 1.02;
  }

  .hero-lede {
    max-width: 510px;
    margin-bottom: 32px;
    color: #65615d;
    font-size: clamp(17px, 1.45vw, 20px);
    line-height: 1.7;
  }

  .hero-actions {
    display: flex;
    align-items: center;
    gap: 26px;
  }

  .primary-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 52px;
    padding: 0 22px;
    border-radius: 12px;
    background: #242320;
    color: #faf8f4;
    text-decoration: none;
    font-weight: 650;
    white-space: nowrap;
    box-shadow: 0 14px 35px rgba(66, 51, 43, 0.12);
    transition: background 180ms ease, color 180ms ease;
  }

  .primary-button:hover {
    background: #bd4f33;
  }

  .windows-symbol {
    font-size: 22px;
    line-height: 1;
  }

  .text-link {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-size: 15px;
    font-weight: 600;
  }

  .text-link :global(svg) {
    transition: transform 180ms ease;
  }

  .text-link:hover :global(svg) {
    transform: translateX(4px);
  }

  .hero-product {
    position: relative;
    z-index: 1;
    width: 100%;
    margin: 0;
  }

  .interactive-media {
    position: relative;
    overflow: hidden;
    border: 1px solid rgba(52, 45, 41, 0.14);
    border-radius: 14px;
    background: #e9e5dd;
    box-shadow: 0 32px 90px rgba(78, 59, 46, 0.16);
    transition: box-shadow 240ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  .interactive-media::after {
    content: '';
    position: absolute;
    inset: 0;
    pointer-events: none;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.13), transparent 38%);
    opacity: 0;
    transition: opacity 220ms ease;
  }

  .interactive-media:hover::after {
    opacity: 1;
  }

  .interactive-media:hover {
    box-shadow: 0 38px 100px rgba(78, 59, 46, 0.2);
  }

  .interactive-media img {
    display: block;
    width: 100%;
    height: auto;
  }

  .hero-halo {
    position: absolute;
    inset: 8% 12% -4% -8%;
    z-index: -1;
    border-radius: 50%;
    background: rgba(191, 80, 51, 0.14);
    filter: blur(80px);
  }

  .fact-strip {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    margin: 0 4.5vw;
    border-top: 1px solid rgba(34, 33, 31, 0.14);
    border-bottom: 1px solid rgba(34, 33, 31, 0.14);
  }

  .fact-strip div {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 86px;
    color: #514e4a;
    font-size: 14px;
  }

  .fact-strip div + div {
    border-left: 1px solid rgba(34, 33, 31, 0.12);
  }

  .fact-strip :global(svg),
  .feature-copy > :global(svg) {
    color: #bd4f33;
  }

  .story-section,
  .trust-section,
  .format-section,
  .feature-row,
  .control-section,
  .faq-section {
    padding: 144px 4.5vw;
  }

  .trust-section {
    display: grid;
    grid-template-columns: minmax(280px, 0.82fr) minmax(0, 1.18fr);
    gap: 9vw;
    align-items: start;
  }

  .trust-copy h2 {
    max-width: 650px;
    margin-bottom: 24px;
    font-size: clamp(40px, 4.25vw, 66px);
    line-height: 1.05;
  }

  .trust-copy p {
    max-width: 520px;
    margin-bottom: 30px;
    color: #68645f;
    font-size: 17px;
    line-height: 1.75;
  }

  .trust-links {
    display: flex;
    flex-wrap: wrap;
    gap: 16px 26px;
  }

  .trust-links a,
  .footer-links a,
  .faq-list p a {
    text-underline-offset: 4px;
    text-decoration-thickness: 1px;
  }

  .trust-links a {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    color: #34312e;
    font-size: 14px;
    font-weight: 650;
  }

  .trust-facts {
    border-top: 1px solid rgba(34, 33, 31, 0.16);
  }

  .trust-facts > div {
    display: grid;
    grid-template-columns: minmax(120px, 0.55fr) minmax(180px, 1fr);
    gap: 24px;
    align-items: center;
    min-height: 82px;
    border-bottom: 1px solid rgba(34, 33, 31, 0.14);
  }

  .trust-facts span {
    color: #7a756f;
    font-size: 13px;
  }

  .trust-facts strong {
    color: #302e2b;
    font-size: 16px;
    font-weight: 650;
  }

  .trust-facts .checksum-row {
    grid-template-columns: minmax(120px, 0.55fr) minmax(0, 1fr) auto;
  }

  .checksum-row code {
    overflow: hidden;
    color: #4f4b46;
    font-family: 'Cascadia Mono', 'SFMono-Regular', Consolas, monospace;
    font-size: 12px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .checksum-row button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    min-width: 78px;
    min-height: 38px;
    padding: 0 13px;
    border: 1px solid rgba(34, 33, 31, 0.16);
    border-radius: 10px;
    background: transparent;
    color: #34312e;
    cursor: pointer;
    font-size: 13px;
    font-weight: 650;
    transition: transform 180ms ease, border-color 180ms ease, background 180ms ease;
  }

  .checksum-row button:hover {
    border-color: rgba(189, 79, 51, 0.55);
    background: rgba(189, 79, 51, 0.08);
  }

  .checksum-row button:active {
    border-color: rgba(189, 79, 51, 0.65);
  }

  .checksum-status {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    min-width: 58px;
  }

  .story-section {
    display: grid;
    grid-template-columns: minmax(260px, 0.72fr) minmax(0, 1.28fr);
    gap: 8vw;
    align-items: end;
  }

  .story-copy {
    padding-bottom: 12px;
  }

  .story-copy h2,
  .format-heading h2,
  .feature-copy h2,
  .control-heading h2,
  .faq-section h2,
  .final-cta h2 {
    margin-bottom: 24px;
    font-size: clamp(40px, 4.25vw, 66px);
    line-height: 1.05;
  }

  .story-copy p,
  .format-heading > p:not(.eyebrow),
  .feature-copy p,
  .control-heading p {
    max-width: 510px;
    margin-bottom: 0;
    color: #68645f;
    font-size: 17px;
    line-height: 1.75;
  }

  .story-visual {
    position: relative;
    margin: 0;
  }

  .story-visual img {
    display: block;
    width: 100%;
    height: auto;
  }

  .format-section {
    border-top: 1px solid rgba(34, 33, 31, 0.12);
  }

  .format-heading {
    display: grid;
    grid-template-columns: minmax(0, 1.15fr) minmax(280px, 0.85fr);
    column-gap: 8vw;
    align-items: end;
    margin-bottom: 60px;
  }

  .format-heading .eyebrow {
    grid-column: 1 / -1;
    margin-bottom: 22px;
  }

  .format-heading h2 {
    max-width: 820px;
    margin-bottom: 0;
  }

  .format-heading > p:not(.eyebrow) {
    margin-bottom: 4px;
  }

  .format-list {
    grid-column: 1 / -1;
    display: flex;
    flex-wrap: wrap;
    gap: 12px 0;
    margin-top: 42px;
    color: #514e4a;
    font-size: 13px;
    font-weight: 650;
    letter-spacing: 0.04em;
  }

  .format-list span {
    padding: 0 20px;
    border-left: 1px solid rgba(34, 33, 31, 0.18);
  }

  .format-list span:first-child {
    padding-left: 0;
    border-left: 0;
  }

  .format-shot {
    width: min(100%, 1240px);
    margin: 0 auto;
  }

  .feature-row {
    display: grid;
    grid-template-columns: minmax(280px, 0.65fr) minmax(0, 1.35fr);
    gap: 7vw;
    align-items: center;
    border-top: 1px solid rgba(34, 33, 31, 0.12);
  }

  .feature-copy > :global(svg) {
    margin-bottom: 30px;
  }

  .app-shot {
    margin: 0;
  }

  .feature-vault {
    grid-template-columns: minmax(0, 1.35fr) minmax(280px, 0.65fr);
  }

  .control-section {
    border-top: 1px solid rgba(34, 33, 31, 0.12);
  }

  .control-heading {
    max-width: 780px;
    margin: 0 auto 64px;
    text-align: center;
  }

  .control-heading p {
    margin-right: auto;
    margin-left: auto;
  }

  .settings-shot {
    width: min(100%, 1180px);
    margin: 0 auto;
  }

  .faq-section {
    display: grid;
    grid-template-columns: 0.7fr 1.3fr;
    gap: 9vw;
    border-top: 1px solid rgba(34, 33, 31, 0.12);
  }

  .faq-section h2 {
    position: sticky;
    top: 116px;
    align-self: start;
  }

  .faq-list details {
    border-bottom: 1px solid rgba(34, 33, 31, 0.16);
  }

  .faq-list details:first-child {
    border-top: 1px solid rgba(34, 33, 31, 0.16);
  }

  .faq-list summary {
    position: relative;
    padding: 28px 46px 28px 0;
    cursor: pointer;
    list-style: none;
    font-family: Georgia, 'Times New Roman', 'Songti SC', serif;
    font-size: 22px;
  }

  .faq-list summary::-webkit-details-marker {
    display: none;
  }

  .faq-list summary::after {
    content: '+';
    position: absolute;
    top: 24px;
    right: 4px;
    color: #bd4f33;
    font-family: 'Segoe UI', sans-serif;
    font-size: 26px;
    font-weight: 300;
    transition: transform 200ms ease;
  }

  .faq-list details[open] summary::after {
    transform: rotate(45deg);
  }

  .faq-list p {
    max-width: 620px;
    margin-bottom: 28px;
    color: #68645f;
    line-height: 1.7;
  }

  .final-cta {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 48px;
    margin: 20px 4.5vw 0;
    padding: 88px 7vw;
    border-radius: 14px;
    background: #2b2926;
    color: #f6f2ea;
  }

  .final-cta h2 {
    margin-bottom: 18px;
  }

  .final-cta p {
    margin-bottom: 0;
    color: #c8c1b8;
    font-size: 17px;
  }

  .light-button {
    flex: 0 0 auto;
    background: #f4eee4;
    color: #292724;
    box-shadow: none;
  }

  .light-button:hover {
    color: #fff8f2;
  }

  footer {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 24px;
    min-height: 156px;
    padding: 36px 4.5vw;
    color: #6d6964;
    font-size: 13px;
  }

  .footer-brand {
    color: #22211f;
  }

  footer p {
    margin: 0;
  }

  footer p:last-child {
    justify-self: end;
  }

  .footer-links {
    display: flex;
    gap: 18px;
  }

  :global(:focus-visible) {
    outline: 3px solid rgba(189, 79, 51, 0.42);
    outline-offset: 4px;
  }

  @media (max-width: 1080px) {
    .hero {
      grid-template-columns: 0.9fr 1.1fr;
    }

  }

  @media (max-width: 820px) {
    .site-header {
      grid-template-columns: 1fr auto;
      min-height: 64px;
      padding: 0 20px;
    }

    .desktop-nav,
    .header-download {
      display: none;
    }

    .menu-button {
      display: grid;
      width: 42px;
      height: 42px;
      place-items: center;
      border: 1px solid rgba(34, 33, 31, 0.14);
      border-radius: 12px;
      background: transparent;
      color: #22211f;
    }

    .mobile-nav {
      position: fixed;
      inset: 64px 0 auto;
      z-index: 19;
      display: grid;
      gap: 0;
      padding: 10px 20px 20px;
      border-bottom: 1px solid rgba(34, 33, 31, 0.13);
      background: #f7f5f0;
      box-shadow: 0 18px 36px rgba(56, 43, 35, 0.09);
    }

    .mobile-nav a {
      padding: 16px 0;
      border-bottom: 1px solid rgba(34, 33, 31, 0.1);
      text-decoration: none;
    }

    .hero {
      grid-template-columns: 1fr;
      gap: 54px;
      min-height: auto;
      padding: 64px 20px 76px;
    }

    .hero-copy {
      padding-right: 0;
    }

    h1 {
      font-size: clamp(46px, 11vw, 68px);
    }

    .hero-product {
      width: 100%;
      margin: 0;
      transform: none;
    }

    .fact-strip {
      grid-template-columns: repeat(2, 1fr);
      margin: 0 20px;
    }

    .fact-strip div:nth-child(3) {
      border-left: 0;
    }

    .fact-strip div:nth-child(n + 3) {
      border-top: 1px solid rgba(34, 33, 31, 0.12);
    }

    .story-section,
    .trust-section,
    .format-section,
    .feature-row,
    .feature-vault,
    .faq-section {
      grid-template-columns: 1fr;
      gap: 48px;
      padding: 96px 20px;
    }

    .trust-section {
      grid-template-columns: 1fr;
    }

    .story-section {
      align-items: start;
    }

    .format-heading {
      grid-template-columns: 1fr;
      gap: 20px;
      margin-bottom: 44px;
    }

    .format-heading .eyebrow,
    .format-list {
      grid-column: auto;
    }

    .format-heading h2 {
      margin-bottom: 4px;
    }

    .format-list {
      margin-top: 18px;
    }

    .feature-vault .feature-copy {
      order: -1;
    }

    .feature-images .app-shot,
    .feature-vault .app-shot {
      width: 100%;
      margin: 0;
    }

    .control-section {
      padding: 96px 20px;
    }

    .control-heading {
      margin-bottom: 44px;
      text-align: left;
    }

    .faq-section h2 {
      position: static;
    }

    .final-cta {
      align-items: flex-start;
      flex-direction: column;
      margin: 0 20px;
      padding: 64px 28px;
    }

    footer {
      grid-template-columns: 1fr;
      padding: 54px 20px;
    }

    footer p:last-child {
      justify-self: start;
    }
  }

  @media (max-width: 520px) {
    .brand {
      font-size: 21px;
    }

    .brand-mark {
      width: 31px;
      height: 31px;
    }

    .hero {
      gap: 42px;
      padding-top: 52px;
    }

    .eyebrow {
      margin-bottom: 18px;
    }

    h1 {
      font-size: 45px;
    }

    .hero-lede {
      font-size: 16px;
      line-height: 1.65;
    }

    .hero-actions {
      align-items: flex-start;
      flex-direction: column;
      gap: 20px;
    }

    .story-copy h2,
    .format-heading h2,
    .feature-copy h2,
    .control-heading h2,
    .faq-section h2,
    .final-cta h2 {
      font-size: 39px;
    }

    .story-section,
    .trust-section,
    .format-section,
    .feature-row,
    .feature-vault,
    .control-section,
    .faq-section {
      padding-top: 78px;
      padding-bottom: 78px;
    }

    .trust-facts > div,
    .trust-facts .checksum-row {
      grid-template-columns: 1fr auto;
      gap: 8px 16px;
      min-height: 76px;
      padding: 16px 0;
    }

    .checksum-row code {
      grid-column: 1 / -1;
      grid-row: 2;
    }

    .checksum-row button {
      grid-column: 2;
      grid-row: 1;
    }

    .format-list span {
      padding: 0 12px;
    }

    .faq-list summary {
      font-size: 19px;
    }

    .final-cta {
      padding: 52px 24px;
    }
  }

  @media (prefers-color-scheme: dark) {
    :global(html),
    :global(body) {
      background: #201f1d;
      color: #f1eee8;
    }

    .site-shell {
      background:
        radial-gradient(circle at 84% 5%, rgba(205, 96, 65, 0.1), transparent 22rem),
        #201f1d;
    }

    .site-header,
    .mobile-nav {
      border-color: rgba(255, 255, 255, 0.11);
      background: rgba(32, 31, 29, 0.9);
    }

    .desktop-nav,
    .hero-lede,
    .story-copy p,
    .trust-copy p,
    .format-heading > p:not(.eyebrow),
    .feature-copy p,
    .control-heading p,
    .faq-list p,
    footer {
      color: #bdb7af;
    }

    .trust-links a,
    .trust-facts strong,
    .checksum-row code,
    .checksum-row button {
      color: #ebe6de;
    }

    .brand,
    .menu-button,
    .footer-brand {
      color: #f1eee8;
    }

    .brand-mark,
    .menu-button,
    .fact-strip,
    .fact-strip div + div,
    .fact-strip div:nth-child(n + 3),
    .format-section,
    .feature-row,
    .control-section,
    .faq-section,
    .faq-list details,
    .faq-list details:first-child,
    .trust-facts,
    .trust-facts > div,
    .checksum-row button {
      border-color: rgba(255, 255, 255, 0.12);
    }

    .trust-facts span {
      color: #a9a39b;
    }

    .fact-strip div {
      color: #d4cec6;
    }

    .interactive-media {
      border-color: rgba(255, 255, 255, 0.12);
      background: #302e2a;
      box-shadow: 0 32px 90px rgba(0, 0, 0, 0.28);
    }

    .format-list {
      color: #d4cec6;
    }

    .format-list span {
      border-color: rgba(255, 255, 255, 0.16);
    }

    .primary-button,
    .header-download {
      background: #ede8df;
      color: #24221f;
    }

    .primary-button:hover,
    .header-download:hover {
      background: #c85b3e;
      color: #fff9f4;
    }

    .final-cta {
      background: #2d2b28;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    :global(html) {
      scroll-behavior: auto;
    }

    *,
    *::before,
    *::after {
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
      transition-duration: 0.01ms !important;
    }

    .interactive-media {
      opacity: 1;
      transform: none;
    }

    .interactive-media::after {
      display: none;
    }
  }
</style>
