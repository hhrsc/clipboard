const host = 'hhrsc.github.io';
const key = '8bd9a36d037547de87850f66b53dce7f';
const keyLocation = `https://${host}/${key}.txt`;
const urlList = [
  `https://${host}/website/`,
  `https://${host}/website/privacy-policy/`,
  `https://${host}/website/guides/windows-clipboard-history/`,
  `https://${host}/website/guides/copy-text-as-markdown-html/`
];

const response = await fetch('https://api.indexnow.org/indexnow', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json; charset=utf-8' },
  body: JSON.stringify({ host, key, keyLocation, urlList })
});

const result = {
  submittedAt: new Date().toISOString(),
  endpoint: 'https://api.indexnow.org/indexnow',
  status: response.status,
  accepted: response.ok,
  keyLocation,
  urlList
};

console.log(JSON.stringify(result, null, 2));
await appendFile(resolve('docs/marketing/indexnow-submissions.jsonl'), `${JSON.stringify(result)}\n`, 'utf8');
if (!response.ok) process.exitCode = 1;
import { appendFile } from 'node:fs/promises';
import { resolve } from 'node:path';
