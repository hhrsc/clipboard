<script lang="ts">
  import { onMount } from 'svelte';
  import {
    IconArrowLeft as ArrowLeft,
    IconArrowRight as ArrowRight,
    IconBrandGithub as BrandGithub,
    IconCheck as Check,
    IconClipboard as Clipboard,
    IconExternalLink as ExternalLink,
    IconSearch as Search,
    IconShieldLock as ShieldLock,
    IconSparkles as Sparkles
  } from '@tabler/icons-svelte';

  const downloadHref =
    'https://github.com/hhrsc/clipboard/releases/download/v0.1.0/Clipboard_0.1.0_x64-setup.exe';
  const sourceHref = 'https://github.com/hhrsc/clipboard';
  const feedbackHref = 'https://github.com/hhrsc/clipboard/issues/new/choose';
  const microsoftHref = 'https://support.microsoft.com/zh-cn/windows/apps/using-the-clipboard';
  const canonicalHref = 'https://hhrsc.github.io/website/guides/windows-clipboard-history/';
  const articleStructuredData = JSON.stringify({
    '@context': 'https://schema.org',
    '@type': 'Article',
    headline: 'Windows 剪贴板历史怎么用？Win + V 与 Clipboard 的选择指南',
    description: '先用好 Win + V，再判断是否需要搜索、分类、图片历史和多格式复制。',
    datePublished: '2026-09-02',
    dateModified: '2026-09-02',
    inLanguage: 'zh-CN',
    mainEntityOfPage: canonicalHref,
    author: { '@type': 'Organization', name: 'Clipboard' },
    publisher: { '@type': 'Organization', name: 'Clipboard' }
  });
  const breadcrumbStructuredData = JSON.stringify({
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: [
      { '@type': 'ListItem', position: 1, name: 'Clipboard 官网', item: 'https://hhrsc.github.io/website/' },
      { '@type': 'ListItem', position: 2, name: 'Windows 剪贴板指南', item: canonicalHref }
    ]
  });

  function interactiveMedia(node: HTMLElement) {
    const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    const coarsePointer = window.matchMedia('(pointer: coarse)').matches;
    if (reduceMotion || coarsePointer) return {};

    let frame = 0;
    const reset = () => {
      cancelAnimationFrame(frame);
      node.style.setProperty('--tilt-x', '0deg');
      node.style.setProperty('--tilt-y', '0deg');
      node.style.setProperty('--media-active', '0');
    };
    const move = (event: PointerEvent) => {
      const rect = node.getBoundingClientRect();
      const x = (event.clientX - rect.left) / rect.width;
      const y = (event.clientY - rect.top) / rect.height;
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(() => {
        node.style.setProperty('--tilt-x', `${(0.5 - y) * 3}deg`);
        node.style.setProperty('--tilt-y', `${(x - 0.5) * 4}deg`);
        node.style.setProperty('--spot-x', `${x * 100}%`);
        node.style.setProperty('--spot-y', `${y * 100}%`);
        node.style.setProperty('--media-active', '1');
      });
    };

    node.addEventListener('pointermove', move);
    node.addEventListener('pointerleave', reset);
    return {
      destroy() {
        node.removeEventListener('pointermove', move);
        node.removeEventListener('pointerleave', reset);
        cancelAnimationFrame(frame);
      }
    };
  }

  onMount(() => {
    const nodes = document.querySelectorAll<HTMLElement>('[data-reveal]');
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (!entry.isIntersecting) continue;
          entry.target.classList.add('is-visible');
          observer.unobserve(entry.target);
        }
      },
      { threshold: 0.12 }
    );
    nodes.forEach((node) => observer.observe(node));
    return () => observer.disconnect();
  });
</script>

<svelte:head>
  <title>Windows 剪贴板历史怎么用？Win + V 与 Clipboard 选择指南</title>
  <meta
    name="description"
    content="先用好 Windows 的 Win + V 剪贴板历史，再判断是否需要搜索、分类、图片历史和多格式复制。"
  />
  <link rel="canonical" href={canonicalHref} />
  <meta name="robots" content="index,follow" />
  <meta property="og:type" content="article" />
  <meta property="og:title" content="Win + V 不够用时，先看你真正缺什么" />
  <meta
    property="og:description"
    content="一份不劝你盲目安装的 Windows 剪贴板选择指南。"
  />
  <meta property="og:url" content={canonicalHref} />
  <meta property="og:image" content="https://hhrsc.github.io/marketing/clipboard-recent.webp" />
  {@html `<script type="application/ld+json">${articleStructuredData}<\/script>`}
  {@html `<script type="application/ld+json">${breadcrumbStructuredData}<\/script>`}
</svelte:head>

<div class="guide-shell" id="top">
  <header>
    <a class="brand" href="/website/" aria-label="返回 Clipboard 官网">
      <span class="brand-mark"><Clipboard size={19} stroke={1.7} /></span>
      Clipboard
    </a>
    <nav aria-label="页面导航">
      <a href="#compare">怎么选</a>
      <a href="#workflow">功能差异</a>
      <a class="back-link" href="/website/"><ArrowLeft size={17} stroke={1.8} />返回官网</a>
    </nav>
  </header>

  <main>
    <section class="hero">
      <div class="hero-copy" data-reveal>
        <p class="eyebrow">Windows 剪贴板指南</p>
        <h1>Win + V 不够用时，先看你真正缺什么。</h1>
        <p class="lede">原生历史够轻便，复杂工作流才需要搜索、分类和格式转换。</p>
        <div class="hero-actions">
          <a class="primary-button" href={downloadHref}>
            下载 Clipboard <ArrowRight size={18} stroke={1.8} />
          </a>
          <a class="text-link" href="#native">先看原生功能</a>
        </div>
        <p class="trust-line"><ShieldLock size={17} stroke={1.8} />本地优先，无需账户，当前版本不含云同步。</p>
      </div>

      <figure class="media-frame hero-media" use:interactiveMedia data-reveal>
        <div class="media-glow" aria-hidden="true"></div>
        <img
          src="/marketing/clipboard-recent.webp"
          alt="Clipboard 最近记录界面，展示可搜索的文字剪贴板历史"
          width="1600"
          height="1000"
        />
        <figcaption>从最近记录开始，不必重新寻找刚才复制过的内容。</figcaption>
      </figure>
    </section>

    <section class="native-section" id="native" data-reveal>
      <div class="section-heading">
        <p class="eyebrow">第一步</p>
        <h2>先用好 Windows 自带的 Win + V。</h2>
      </div>
      <div class="native-copy">
        <p>
          按 <kbd>Win</kbd> + <kbd>V</kbd> 可以打开剪贴板历史。它能保存最近复制的文字、HTML 和位图，也能固定常用项目，并可选择在设备之间同步。
        </p>
        <p>
          微软当前说明中，历史最多保留 25 项，单项上限为 4 MB。重启设备后，未固定的历史会被清除。
        </p>
        <a href={microsoftHref} target="_blank" rel="noreferrer">
          查看微软官方说明 <ExternalLink size={16} stroke={1.8} />
        </a>
      </div>
    </section>

    <section class="compare-section" id="compare">
      <div class="section-heading" data-reveal>
        <p class="eyebrow">怎么选</p>
        <h2>轻量使用留在原生，重复工作交给工具。</h2>
        <p>不是功能越多越好。判断标准只有一个：它能不能减少你每天重复寻找、整理和转换内容的时间。</p>
      </div>

      <div class="comparison" data-reveal>
        <div class="comparison-head" aria-hidden="true">
          <span>使用场景</span>
          <span>Win + V</span>
          <span>Clipboard</span>
        </div>
        <div class="comparison-row">
          <div><strong>临时找回刚复制的内容</strong><small>偶尔使用，记录不多</small></div>
          <span class="fit"><Check size={18} />够用</span>
          <span class="fit quiet">也支持</span>
        </div>
        <div class="comparison-row">
          <div><strong>按关键词快速搜索</strong><small>记录变多后不靠滚动找</small></div>
          <span class="limit">有限</span>
          <span class="fit"><Check size={18} />适合</span>
        </div>
        <div class="comparison-row">
          <div><strong>分类管理文字和图片</strong><small>把项目素材与日常片段分开</small></div>
          <span class="limit">不提供分类</span>
          <span class="fit"><Check size={18} />适合</span>
        </div>
        <div class="comparison-row">
          <div><strong>选取文字后转换格式</strong><small>复制为 Markdown、HTML 或单行文本</small></div>
          <span class="limit">不提供</span>
          <span class="fit"><Check size={18} />适合</span>
        </div>
      </div>
    </section>

    <section class="workflow-section" id="workflow">
      <div class="workflow-copy" data-reveal>
        <p class="eyebrow">值得安装的时刻</p>
        <h2>当“复制一下”开始变成一段工作流。</h2>
        <div class="feature-list">
          <article>
            <span><Search size={20} stroke={1.7} /></span>
            <div>
              <h3>需要搜索与分类</h3>
              <p>按内容、类型和日期缩小范围，把高频片段固定或归类。</p>
            </div>
          </article>
          <article>
            <span><Sparkles size={20} stroke={1.7} /></span>
            <div>
              <h3>需要多格式复制</h3>
              <p>选取记录中的一段文字，再复制为 Markdown、HTML 或规整的单行文本。</p>
            </div>
          </article>
          <article>
            <span><ShieldLock size={20} stroke={1.7} /></span>
            <div>
              <h3>希望内容留在本机</h3>
              <p>当前版本无需账户，不提供云同步。历史与本地密码库由你自己控制。</p>
            </div>
          </article>
        </div>
      </div>

      <figure class="media-frame workflow-media" use:interactiveMedia data-reveal>
        <div class="media-glow" aria-hidden="true"></div>
        <img
          src="/marketing/clipboard-images.webp"
          alt="Clipboard 图片记录界面，完整展示图片历史与分类区域"
          width="1600"
          height="1000"
        />
        <figcaption>图片完整显示，移动鼠标时界面会轻微响应。</figcaption>
      </figure>
    </section>

    <section class="decision-note" data-reveal>
      <p class="eyebrow">一句话结论</p>
      <blockquote>
        如果你只想找回刚刚复制的几条内容，先用 Win + V。<br />如果你每天都在找、分、改、重复复制，Clipboard 才会真正省时间。
      </blockquote>
    </section>

    <section class="final-cta" data-reveal>
      <div>
        <h2>先试用，再决定<br />它值不值得留下。</h2>
        <p>Windows 64 位安装包。版本、源码与校验值全部公开。</p>
      </div>
      <div class="final-actions">
        <a class="primary-button light-button" href={downloadHref}>
          下载 Clipboard <ArrowRight size={18} stroke={1.8} />
        </a>
        <a href={sourceHref} target="_blank" rel="noreferrer">
          <BrandGithub size={18} stroke={1.8} />查看源码
        </a>
      </div>
    </section>
  </main>

  <footer>
    <a class="brand footer-brand" href="#top" aria-label="返回顶部">
      <span class="brand-mark"><Clipboard size={19} stroke={1.7} /></span>
      Clipboard
    </a>
    <p>一款安静、直接的 Windows 本地剪贴板工具。</p>
    <p class="footer-links">
      <a href="/website/guides/copy-text-as-markdown-html/">多格式复制指南</a>
      <a href="/website/privacy-policy/">隐私说明</a>
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

  :global(a) {
    color: inherit;
  }

  .guide-shell {
    overflow: hidden;
    background:
      radial-gradient(circle at 88% 4%, rgba(189, 79, 51, 0.13), transparent 28rem),
      #f7f5f0;
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
  nav,
  .back-link,
  .hero-actions,
  .primary-button,
  .trust-line,
  .native-copy a,
  .fit,
  .final-actions,
  .final-actions > a,
  .footer-links {
    display: flex;
    align-items: center;
  }

  .brand {
    gap: 10px;
    font-family: Georgia, 'Times New Roman', 'Songti SC', serif;
    font-size: 23px;
    font-weight: 600;
    text-decoration: none;
  }

  .brand-mark {
    display: grid;
    width: 32px;
    height: 32px;
    place-items: center;
    border: 1px solid rgba(34, 33, 31, 0.14);
    border-radius: 10px;
  }

  nav {
    gap: 28px;
    font-size: 14px;
    font-weight: 650;
  }

  nav a {
    text-decoration: none;
  }

  .back-link {
    gap: 7px;
  }

  main {
    padding-top: 46px;
  }

  .hero,
  .native-section,
  .compare-section,
  .workflow-section,
  .decision-note {
    width: min(100% - 9vw, 1180px);
    margin-inline: auto;
  }

  .hero {
    display: grid;
    grid-template-columns: minmax(0, 0.9fr) minmax(440px, 1.1fr);
    align-items: center;
    gap: clamp(48px, 7vw, 100px);
    min-height: 700px;
    padding: 50px 0 104px;
  }

  .eyebrow {
    margin: 0 0 22px;
    color: #bd4f33;
    font-size: 12px;
    font-weight: 750;
    letter-spacing: 0.16em;
    text-transform: uppercase;
  }

  h1,
  h2,
  h3,
  p,
  figure,
  blockquote {
    margin-top: 0;
  }

  h1,
  h2,
  blockquote {
    font-family: Georgia, 'Times New Roman', 'Songti SC', serif;
    font-weight: 500;
  }

  h1 {
    max-width: 650px;
    margin-bottom: 28px;
    font-size: clamp(54px, 6vw, 86px);
    line-height: 0.98;
    letter-spacing: -0.055em;
  }

  .lede {
    max-width: 580px;
    margin-bottom: 34px;
    color: #68645f;
    font-size: 18px;
    line-height: 1.75;
  }

  .hero-actions {
    gap: 22px;
    margin-bottom: 28px;
  }

  .primary-button {
    justify-content: center;
    gap: 10px;
    min-height: 50px;
    padding: 0 22px;
    border-radius: 10px;
    background: #bd4f33;
    color: #fffaf4;
    font-size: 15px;
    font-weight: 720;
    text-decoration: none;
    transition: transform 180ms ease, background 180ms ease;
  }

  .primary-button:hover {
    transform: translateY(-2px);
    background: #a94229;
  }

  .text-link {
    font-weight: 680;
    text-underline-offset: 5px;
  }

  .trust-line {
    gap: 8px;
    margin: 0;
    color: #76716b;
    font-size: 13px;
  }

  .media-frame {
    --tilt-x: 0deg;
    --tilt-y: 0deg;
    --spot-x: 50%;
    --spot-y: 50%;
    --media-active: 0;
    position: relative;
    isolation: isolate;
    margin-bottom: 0;
    padding: 22px 22px 16px;
    border: 1px solid rgba(34, 33, 31, 0.13);
    border-radius: 16px;
    background: rgba(250, 248, 243, 0.72);
    box-shadow: 0 30px 80px rgba(62, 43, 31, 0.1);
    transform: perspective(1100px) rotateX(var(--tilt-x)) rotateY(var(--tilt-y));
    transition: transform 180ms ease, box-shadow 240ms ease;
  }

  .media-frame:hover {
    box-shadow: 0 36px 94px rgba(62, 43, 31, 0.16);
  }

  .media-glow {
    position: absolute;
    z-index: -1;
    inset: 0;
    border-radius: inherit;
    background: radial-gradient(circle at var(--spot-x) var(--spot-y), rgba(189, 79, 51, calc(0.16 * var(--media-active))), transparent 44%);
    pointer-events: none;
  }

  .media-frame img {
    display: block;
    width: 100%;
    height: auto;
    border: 1px solid rgba(34, 33, 31, 0.12);
    border-radius: 10px;
    object-fit: contain;
  }

  figcaption {
    margin-top: 14px;
    color: #77716b;
    font-size: 12px;
    line-height: 1.5;
  }

  .native-section {
    display: grid;
    grid-template-columns: minmax(0, 0.9fr) minmax(0, 1.1fr);
    gap: 8vw;
    padding: 112px 0;
    border-top: 1px solid rgba(34, 33, 31, 0.14);
    border-bottom: 1px solid rgba(34, 33, 31, 0.14);
  }

  h2 {
    margin-bottom: 22px;
    font-size: clamp(38px, 4.6vw, 64px);
    line-height: 1.06;
    letter-spacing: -0.04em;
  }

  .native-copy p,
  .section-heading > p:last-child,
  .feature-list p,
  .final-cta p {
    color: #68645f;
    line-height: 1.75;
  }

  .native-copy p {
    margin-bottom: 18px;
    font-size: 17px;
  }

  kbd {
    padding: 2px 7px 3px;
    border: 1px solid rgba(34, 33, 31, 0.18);
    border-bottom-width: 2px;
    border-radius: 5px;
    background: #ede9e1;
    font: 650 13px 'Segoe UI', sans-serif;
  }

  .native-copy a {
    gap: 7px;
    width: fit-content;
    margin-top: 28px;
    font-weight: 700;
    text-underline-offset: 5px;
  }

  .compare-section {
    padding: 132px 0;
  }

  .compare-section .section-heading {
    display: grid;
    grid-template-columns: 0.8fr 1.2fr;
    column-gap: 8vw;
    margin-bottom: 62px;
  }

  .compare-section .section-heading .eyebrow {
    grid-column: 1;
  }

  .compare-section .section-heading h2 {
    grid-column: 1;
  }

  .compare-section .section-heading > p:last-child {
    grid-column: 2;
    grid-row: 1 / span 2;
    align-self: end;
    max-width: 510px;
    margin-bottom: 22px;
    font-size: 17px;
  }

  .comparison {
    border-top: 1px solid #34312e;
  }

  .comparison-head,
  .comparison-row {
    display: grid;
    grid-template-columns: minmax(0, 1.7fr) minmax(120px, 0.65fr) minmax(120px, 0.65fr);
    align-items: center;
    gap: 24px;
  }

  .comparison-head {
    padding: 16px 22px;
    color: #77716b;
    font-size: 12px;
    font-weight: 750;
    letter-spacing: 0.08em;
  }

  .comparison-row {
    min-height: 104px;
    padding: 20px 22px;
    border-top: 1px solid rgba(34, 33, 31, 0.12);
  }

  .comparison-row:last-child {
    border-bottom: 1px solid rgba(34, 33, 31, 0.12);
  }

  .comparison-row strong,
  .comparison-row small {
    display: block;
  }

  .comparison-row strong {
    margin-bottom: 7px;
    font-size: 16px;
  }

  .comparison-row small {
    color: #77716b;
    font-size: 13px;
  }

  .fit {
    gap: 7px;
    color: #2f6e52;
    font-weight: 700;
  }

  .fit.quiet,
  .limit {
    color: #77716b;
    font-size: 14px;
    font-weight: 600;
  }

  .workflow-section {
    display: grid;
    grid-template-columns: minmax(0, 0.9fr) minmax(440px, 1.1fr);
    align-items: center;
    gap: clamp(56px, 8vw, 112px);
    padding: 120px 0 132px;
  }

  .feature-list {
    margin-top: 42px;
    border-top: 1px solid rgba(34, 33, 31, 0.14);
  }

  .feature-list article {
    display: grid;
    grid-template-columns: 42px 1fr;
    gap: 18px;
    padding: 24px 0;
    border-bottom: 1px solid rgba(34, 33, 31, 0.14);
  }

  .feature-list article > span {
    display: grid;
    width: 38px;
    height: 38px;
    place-items: center;
    border: 1px solid rgba(34, 33, 31, 0.14);
    border-radius: 10px;
    color: #bd4f33;
  }

  h3 {
    margin: 0 0 7px;
    font-size: 16px;
  }

  .feature-list p {
    margin-bottom: 0;
    font-size: 14px;
  }

  .decision-note {
    padding: 118px 0;
    border-top: 1px solid rgba(34, 33, 31, 0.14);
  }

  blockquote {
    max-width: 980px;
    margin-bottom: 0;
    font-size: clamp(34px, 4.8vw, 62px);
    line-height: 1.18;
    letter-spacing: -0.035em;
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
    font-size: 16px;
  }

  .final-actions {
    flex: 0 0 auto;
    flex-direction: column;
    align-items: stretch;
    gap: 16px;
  }

  .final-actions > a {
    justify-content: center;
    gap: 8px;
    font-weight: 700;
    text-underline-offset: 5px;
  }

  .light-button {
    background: #f4eee4;
    color: #292724;
  }

  .light-button:hover {
    background: #fffaf4;
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
    gap: 18px;
  }

  [data-reveal] {
    opacity: 0;
    transform: translateY(24px);
    transition: opacity 700ms cubic-bezier(0.16, 1, 0.3, 1), transform 700ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  [data-reveal]:global(.is-visible) {
    opacity: 1;
    transform: translateY(0);
  }

  .media-frame[data-reveal],
  .media-frame[data-reveal]:global(.is-visible) {
    transform: perspective(1100px) translateY(calc(24px * (1 - var(--media-active)))) rotateX(var(--tilt-x)) rotateY(var(--tilt-y));
  }

  .media-frame[data-reveal]:global(.is-visible) {
    transform: perspective(1100px) rotateX(var(--tilt-x)) rotateY(var(--tilt-y));
  }

  :global(:focus-visible) {
    outline: 3px solid rgba(189, 79, 51, 0.42);
    outline-offset: 4px;
  }

  @media (max-width: 940px) {
    .hero,
    .workflow-section {
      grid-template-columns: 1fr;
    }

    .hero {
      min-height: 0;
      padding-top: 72px;
    }

    .hero-copy {
      max-width: 760px;
    }

    .hero-media,
    .workflow-media {
      width: min(100%, 760px);
    }

    .workflow-copy {
      max-width: 720px;
    }
  }

  @media (max-width: 720px) {
    header {
      min-height: 64px;
      padding: 0 20px;
    }

    nav > a:not(.back-link) {
      display: none;
    }

    .back-link {
      font-size: 0;
    }

    main {
      padding-top: 18px;
    }

    .hero,
    .native-section,
    .compare-section,
    .workflow-section,
    .decision-note {
      width: auto;
      margin-inline: 20px;
    }

    .hero {
      gap: 52px;
      padding: 54px 0 80px;
    }

    h1 {
      font-size: clamp(47px, 14vw, 66px);
    }

    .hero-actions {
      align-items: flex-start;
      flex-direction: column;
    }

    .media-frame {
      padding: 12px 12px 10px;
    }

    .native-section {
      grid-template-columns: 1fr;
      gap: 32px;
      padding: 82px 0;
    }

    .compare-section,
    .workflow-section,
    .decision-note {
      padding: 88px 0;
    }

    .compare-section .section-heading {
      display: block;
      margin-bottom: 42px;
    }

    .compare-section .section-heading > p:last-child {
      margin-top: 24px;
    }

    .comparison {
      overflow-x: auto;
    }

    .comparison-head,
    .comparison-row {
      grid-template-columns: minmax(220px, 1.6fr) minmax(110px, 0.7fr) minmax(110px, 0.7fr);
      min-width: 540px;
    }

    .comparison-head,
    .comparison-row {
      padding-inline: 12px;
    }

    .workflow-section {
      gap: 56px;
    }

    blockquote br {
      display: none;
    }

    .final-cta {
      align-items: flex-start;
      flex-direction: column;
      margin-inline: 12px;
      padding: 64px 28px;
    }

    .final-actions {
      width: 100%;
    }

    footer {
      grid-template-columns: 1fr;
      justify-items: start;
      min-height: 0;
      padding: 54px 20px;
    }

    footer p:last-child {
      justify-self: start;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    :global(html) {
      scroll-behavior: auto;
    }

    [data-reveal] {
      opacity: 1;
      transform: none;
      transition: none;
    }

    .media-frame,
    .primary-button {
      transition: none;
    }
  }

  @media (prefers-color-scheme: dark) {
    :global(html),
    :global(body) {
      background: #201f1d;
      color: #f1eee8;
    }

    .guide-shell {
      background:
        radial-gradient(circle at 88% 4%, rgba(205, 96, 65, 0.1), transparent 28rem),
        #201f1d;
    }

    header,
    .brand-mark,
    .native-section,
    .comparison-row,
    .feature-list,
    .feature-list article,
    .feature-list article > span,
    .decision-note {
      border-color: rgba(255, 255, 255, 0.12);
    }

    .lede,
    .native-copy p,
    .section-heading > p:last-child,
    .feature-list p,
    .trust-line,
    .comparison-row small,
    .fit.quiet,
    .limit,
    figcaption,
    footer {
      color: #bdb7af;
    }

    .media-frame {
      border-color: rgba(255, 255, 255, 0.12);
      background: rgba(42, 40, 37, 0.76);
      box-shadow: 0 30px 80px rgba(0, 0, 0, 0.25);
    }

    .media-frame img,
    kbd {
      border-color: rgba(255, 255, 255, 0.13);
    }

    kbd {
      background: #32302d;
      color: #f1eee8;
    }

    .comparison {
      border-top-color: #d5cfc8;
    }

    .fit {
      color: #80bf9b;
    }

    .footer-brand {
      color: #f1eee8;
    }
  }
</style>
