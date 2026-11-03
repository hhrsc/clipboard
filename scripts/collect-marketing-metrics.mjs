import { execFile } from 'node:child_process';
import { appendFile, mkdir } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { promisify } from 'node:util';

const distributionRepository = 'hhrsc/clipboard';
const feedbackRepository = 'hhrsc/clipboard';
const pagesRepository = 'hhrsc/hhrsc.github.io';
const releaseTag = 'v0.1.0';
const websiteUrl = 'https://hhrsc.github.io/website/';
const historyFile = resolve('docs/marketing/metrics-history.jsonl');
const execFileAsync = promisify(execFile);

const apiHeaders = {
  Accept: 'application/vnd.github+json',
  'User-Agent': 'clipboard-marketing-metrics',
  'X-GitHub-Api-Version': '2022-11-28',
  ...(process.env.GH_TOKEN ? { Authorization: `Bearer ${process.env.GH_TOKEN}` } : {})
};

async function getJson(url) {
  const response = await fetch(url, { headers: apiHeaders });
  if (!response.ok) throw new Error(`${url} 返回 ${response.status}`);
  return response.json();
}

async function getGitHubJson(path) {
  if (process.env.GH_TOKEN) return getJson(`https://api.github.com/${path}`);
  const { stdout } = await execFileAsync('gh', ['api', path], {
    encoding: 'utf8',
    maxBuffer: 2 * 1024 * 1024,
    windowsHide: true
  });
  return JSON.parse(stdout);
}

async function getTraffic(repository) {
  try {
    const base = `repos/${repository}/traffic`;
    const [views, clones, referrers, paths] = await Promise.all([
      getGitHubJson(`${base}/views`),
      getGitHubJson(`${base}/clones`),
      getGitHubJson(`${base}/popular/referrers`),
      getGitHubJson(`${base}/popular/paths`)
    ]);
    return {
      available: true,
      views: { count: views.count, uniques: views.uniques },
      clones: { count: clones.count, uniques: clones.uniques },
      referrers,
      paths
    };
  } catch (error) {
    return {
      available: false,
      reason: error instanceof Error ? error.message : String(error)
    };
  }
}

const [release, repo, website, productTraffic, pagesTraffic] = await Promise.all([
  getGitHubJson(`repos/${distributionRepository}/releases/tags/${releaseTag}`),
  getGitHubJson(`repos/${feedbackRepository}`),
  fetch(websiteUrl, { redirect: 'follow' }),
  getTraffic(feedbackRepository),
  getTraffic(pagesRepository)
]);

const assets = release.assets.map(({ name, download_count: downloads, size, browser_download_url: url }) => ({
  name,
  downloads,
  size,
  url
}));

const snapshot = {
  capturedAt: new Date().toISOString(),
  website: {
    url: websiteUrl,
    status: website.status,
    available: website.ok
  },
  release: {
    tag: release.tag_name,
    publishedAt: release.published_at,
    url: release.html_url,
    totalDownloads: assets.reduce((total, asset) => total + asset.downloads, 0),
    assets
  },
  repository: {
    url: repo.html_url,
    stars: repo.stargazers_count,
    forks: repo.forks_count,
    openIssuesAndPullRequests: repo.open_issues_count
  },
  traffic: {
    productRepository: productTraffic,
    pagesRepository: pagesTraffic
  }
};

await mkdir(dirname(historyFile), { recursive: true });
await appendFile(historyFile, `${JSON.stringify(snapshot)}\n`, 'utf8');

console.log(JSON.stringify(snapshot, null, 2));
