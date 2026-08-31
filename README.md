# Clipboard · Tag-V1 阶段总结

基于 **Tauri 2 + Svelte 5 / SvelteKit + Rust** 的 Windows 本地剪贴板工具，包含 Characters、Images、Passwords、Settings 四个页面。

本分支是阶段归档，不是 Git tag，也不是版本号升级：包版本仍为 `0.1.0`，应用标识仍为 `com.lenovo.my-clipboard`。讨论过的品牌名 Clasp / 拾贴尚未统一应用到产品标题和包名，本次不改名，以免影响数据与启动配置。

## 本阶段实现

| 模块 | 当前实现 |
| --- | --- |
| Characters | 原 Recent 更名；仅显示文本历史，支持搜索、选择、复制、删除、置顶和分类。图片独立展示于 Images，不迁移或删除已有图片数据。 |
| Images | 原生图片历史、缩略图、预览、复制、删除；分类、搜索和日期取交集，侧栏显示图片数量；无结果时清空旧预览。清空图片作用于全部图片，不限当前筛选。 |
| Passwords | 本地加密密码库、条目编辑、用户名/密码复制、筛选和排序；分组创建、重命名、移动和删除，删除分组不删除密码；分组与条目以同一加密快照串行保存。 |
| 密码库兼容 | v2 载荷保存分组与关联，兼容旧库和旧 JSON 导入，导出 v2；主密码轮换和备份通路保留分组。新主密码要求 8–16 个字符，已有旧密码保留解锁兼容。 |
| 打开密码本 | 默认需要主密码。Windows 可在设置中关闭每次输入要求，利用当前用户的 DPAPI 保护本地解锁材料，并非明文保存密码；首次建库仍需设置主密码。共享 Windows 账户时不建议关闭。 |
| 格式化复制 | 捕获可用 HTML，以 DOMPurify 净化，再通过 Turndown + GFM 转 Markdown；HTML 复制同时写入富文本和纯文本备用格式。历史不直接渲染外部 HTML，超限/读取失败降级保留纯文本。 |
| Settings | 监听、自启动、文本保留时长和原生快捷键保存；快捷键直接捕获按键组合，不要求手工输入。可用范围受原生注册规则约束，不承诺所有按键组合都可用。 |
| 完整重置 | `DELETE` 确认、写入排空、持久化标记与中断恢复；清理本应用历史、密码库、分组和图片缓存，恢复 Capture On、24 小时、Alt+C、自启动 Off，不清除系统剪贴板或外部备份。 |
| 窗口适配 | 默认逻辑尺寸 1586×992，最小 360×400；窄窗口重排、导航收拢、菜单视口约束、短窗口滚动。关闭窗口隐藏到托盘。 |
| 图标 | 已采用夹子精灵 A1 大圆角图标：陶土橙、象牙色主体、深色夹头；接入桌面 PNG/ICO/ICNS、favicon 和默认托盘图标。构建脚本跟踪 ICO 变化，避免旧资源被缓存。 |

没有新增云同步、密码生成器或安全评分。图片上限为 10 条 / 80 MiB；未分类、未置顶文本上限为 50 条，其保留时间默认 24 小时。分类或置顶文本不按该普通文本规则清理。

## 架构与设计边界

`src/routes/+page.svelte` 负责数据、状态、保存队列及原生命令；`src/lib/ClipboardApp.svelte` 是四页共用的正式 UI；原生入口为 `src-tauri/src/main.rs`。Windows 便捷解锁位于 `src-tauri/src/local_vault_unlock.rs`。

不是旧 UI 与新 Demo 两套产品。仅在浏览器开发环境下，`?reference=recent`、`images`、`passwords`、`settings` 启用同一 UI 的固定样例数据，用于视觉对照；生产和 Tauri 环境不启用该模式。原生功能须在桌面程序验证，浏览器预览不能代替桌面验收。

- [AGENTS.md](AGENTS.md)：项目协作、视觉验收和重启规则。
- [reference/](reference/)：原始设计图，视觉事实来源。
- [docs/visual-spec.md](docs/visual-spec.md)：尺寸、排版和组件规范。
- 后续经用户确认的 Characters 命名、响应式行为与图标变更，保留在当前实现中。

## 验证状态

### 本次归档重新执行

| 检查 | 结果 / 范围 |
| --- | --- |
| `npm run check` | 通过，0 errors / 0 warnings。 |
| `cargo check --tests` | 通过；编译检查，不等于执行 Rust 测试或桌面 E2E。 |
| 公开入口 Svelte 编译 | 从提交快照排除 4 个无业务调用的私人恢复辅助函数后，编译通过，0 warnings；本地原文件未覆盖。 |
| Git 暂存区检查 | 无空白错误、无 Gitlink；改动文件未命中本次识别出的私人值和凭据特征。此检查不是完整安全审计。 |
| 清理完整性 | 保留 EXE 和本地入口源码的 SHA-256 未变化；参考图、规范、必要素材、Agent 文件及私人备份保留。 |
| 应用重启 | 清理后重新启动保留的正式构建 EXE；完整路径一致、单实例、Responding=True。这不是重新构建或完整桌面验收。 |

### 既有阶段记录，不冒充本次重跑

此前记录包含四页无损 PNG 对照、响应式布局/交互检查及部分原生检查。旧视觉结论为 **PASS WITH ASSET LIMITATIONS**：缺少原始字体、部分 SVG 与原始高清图片，不能宣称严格 pixel-perfect。旧结论不自动覆盖后来新增的密码、重置和便捷解锁功能。

最近图标阶段记录的正式 Tauri 构建命令：

```powershell
npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc
```

该 EXE 时间为 2026-08-31 22:12:10，SHA-256 为 `5b922fe8fff0986e20c3f0a896248872ffe1a23b1b40ff5a646faeabbd16a1ab`；本次重新核对哈希一致并保留本地文件。旧记录还确认了 EXE 六个图标尺寸与 ICO 的 RGBA 字节一致。本次没有重新打包安装程序，也没有把本地 EXE 上传为 Release 附件。

### 待用户验收

- 密码筛选、搜索、排序、分组组合正确；行菜单仅操作目标条目；失败时草稿不丢失。
- 旧密码库升级、重启、主密码轮换、JSON 导入导出后，内容和分组不丢失。
- 图片分类、搜索、日期交集正确；无结果及加载失败状态正确。
- 网页标题、列表、链接、代码、表格转 Markdown；不同粘贴目标下的 HTML 与纯文本备用格式；格式变化监听和自身复制去重。
- 快捷键录制、冲突提示、保存与重启；主密码要求开关在 Windows 登录用户边界内的行为。
- **仅用隔离测试资料**检查重置、中断恢复、清理范围和重启不复活旧数据；不要拿真实密码库做清空测试。
- 多尺寸桌面窗口、实时监听、图片/文本复制、托盘行为和新增菜单的视觉回归。

本阶段状态：**实现与编译检查已完成，完整桌面功能验收待用户完成**。本次没有操作真实密码、清空应用数据或执行实际粘贴验证；未重跑依赖安全审计，也不声称没有安全问题。

## 开发与构建

Windows 需要 Node.js/npm、Rust MSVC 工具链、Visual Studio C++ 构建工具与 WebView2 Runtime。Python 图标/图像检查脚本另需 Pillow；浏览器 QA 需要可用的 Playwright 环境。

```powershell
npm ci
npm run check
npm run tauri -- dev
```

仅看视觉样例：运行 `npm run dev`，打开 `http://127.0.0.1:1420/?reference=recent`；其它页面将参数替换为 `images`、`passwords`、`settings`。

正式编译前退出运行中的本项目 EXE，再执行：

```powershell
npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc
```

成功后运行 `src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe`。不要以裸 `cargo build` 代替正式 Tauri 前端/原生联合构建，也不要在编译失败时启动旧产物并声称构建成功。缓存已清理，下一次编译会重新生成缓存，耗时可能增加。

`scripts/` 保留视觉测量、PNG 差分、响应式、快捷键、分类、密码及原生 QA 脚本与合成 fixtures。多数浏览器脚本是交给 Playwright 执行的 `async (page) => ...` 片段，不是可直接 `node 文件名` 运行的独立程序；截图前需创建脚本使用的输出目录。`production-smoke.js` 会修改生产数据，不能当只读检查运行；请使用独立测试配置和资料开展原生验收。

视觉比较固定 **1586×992 / deviceScaleFactor=1 / zoom=100% / PNG**，不缩放、不转 JPEG。配置见 `scripts/pixel-qa.config.json`。

## 图标来源与维护

- 原图：`assets/app-icon-clip-sprite.png`（1254×1254）。
- 当前源图：`assets/app-icon-clip-sprite-rounded.png`，仅用五次超椭圆遮罩修改 alpha，保留原 RGB。
- 遮罩生成脚本：`scripts/make-rounded-app-icon.py`。
- EXE 图标检查：`scripts/verify-exe-icon.py`，直接读取 PE 图标资源，不依赖 Windows Shell 缓存。
- 当前资源：`src-tauri/icons/` 顶层桌面文件与 `static/favicon.png`。重新生成时先输出到临时目录，只更新桌面文件，不覆盖 Android/iOS 配置。

## 归档与清理记录

2026-08-31 创建分支 `Tag-V1`，先提交阶段快照 **[`7296a07`](https://github.com/hhrsc/clipboard/commit/7296a07ad24f3d5ede39d86f8d53ec004f1dcb53)**，再清理并整理本 README。

- 直接删除约 **13.09 GiB** 的可再生缓存及旧产物，主要是 Cargo debug/release 中间文件、Svelte/Vite 缓存、构建副本、浏览器日志和 QA 截图。按文件逻辑大小统计，不等同于磁盘实际释放量。
- 已跟踪的旧 `design-qa/`、`design-qa.md`、阶段交接/响应式/图标报告及误生成的终端日志，从当前目录删除；仍可在阶段快照查看和恢复。
- 未采用的图标草稿、设计展示稿和未跟踪的旧计划笔记（约 6 MiB）移至 Windows 回收站，可恢复；未清空回收站。
- 保留源码、构建配置和锁文件、现用素材、设计原图、视觉规范、可复用脚本、已安装依赖，以及原路径下的当前可运行 EXE。
- 保留 Git 历史与 Agent 必读文件。本地私人恢复材料、备份和敏感辅助代码不上传、不删除；本机入口源码因此有**有意保留的未提交差异**，不要全量 `git add .` 带入公开仓库。
- 不清理系统或其它项目的缓存，不删除运行时 AppData、Windows 剪贴板、外部导出文件或仓库外备份。

公开提交排除了本机私人恢复材料；干净克隆从源码构建，不依赖这些资料。旧报告中已移除的截图链接应通过阶段快照查看，新截图由保留的 QA 脚本重新生成。
