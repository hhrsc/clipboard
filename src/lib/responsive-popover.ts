const adaptiveQuery = '(max-width: 1585px), (max-height: 991px), (min-width: 1700px), (min-height: 1100px)';

export function responsivePopover(node: HTMLElement, selector: string) {
  const root = node.closest('.ref-window');
  const anchor = root?.querySelector<HTMLElement>(selector);
  if (!root || !anchor) return;
  const marker = document.createComment('');
  node.before(marker);
  const originalStyle = node.getAttribute('style');
  const media = matchMedia(adaptiveQuery);
  let floating = false;
  let frame = 0;

  function restore() {
    marker.before(node);
    if (originalStyle === null) node.removeAttribute('style');
    else node.setAttribute('style', originalStyle);
    floating = false;
  }

  function place() {
    frame = 0;
    if (!media.matches) {
      if (floating) restore();
      return;
    }
    if (!floating) {
      const { width, font } = getComputedStyle(node);
      // 移到窗口层，避免滚动容器和 transform 裁切菜单；基准布局不受影响。
      root!.append(node);
      Object.assign(node.style, { position: 'fixed', inset: 'auto', margin: '0', width, font, zIndex: '37' });
      floating = true;
    }
    node.style.maxHeight = `${Math.min(320, innerHeight - 24)}px`;
    node.style.maxWidth = `${innerWidth - 24}px`;
    const rect = anchor!.getBoundingClientRect();
    const { width, height } = node.getBoundingClientRect();
    const left = node.classList.contains('collection-picker') ? rect.left : rect.right - width;
    const top = rect.bottom + 6 + height <= innerHeight - 12 ? rect.bottom + 6 : rect.top - height - 6;
    node.style.left = `${Math.max(12, Math.min(left, innerWidth - width - 12))}px`;
    node.style.top = `${Math.max(12, Math.min(top, innerHeight - height - 12))}px`;
  }

  function schedule() {
    if (!frame) frame = requestAnimationFrame(place);
  }
  const observer = new ResizeObserver(schedule);
  observer.observe(node);
  observer.observe(anchor);
  media.addEventListener('change', schedule);
  window.addEventListener('resize', schedule);
  document.addEventListener('scroll', schedule, true);
  place();
  return {
    destroy() {
      cancelAnimationFrame(frame);
      observer.disconnect();
      media.removeEventListener('change', schedule);
      window.removeEventListener('resize', schedule);
      document.removeEventListener('scroll', schedule, true);
      if (floating) node.remove();
      marker.remove();
    }
  };
}
