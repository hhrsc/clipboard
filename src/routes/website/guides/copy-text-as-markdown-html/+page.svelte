<script lang="ts">
  import { onMount } from 'svelte';
  import {
    IconArrowLeft as ArrowLeft,
    IconArrowRight as ArrowRight,
    IconBrandGithub as BrandGithub,
    IconCheck as Check,
    IconClipboard as Clipboard,
    IconCode as Code,
    IconExternalLink as ExternalLink,
    IconMarkdown as Markdown,
    IconPilcrow as Pilcrow,
    IconShieldLock as ShieldLock
  } from '@tabler/icons-svelte';

  const downloadHref =
    'https://github.com/hhrsc/clipboard/releases/download/v0.1.0/Clipboard_0.1.0_x64-setup.exe';
  const sourceHref = 'https://github.com/hhrsc/clipboard';
  const feedbackHref = 'https://github.com/hhrsc/clipboard/issues/new/choose';
  const windowsClipboardHref = 'https://support.microsoft.com/zh-cn/windows/apps/using-the-clipboard';
  const powerToysHref = 'https://learn.microsoft.com/zh-cn/windows/powertoys/advanced-paste';
  const canonicalHref = 'https://hhrsc.github.io/website/guides/copy-text-as-markdown-html/';
  const articleStructuredData = JSON.stringify({
    '@context': 'https://schema.org',
    '@type': 'Article',
    headline: 'Windows 如何把文字复制为 Markdown、HTML 或纯文本？',
    description: '比较 Win + V、PowerToys Advanced Paste 与 Clipboard，选择适合自己的文字复制方式。',
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
      { '@type': 'ListItem', position: 2, name: 'Windows 剪贴板指南', item: 'https://hhrsc.github.io/website/guides/windows-clipboard-history/' },
      { '@type': 'ListItem', position: 3, name: '多格式复制指南', item: canonicalHref }
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
  <title>Windows 如何把文字复制为 Markdown、HTML 或纯文本？</title>
  <meta
    name="description"
    content="比较 Win + V、PowerToys Advanced Paste 与 Clipboard，选择适合自己的 Markdown、HTML、纯文本和单行文本复制方式。"
  />
  <link rel="canonical" href={canonicalHref} />
  <meta name="robots" content="index,follow" />
  <meta property="og:type" content="article" />
  <meta property="og:title" content="把复制的文字，带成你要的格式" />
  <meta property="og:description" content="只选需要的句子，再复制为 Markdown、HTML、纯文本或单行文本。" />
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
      <a href="#choose">怎么选</a>
      <a href="#workflow">怎么用</a>
      <a class="back-link" href="/website/"><ArrowLeft size={17} stroke={1.8} />返回官网</a>
    </nav>
  </header>

  <main>
    <section class="hero">
      <div class="hero-copy" data-reveal>
        <h1>把复制的文字，带成你要的格式。</h1>
        <p class="lede">只选需要的句子，再复制为纯文本、Markdown、HTML 或单行文本。</p>
        <div class="hero-actions">
          <a class="primary-button" href={downloadHref}>
            下载 Clipboard <ArrowRight size={18} stroke={1.8} />
          </a>
          <a class="text-link" href="#choose">先看怎么选</a>
        </div>
      </div>

      <figure class="media-frame hero-media" use:interactiveMedia data-reveal>
        <div class="media-glow" aria-hidden="true"></div>
        <img
          src="/marketing/clipboard-recent.webp"
          alt="Clipboard 文字详情界面，展示复制选中文字、Markdown、HTML 和去换行操作"
          width="1600"
          height="1000"
        />
        <figcaption>保留整条记录，需要粘贴时只取其中一段。</figcaption>
      </figure>
    </section>

    <section class="source-note" data-reveal>
      <div>
        <h2>先分清两个问题。</h2>
        <p>你是只想转换当前剪贴板，还是还需要回找、搜索和整理历史内容？答案不同，合适的工具也不同。</p>
      </div>
      <div class="source-links">
        <a href={windowsClipboardHref} target="_blank" rel="noreferrer">
          Windows 剪贴板说明 <ExternalLink size={16} stroke={1.8} />
        </a>
        <a href={powerToysHref} target="_blank" rel="noreferrer">
          PowerToys Advanced Paste <ExternalLink size={16} stroke={1.8} />
        </a>
      </div>
    </section>

    <section class="choose-section" id="choose">
      <div class="section-heading" data-reveal>
        <p class="eyebrow">按需求选择</p>
        <h2>不是每个人都需要再装一个剪贴板工具。</h2>
        <p>偶尔找回内容，用 Win + V。只做格式转换，PowerToys 很合适。历史越来越多，再考虑 Clipboard。</p>
      </div>

      <div class="comparison" data-reveal>
        <div class="comparison-head" aria-hidden="true">
          <span>你的需求</span>
          <span>更合适的方式</span>
          <span>原因</span>
        </div>
        <div class="comparison-row">
          <div><strong>找回最近复制的内容</strong><small>固定常用项，或在设备间同步</small></div>
          <span class="method">Win + V</span>
          <span class="reason"><Check size={18} />系统原生，开箱即用</span>
        </div>
        <div class="comparison-row">
          <div><strong>把当前内容换一种格式粘贴</strong><small>纯文本、Markdown、JSON 或文件</small></div>
          <span class="method">PowerToys</span>
          <span class="reason"><Check size={18} />转换范围更广</span>
        </div>
        <div class="comparison-row">
          <div><strong>回找历史，再选一段文字转换</strong><small>搜索、分类、置顶与多格式复制</small></div>
          <span class="method accent">Clipboard</span>
          <span class="reason"><Check size={18} />围绕历史整理</span>
        </div>
      </div>
    </section>

    <section class="workflow-section" id="workflow">
      <figure class="media-frame workflow-media" use:interactiveMedia data-reveal>
        <div class="media-glow" aria-hidden="true"></div>
        <img
          src="/marketing/clipboard-recent.webp"
          alt="Clipboard 最近记录与选中文字区域的完整界面"
          width="1600"
          height="1000"
        />
        <figcaption>图片完整展示，鼠标移动时只做轻微响应。</figcaption>
      </figure>

      <div class="workflow-copy" data-reveal>
        <h2>保留原文，粘贴时再决定格式。</h2>
        <div class="format-list">
          <article>
            <span><Markdown size={21} stroke={1.6} /></span>
            <div>
              <h3>复制为 Markdown</h3>
              <p>把网页中的标题、链接和列表转成适合笔记与文档的 Markdown。</p>
            </div>
          </article>
          <article>
            <span><Code size={21} stroke={1.6} /></span>
            <div>
              <h3>复制为 HTML</h3>
              <p>将可用的富文本结构写回剪贴板，粘贴到支持 HTML 的应用。</p>
            </div>
          </article>
          <article>
            <span><Pilcrow size={21} stroke={1.6} /></span>
            <div>
              <h3>复制为单行文本</h3>
              <p>去掉换行，适合表格单元格、搜索框、命令参数和表单输入。</p>
            </div>
          </article>
        </div>
      </div>
    </section>

    <section class="steps-section" data-reveal>
      <div class="step-intro">
        <h2>只做三件事。</h2>
        <p>打开记录，框选真正需要的文字，点击目标格式。原始记录不会因为这次转换而改变。</p>
      </div>
      <ol>
        <li><strong>找到记录</strong><span>用关键词、类型或分类缩小范围。</span></li>
        <li><strong>选中文字</strong><span>一条长记录里，只框选这次要用的句子。</span></li>
        <li><strong>选择格式</strong><span>复制后直接粘贴到笔记、编辑器或表格。</span></li>
      </ol>
    </section>

    <section class="privacy-note" data-reveal>
      <ShieldLock size={28} stroke={1.6} />
      <div>
        <h2>格式转换在本机完成。</h2>
        <p>Clipboard 当前无需账户，也不提供云同步。剪贴板历史保存在本机，源码公开可查。</p>
      </div>
      <a href="/website/privacy-policy/">查看隐私说明 <ArrowRight size={17} stroke={1.8} /></a>
    </section>

    <section class="final-cta" data-reveal>
      <div>
        <h2>让复制停在<br />你真正要用的地方。</h2>
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
      <a href="/website/guides/windows-clipboard-history/">Win + V 指南</a>
      <a href={feedbackHref} target="_blank" rel="noreferrer">反馈</a>
    </p>
  </footer>
</div>

<style>
  :global(*) { box-sizing: border-box; }
  :global(html) { scroll-behavior: smooth; background: #f7f5f0; }
  :global(body) {
    margin: 0;
    background: #f7f5f0;
    color: #22211f;
    font-family: 'Segoe UI Variable Text', 'Segoe UI', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  }
  :global(a) { color: inherit; }

  .guide-shell {
    overflow: hidden;
    background: radial-gradient(circle at 88% 4%, rgba(189, 79, 51, 0.13), transparent 28rem), #f7f5f0;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 72px;
    padding: 0 4.5vw;
    border-bottom: 1px solid rgba(34, 33, 31, 0.12);
  }

  .brand, nav, .back-link, .hero-actions, .primary-button, .source-links,
  .source-links a, .reason, .privacy-note, .privacy-note a, .final-actions,
  .final-actions > a, .footer-links { display: flex; align-items: center; }

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
  nav { gap: 28px; font-size: 14px; font-weight: 650; }
  nav a { text-decoration: none; }
  .back-link { gap: 7px; }
  main { padding-top: 46px; }

  .hero, .source-note, .choose-section, .workflow-section, .steps-section, .privacy-note {
    width: min(100% - 9vw, 1180px);
    margin-inline: auto;
  }
  .hero {
    display: grid;
    grid-template-columns: minmax(0, 0.88fr) minmax(440px, 1.12fr);
    align-items: center;
    gap: clamp(48px, 7vw, 100px);
    min-height: 700px;
    padding: 50px 0 104px;
  }
  h1, h2, h3, p, figure { margin-top: 0; }
  h1, h2 { font-family: Georgia, 'Times New Roman', 'Songti SC', serif; font-weight: 500; }
  h1 {
    max-width: 650px;
    margin-bottom: 28px;
    font-size: clamp(54px, 6vw, 86px);
    line-height: 0.98;
    letter-spacing: -0.055em;
  }
  h2 { margin-bottom: 22px; font-size: clamp(38px, 4.6vw, 64px); line-height: 1.06; letter-spacing: -0.04em; }
  .lede {
    max-width: 560px;
    margin-bottom: 34px;
    color: #68645f;
    font-size: 18px;
    line-height: 1.75;
  }
  .hero-actions { gap: 22px; }
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
  .primary-button:hover { transform: translateY(-2px); background: #a94229; }
  .primary-button:active { transform: translateY(0) scale(0.98); }
  .text-link { font-weight: 680; text-underline-offset: 5px; }

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
  .media-frame:hover { box-shadow: 0 36px 94px rgba(62, 43, 31, 0.16); }
  .media-glow {
    position: absolute;
    z-index: -1;
    inset: 0;
    border-radius: inherit;
    background: radial-gradient(circle at var(--spot-x) var(--spot-y), rgba(189, 79, 51, calc(0.16 * var(--media-active))), transparent 44%);
    pointer-events: none;
  }
  .media-frame img { display: block; width: 100%; height: auto; border: 1px solid rgba(34, 33, 31, 0.12); border-radius: 10px; object-fit: contain; }
  figcaption { margin-top: 14px; color: #77716b; font-size: 12px; line-height: 1.5; }

  .source-note {
    display: grid;
    grid-template-columns: minmax(0, 1.15fr) minmax(300px, 0.85fr);
    align-items: end;
    gap: 8vw;
    padding: 100px 0;
    border-top: 1px solid rgba(34, 33, 31, 0.14);
    border-bottom: 1px solid rgba(34, 33, 31, 0.14);
  }
  .source-note h2 { font-size: clamp(38px, 4vw, 56px); }
  .source-note p, .section-heading > p:last-child, .format-list p, .step-intro p, .privacy-note p, .final-cta p {
    color: #68645f;
    line-height: 1.75;
  }
  .source-note p { max-width: 670px; margin-bottom: 0; font-size: 17px; }
  .source-links { align-items: flex-start; flex-direction: column; gap: 16px; }
  .source-links a { gap: 7px; font-weight: 700; text-underline-offset: 5px; }

  .choose-section { padding: 132px 0; }
  .section-heading { max-width: 760px; margin-bottom: 62px; }
  .eyebrow { margin: 0 0 22px; color: #bd4f33; font-size: 12px; font-weight: 750; letter-spacing: 0.16em; text-transform: uppercase; }
  .section-heading > p:last-child { max-width: 660px; margin-bottom: 0; font-size: 17px; }
  .comparison { border-top: 1px solid #34312e; }
  .comparison-head, .comparison-row {
    display: grid;
    grid-template-columns: minmax(0, 1.4fr) minmax(150px, 0.55fr) minmax(220px, 0.75fr);
    align-items: center;
    gap: 24px;
  }
  .comparison-head { padding: 16px 22px; color: #77716b; font-size: 12px; font-weight: 750; letter-spacing: 0.08em; }
  .comparison-row { min-height: 112px; padding: 20px 22px; border-top: 1px solid rgba(34, 33, 31, 0.12); }
  .comparison-row:last-child { border-bottom: 1px solid rgba(34, 33, 31, 0.12); }
  .comparison-row strong, .comparison-row small { display: block; }
  .comparison-row strong { margin-bottom: 7px; font-size: 16px; }
  .comparison-row small { color: #77716b; font-size: 13px; }
  .method { font-weight: 720; }
  .method.accent { color: #bd4f33; }
  .reason { gap: 7px; color: #2f6e52; font-size: 14px; font-weight: 700; }

  .workflow-section {
    display: grid;
    grid-template-columns: minmax(440px, 1.1fr) minmax(0, 0.9fr);
    align-items: center;
    gap: clamp(56px, 8vw, 112px);
    padding: 120px 0 132px;
  }
  .format-list { margin-top: 42px; border-top: 1px solid rgba(34, 33, 31, 0.14); }
  .format-list article { display: grid; grid-template-columns: 42px 1fr; gap: 18px; padding: 24px 0; border-bottom: 1px solid rgba(34, 33, 31, 0.14); }
  .format-list article > span { display: grid; width: 38px; height: 38px; place-items: center; border: 1px solid rgba(34, 33, 31, 0.14); border-radius: 10px; color: #bd4f33; }
  h3 { margin: 0 0 7px; font-size: 16px; }
  .format-list p { margin-bottom: 0; font-size: 14px; }

  .steps-section {
    display: grid;
    grid-template-columns: minmax(0, 0.8fr) minmax(0, 1.2fr);
    gap: 8vw;
    padding: 112px 0;
    border-top: 1px solid rgba(34, 33, 31, 0.14);
  }
  .step-intro p { max-width: 520px; margin-bottom: 0; font-size: 17px; }
  ol { margin: 0; padding: 0; list-style: none; counter-reset: workflow; }
  li {
    counter-increment: workflow;
    display: grid;
    grid-template-columns: 44px 0.7fr 1.3fr;
    gap: 20px;
    padding: 24px 0;
    border-bottom: 1px solid rgba(34, 33, 31, 0.14);
  }
  li:first-child { border-top: 1px solid rgba(34, 33, 31, 0.14); }
  li::before { content: counter(workflow, decimal-leading-zero); color: #bd4f33; font: 700 12px 'Segoe UI', sans-serif; }
  li strong { font-size: 15px; }
  li span { color: #68645f; font-size: 14px; line-height: 1.6; }

  .privacy-note {
    gap: 26px;
    margin-bottom: 118px;
    padding: 36px 40px;
    border: 1px solid rgba(34, 33, 31, 0.14);
    border-radius: 14px;
  }
  .privacy-note > :global(svg) { flex: 0 0 auto; color: #bd4f33; }
  .privacy-note div { flex: 1; }
  .privacy-note h2 { margin-bottom: 8px; font-family: inherit; font-size: 22px; font-weight: 720; letter-spacing: -0.02em; }
  .privacy-note p { margin-bottom: 0; font-size: 14px; }
  .privacy-note a { flex: 0 0 auto; gap: 7px; font-weight: 700; text-underline-offset: 5px; }

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
  .final-cta h2 { margin-bottom: 18px; }
  .final-cta p { margin-bottom: 0; color: #c8c1b8; font-size: 16px; }
  .final-actions { flex: 0 0 auto; flex-direction: column; align-items: stretch; gap: 16px; }
  .final-actions > a { justify-content: center; gap: 8px; font-weight: 700; text-underline-offset: 5px; }
  .light-button { background: #f4eee4; color: #292724; }
  .light-button:hover { background: #fffaf4; }

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
  .footer-brand { color: #22211f; }
  footer p { margin: 0; }
  footer p:last-child { justify-self: end; }
  .footer-links { gap: 18px; }

  [data-reveal] { opacity: 0; transform: translateY(24px); transition: opacity 700ms cubic-bezier(0.16, 1, 0.3, 1), transform 700ms cubic-bezier(0.16, 1, 0.3, 1); }
  [data-reveal]:global(.is-visible) { opacity: 1; transform: translateY(0); }
  .media-frame[data-reveal], .media-frame[data-reveal]:global(.is-visible) { transform: perspective(1100px) translateY(calc(24px * (1 - var(--media-active)))) rotateX(var(--tilt-x)) rotateY(var(--tilt-y)); }
  .media-frame[data-reveal]:global(.is-visible) { transform: perspective(1100px) rotateX(var(--tilt-x)) rotateY(var(--tilt-y)); }
  :global(:focus-visible) { outline: 3px solid rgba(189, 79, 51, 0.42); outline-offset: 4px; }

  @media (max-width: 940px) {
    .hero, .workflow-section { grid-template-columns: 1fr; }
    .hero { min-height: 0; padding-top: 72px; }
    .hero-copy { max-width: 760px; }
    .hero-media, .workflow-media { width: min(100%, 760px); }
    .workflow-copy { max-width: 720px; }
  }

  @media (max-width: 720px) {
    header { min-height: 64px; padding: 0 20px; }
    nav > a:not(.back-link) { display: none; }
    .back-link { font-size: 0; }
    main { padding-top: 18px; }
    .hero, .source-note, .choose-section, .workflow-section, .steps-section, .privacy-note { width: auto; margin-inline: 20px; }
    .hero { gap: 52px; padding: 54px 0 80px; }
    h1 { font-size: clamp(47px, 14vw, 66px); }
    .hero-actions { align-items: flex-start; flex-direction: column; }
    .media-frame { padding: 12px 12px 10px; }
    .source-note, .steps-section { grid-template-columns: 1fr; gap: 36px; padding: 82px 0; }
    .choose-section, .workflow-section { padding: 88px 0; }
    .section-heading { margin-bottom: 42px; }
    .comparison { overflow-x: auto; }
    .comparison-head, .comparison-row { grid-template-columns: minmax(220px, 1.2fr) minmax(135px, 0.6fr) minmax(200px, 0.8fr); min-width: 620px; padding-inline: 12px; }
    .workflow-section { gap: 56px; }
    li { grid-template-columns: 36px 1fr; }
    li span { grid-column: 2; }
    .privacy-note { align-items: flex-start; flex-wrap: wrap; margin-bottom: 88px; padding: 28px 24px; }
    .privacy-note div { min-width: calc(100% - 60px); }
    .privacy-note a { margin-left: 54px; }
    .final-cta { align-items: flex-start; flex-direction: column; margin-inline: 12px; padding: 64px 28px; }
    .final-actions { width: 100%; }
    footer { grid-template-columns: 1fr; justify-items: start; min-height: 0; padding: 54px 20px; }
    footer p:last-child { justify-self: start; }
  }

  @media (prefers-reduced-motion: reduce) {
    :global(html) { scroll-behavior: auto; }
    [data-reveal] { opacity: 1; transform: none; transition: none; }
    .media-frame, .primary-button { transition: none; }
  }

  @media (prefers-color-scheme: dark) {
    :global(html), :global(body) { background: #201f1d; color: #f1eee8; }
    .guide-shell { background: radial-gradient(circle at 88% 4%, rgba(205, 96, 65, 0.1), transparent 28rem), #201f1d; }
    header, .brand-mark, .source-note, .comparison-row, .format-list, .format-list article,
    .format-list article > span, .steps-section, li, li:first-child, .privacy-note { border-color: rgba(255, 255, 255, 0.12); }
    .lede, .source-note p, .section-heading > p:last-child, .format-list p, .step-intro p,
    .privacy-note p, .comparison-row small, li span, figcaption, footer { color: #bdb7af; }
    .media-frame { border-color: rgba(255, 255, 255, 0.12); background: rgba(42, 40, 37, 0.76); box-shadow: 0 30px 80px rgba(0, 0, 0, 0.25); }
    .media-frame img { border-color: rgba(255, 255, 255, 0.13); }
    .comparison { border-top-color: #d5cfc8; }
    .reason { color: #80bf9b; }
    .footer-brand { color: #f1eee8; }
  }
</style>
