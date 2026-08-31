import DOMPurify from 'dompurify';
import TurndownService from 'turndown';
import { gfm } from 'turndown-plugin-gfm';

export const MAX_HTML_BYTES = 1024 * 1024;
const tags = ['p', 'br', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'strong', 'b', 'em', 'i', 'u', 's', 'del', 'ul', 'ol', 'li', 'blockquote', 'pre', 'code', 'a', 'table', 'thead', 'tbody', 'tfoot', 'tr', 'th', 'td', 'hr', 'div', 'span'];
const markdown = new TurndownService({ headingStyle: 'atx', bulletListMarker: '-', codeBlockStyle: 'fenced' });
markdown.use(gfm);

export function sanitizeClipboardHtml(raw?: string | null): { html?: string; warning?: string } {
  if (!raw) return {};
  if (new TextEncoder().encode(raw).length > MAX_HTML_BYTES) return { warning: 'HTML exceeds 1 MiB; only plain text was saved.' };
  try {
    const html = DOMPurify.sanitize(raw, {
      ALLOWED_TAGS: tags,
      ALLOWED_ATTR: ['href', 'title', 'colspan', 'rowspan', 'start'],
      ALLOW_DATA_ATTR: false,
      ALLOWED_URI_REGEXP: /^(?:https?:|mailto:|#)/i,
      FORBID_TAGS: ['script', 'style', 'iframe', 'object', 'embed', 'svg', 'math', 'img'],
    });
    return html.trim() ? { html } : {};
  } catch {
    return { warning: 'HTML could not be processed; only plain text was saved.' };
  }
}

export function textToHtml(text: string): string {
  const escaped = text.replace(/\r\n?/g, '\n').replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
  return escaped.split(/\n\n/).map(paragraph => `<p>${paragraph.replaceAll('\n', '<br>')}</p>`).join('\n');
}

export function toMarkdown(text: string, html?: string): string {
  const safe = sanitizeClipboardHtml(html).html;
  if (safe) return markdown.turndown(safe);
  return text.replace(/\r\n?/g, '\n').replace(/([\\`*_{}\[\]()#+.!|>~-])/g, '\\$1');
}

export function toClipboardHtml(text: string, html?: string): string {
  return sanitizeClipboardHtml(html).html || textToHtml(text);
}

export function textSignature(text: string, html?: string): string {
  const value = JSON.stringify([text, html || '']);
  let first = 0x811c9dc5;
  let second = 0x9e3779b9;
  for (let index = 0; index < value.length; index++) {
    first = Math.imul(first ^ value.charCodeAt(index), 16777619);
    second = Math.imul(second ^ value.charCodeAt(index), 2246822519);
  }
  return `text:${value.length}:${first >>> 0}:${second >>> 0}`;
}
