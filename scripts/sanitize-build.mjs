import { readdir, readFile, rm, writeFile } from 'node:fs/promises';

const buildUrl = new URL('../build/', import.meta.url);

async function sanitizeHtml(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const url = new URL(entry.name + (entry.isDirectory() ? '/' : ''), directory);
    if (entry.isDirectory()) {
      await sanitizeHtml(url);
    } else if (entry.name.endsWith('.html')) {
      const source = await readFile(url, 'utf8');
      const sanitized = source.replace(/^[\t ]+$/gm, '');
      if (sanitized !== source) await writeFile(url, sanitized);
    }
  }
}

// 仅排除构建副本；保留用户的原始恢复文件。
await rm(new URL('recovery-import.json', buildUrl), { force: true });
await sanitizeHtml(buildUrl);
