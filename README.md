<div align="center">
  <img src="https://hhrsc.github.io/favicon.png" width="88" height="88" alt="Clipboard 图标" />
  <h1>Clipboard</h1>
  <p><strong>复制以后，依然找得到。</strong></p>
  <p>一款本地优先的 Windows 剪贴板工具。保存文字与图片历史，自由选取需要的内容，再复制为纯文本、Markdown、HTML 或单行文本。</p>

  <p>
    <a href="https://hhrsc.github.io/website/">官网</a> ·
    <a href="https://github.com/hhrsc/clipboard/releases/download/v0.1.0/Clipboard_0.1.0_x64-setup.exe"><strong>下载 Windows 版</strong></a> ·
    <a href="https://github.com/hhrsc/clipboard/releases/tag/v0.1.0">版本说明</a> ·
    <a href="https://github.com/hhrsc/clipboard/issues/new/choose">问题与建议</a>
  </p>

  <p>
    <img alt="Windows 10 和 11" src="https://img.shields.io/badge/Windows-10%20%7C%2011-2d6f8e?logo=windows11&logoColor=white" />
    <img alt="版本 0.1.0" src="https://img.shields.io/badge/version-0.1.0-bb5f43" />
    <img alt="本地优先" src="https://img.shields.io/badge/data-local%20first-3d7055" />
    <img alt="MIT License" src="https://img.shields.io/badge/license-MIT-5c554d" />
  </p>
</div>

![Clipboard 的文字历史与详情界面](https://hhrsc.github.io/marketing/clipboard-recent.webp)

## 为什么做它

Windows 的 `Win + V` 适合临时找回最近内容，但当记录变多，搜索、分类、图片管理和格式转换就会开始浪费时间。Clipboard 把这些操作集中在一个安静的桌面窗口里：打开、找到、复制，然后继续手上的事。

## 使用指南

- [Windows 剪贴板历史怎么用？Win + V 与 Clipboard 选择指南](https://hhrsc.github.io/website/guides/windows-clipboard-history/)
- [如何把文字复制为 Markdown、HTML、纯文本或单行文本](https://hhrsc.github.io/website/guides/copy-text-as-markdown-html/)

## 现在能做什么

- 保存文字、富文本和图片历史
- 搜索、分类、置顶并设置普通文本保留时间
- 在详情中自由选取真正需要的文字
- 复制为纯文本、Markdown、HTML 或单行文本
- 使用自定义全局快捷键快速打开
- 在本机使用加密密码库整理常用账号信息

## 本地优先

当前版本不要求账户，没有云同步，也没有在应用内接入广告或行为分析。剪贴板历史与密码库保存在你的 Windows 设备上。

问题反馈是公开的，请勿在 Issue 中粘贴密码、令牌或真实剪贴板隐私内容。完整说明见[隐私政策](https://hhrsc.github.io/website/privacy-policy/)。

## 下载

| 项目 | 信息 |
| --- | --- |
| 当前版本 | `0.1.0` |
| 平台 | 64 位 Windows 10 / 11 |
| 安装包大小 | 3.52 MB |
| SHA-256 | `C5BE35E575E698696C376DF46247D4A670D0C8880098F5F2EAD5B7F921B8AE4C` |

[从 GitHub Releases 下载](https://github.com/hhrsc/clipboard/releases/tag/v0.1.0)。独立开发者的新应用可能暂时触发 Windows SmartScreen 提醒，请只使用官网或本仓库的版本页，并核对上面的校验值。

## 当前边界

Clipboard 仍处于早期阶段，目前只提供 64 位 Windows 版本，也没有跨设备同步。功能和数据安全相关问题会优先处理；如果你愿意，请描述自己的真实工作流，而不只是给出一个功能名称。

- [报告可以复现的问题](https://github.com/hhrsc/clipboard/issues/new?template=bug_report.yml)
- [提交真实场景中的功能建议](https://github.com/hhrsc/clipboard/issues/new?template=feature_request.yml)
- [观看 20 秒产品演示](https://github.com/hhrsc/clipboard/releases/download/v0.1.0/clipboard-promo-vertical-v1.mp4)

## 开发

技术栈：Tauri 2、Svelte 5 / SvelteKit、Rust。

```powershell
npm ci
npm run check
npm run tauri -- dev
```

正式 Windows 构建：

```powershell
npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc
```

浏览器预览不能代替 Tauri 原生功能验收。涉及密码库、重置或导入导出时，请使用隔离测试资料。

## License

[MIT](LICENSE)
