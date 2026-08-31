import { rm } from 'node:fs/promises';

// 仅排除构建副本；保留用户的原始恢复文件。
await rm(new URL('../build/recovery-import.json', import.meta.url), { force: true });
