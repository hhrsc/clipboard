# Phase 2 — Pixel QA + Native Integration

日期：2026-08-31。范围：Windows Tauri 桌面、1586 × 992 目标 viewport、四页参考状态和现有核心业务。Phase 1 报告已保留在 `design-qa/phase1-report.md`。

## Visual QA

### 无损采集

- 来源：`reference/{recent,images,passwords,settings}.png`，均为 1586×992。
- Chromium / Playwright CLI 直接输出 PNG，viewport=1586×992、deviceScaleFactor=1、zoom=100%、visualViewport.scale=1。
- 无 JPEG 中转、无后续 resize、无有损压缩。PNG 自身的无损编码不改变像素。
- 每页有 `reference.png`、`implementation.png`、50% `overlay.png`、绝对 RGB 差分 `diff.png`。
- 另有 4 倍亮度 `diff-amplified.png`、整页并排图及未缩放的局部并排图。增强图仅辅助查看，不替代原差值。
- 第一版无损基线在 `design-qa/baseline/`；修正、重新截图和计算后的结果在 `design-qa/final/`。
- `capture-environment.json` 记录环境；`png-manifest.json` 校验 16 张核心图的 PNG 签名、尺寸和 SHA-256。
- 原生 WebView2 151.0.4129.107 的 CSS viewport 也是1586×992，DPR=1.5。原生功能证据不混入 DPR=1 的参考差分。桌面观察通道的截图不用于最终像素验收。

### 四页结论

| 页面 | 状态 | 已高度一致 / 修正内容 | 剩余差异 |
|---|---|---|---|
| Recent | 高度一致，素材受限 | 306px 侧栏、列表/详情、内容框、底部操作；分栏左移1px；Copy all 高44px、按钮gap 7px；合集选中背景高度修正 | 字体字形/抗锯齿、部分图标轮廓、按钮及背景细纹理 |
| Images | 高度一致，素材受限 | 174px 四列缩略图、13px gap、427×480预览、原素材裁切和分组位置 | 原始高清图不可恢复；字体、图标与细纹理不同 |
| Passwords | 高度一致，素材受限 | 列表、详情、底栏；Title/Username输入框约y=212/322、高52/55；Password约y=435 | 原字体、密码圆点字形、SVG、按钮纹理；真实后端没有示例分组 |
| Settings | 高度一致，素材受限 | 设置行、横线、控件、Danger zone；键帽字体与宽度修正 | 字体/SVG、背景微纹理；缺少hover设计状态 |

共同修正：Collections标题上移4px、合集内容左移1px、选中底色按采样值修正。没有添加渐变、装饰阴影或动画。真实数据中长图片名称采用单行省略，修复其覆盖PNG标签的问题，不改变参考图短文件名布局。

### 定量结果

MAE为所有像素三个通道的平均绝对差（0–255）；阈值面积指任一通道差值>24的像素比例。**不是相似度百分比或自动通过阈值。**

| 页面 | 最终MAE | 差值>24像素占比 |
|---|---:|---:|
| Recent | 5.5279 | 5.4087% |
| Images | 2.8203 | 2.5856% |
| Passwords | 4.8043 | 3.7417% |
| Settings | 3.8380 | 3.5156% |

Recent整体MAE没有单调下降，阈值差异面积下降；没有为降低数字撤回按参考边界测量的修正。大面积空白会影响整页指标，因此同时检查局部并排图。

Images内部区域 `(1135,110)-(1540,565)` 和 `(350,230)-(500,420)` 的RGB MAE均为 **0**，仅代表这两个抽样区域，不外推为全页一致。

| 页面 | PNG | 并排图 | 叠加图 | 差分图 |
|---|---|---|---|---|
| Recent | [implementation](design-qa/final/recent/implementation.png) | [comparison](design-qa/final/recent/comparison.png) | [overlay](design-qa/final/recent/overlay.png) | [diff](design-qa/final/recent/diff.png) |
| Images | [implementation](design-qa/final/images/implementation.png) | [comparison](design-qa/final/images/comparison.png) | [overlay](design-qa/final/images/overlay.png) | [diff](design-qa/final/images/diff.png) |
| Passwords | [implementation](design-qa/final/passwords/implementation.png) | [comparison](design-qa/final/passwords/comparison.png) | [overlay](design-qa/final/passwords/overlay.png) | [diff](design-qa/final/passwords/diff.png) |
| Settings | [implementation](design-qa/final/settings/implementation.png) | [comparison](design-qa/final/settings/comparison.png) | [overlay](design-qa/final/settings/overlay.png) | [diff](design-qa/final/settings/diff.png) |

### 状态边界

参考选中态逐页比较。根窗口不滚动，列表和设置内容独立滚动。参考fixture的滚动条形状按图呈现；原生滚动条随实际数据变化，不伪造历史数量。

参考没有hover、空状态、锁库、加载失败及扩展原生设置设计稿，这些状态只做功能验证，不虚构1:1结论。真实内容、数量、时间、分类与示例不同是数据差异，不是两套UI。

## Functional QA

### 方法与证据

先构建并运行隔离QA **exe**，identifier=`com.lenovo.my-clipboard.qa-phase2`。通过其实际 `tauri.localhost` WebView测试；IPC、存储和Windows剪贴板均为真实调用，没有mock。

测试使用明确标记的临时文本、参考图片和合成密码。脚本检查QA identifier，防止误清正式数据。Windows原剪贴板已恢复，开机启动测试恢复初始状态；未读写用户真实密码。

随后运行正式identifier=`com.lenovo.my-clipboard`的release exe，验证页面、真实历史、临时空合集。删除测试合集后核对原记录ID和内容全部保留。

`design-qa/final/functional-results.json`：**39项程序化检查，0失败**。另有6项重启观察/文件读回检查及6项Rust单元测试；该数字不是覆盖率。

| 实际测试项目 | 结果 | 方法 |
|---|---|---|
| 真实桌面启动 | 通过 | QA/正式exe、原生bridge存在、reference-mode不存在 |
| 窗口尺寸 | 通过 | 原生CSS viewport1586×992、zoom=1 |
| 四页切换 | 通过 | 正式版逐页点击 |
| 真实历史读取 | 通过 | 正式版30条记录，无存储错误提示 |
| 文本实时监听 | 通过 | 写Windows剪贴板，轮询自动生成原生历史 |
| 文本显示、选择 | 通过 | 点击后详情与测试文本相同 |
| 点击复制文本 | 通过 | Copy all后原生读回一致 |
| 搜索及无结果 | 通过 | 无匹配行数为0，清空搜索恢复 |
| 删除文本 | 通过 | UI删除后store无该记录 |
| pin | 通过 | isPinned写入原生store，重启后保留 |
| 空历史 | 通过 | 空store显示空状态，无假数据 |
| 图片实时捕获 | 通过 | Windows图片剪贴板生成file-backed记录 |
| 缩略图和预览 | 通过 | 原生图片成功解码；重启后窗口中仍可见 |
| 点击复制图片 | 通过 | Copy image后原生剪贴板类型为image |
| 图片删除 | 通过 | UI删除隔离记录并核对store |
| 图片失败状态 | 通过 | 注入QA缺失路径，出现错误状态，复制禁用 |
| 密码库setup/unlock/lock | 通过 | 复用原有AES-GCM/Argon2后端，仅用合成主密码 |
| 密码新增/编辑/删除 | 通过 | UI操作后原生解密读回核对合成条目 |
| 用户名/密码复制 | 通过 | 原生剪贴板读回合成内容 |
| 密码搜索 | 通过 | 无匹配项显示空列表 |
| 密码JSON导入 | 通过 | 现有file input/change处理器导入合成JSON并保存；未自动化系统选择器 |
| capture暂停 | 通过 | Off后写入新测试文本，超过轮询周期仍未加入 |
| retention保存 | 通过 | UI保存25小时，原生读回；重启后文件仍为25 |
| 开机启动设置 | 通过 | 插件enable/disable读回，恢复初始状态；未模拟机器开机 |
| 快捷键保存 | 通过 | Alt+V注册和写入，重启后的UI显示Alt+V |
| 合集创建/重命名/删除 | 通过 | 正式版仅操作临时空合集，原记录保留 |
| QA进程重启 | 通过 | PID20996→3740；普通启动，文本/图片及Capture Off/Alt+V恢复 |
| 正式版进程重启 | 通过 | PID25496→19872；原30条记录逐条核对保留，新UI正常进入 |
| 前端runtime error | 测试期间未发现 | pageerror与控制台检查；旧图片404单独列为资源问题 |
| Rust panic | 测试期间未发现 | 进程存活、stderr无panic；不外推所有故障条件 |
| Svelte检查 | 通过 | npm run check：0 errors、0 warnings |
| Rust测试 | 通过 | 6/6：快捷键校验、旧迁移、加密往返、错误主密码、备份、主密码轮换 |
| release build | 通过 | npm run tauri -- build --no-bundle，非裸Cargo |

未执行或不作通过承诺：JSON导出下载落盘全流程、系统文件选择器、Danger zone真清空、机器开机后自启动、完整24小时自然过期、磁盘写满/断电恢复、所有快捷键、其他OS/DPI/窗口尺寸。正式密码库需由用户自己设置或解锁主密码，未自动创建用户主密码。

### 4张既有缓存图片缺失

正式历史的4个图片路径在启动前目录检查时已无文件。记录保留，界面显示失败状态；对应4个HTTP404不是JS exception。在项目列出的旧备份目录与static中按原文件名查找，未找到原文件；没有替换或自动删除记录。

## Remaining blockers

### CODE issue

本轮测试范围内无剩余阻断性代码错误；未覆盖的条件不视为通过。已修复隐藏旧UI并存、原生快照写入顺序、保存失败提示/回滚、锁库状态、图片错误状态、设置保存及长图片名覆盖标签。

### ASSET issue

1. 缺原字体文件与精确字重，字形、内部度量和栅格化不同。
2. 缺部分原始SVG，现有图标库路径及墨迹范围不完全相同，不手绘假SVG。
3. 图片只有截图级裁片，无法恢复更高清源图，目标尺寸保留原裁片。
4. 原图微弱纹理没有独立素材，不凭主观加渐变或模糊模拟。
5. 正式历史4张旧缓存图缺失，需要原文件或可用备份。

### PLATFORM issue

QA远程调试重启曾被策略拦截；通过**不启用远程调试的普通exe启动**、Windows窗口读取及文件读回完成重启检查，见 `native-restart-observation.json`。浏览器PNG采集限制已解决。DPR不同的原生与浏览器证据没有混算。

### UNKNOWN

无法从现有文件确定旧图片为何缺失；不归因于某个旧版本。没有hover/其他viewport设计稿，无法给出1:1结论。导出落盘、故障恢复等未测试项仍未验证。

## Architecture

**新UI已经正式替代旧UI。**

`+page.svelte`业务控制器 → `ClipboardApp.svelte`正式视觉UI → Tauri IPC → 原有Rust历史store / 图片缓存 / 加密密码库 / OS设置。

- ReferenceClipboard已更名为ClipboardApp；旧隐藏DOM及其CSS已移除，不再同时挂载两套应用。
- 参考fixture仅能在开发浏览器通过query启用，必须DEV且非Tauri；正式原生版不能query切成假数据。
- 保留已验收布局，没有为拆组件做大规模重构。
- 现有密码后端无分类，正式页只显示真实All items，不伪造Work/Finance数据。
- 折叠Desktop & storage承接原生启动项、保留时间和存储规则，不出现在参考fixture中。
- 不再每次启动强制enable自启动；按用户配置读回。
- 旧历史迁移写原生成功后才删除旧key，失败保留并提示；不自动修写用户密码字段。
- 停止启动自动导入static/recovery-import.json，并排除其发布副本；原文件保留，避免私人恢复数据打入exe。
- 旧68条CSS warning随被替代的旧UI CSS退出编译；没有为了清warning重构无关模块。

## 修改文件

| 文件 | 变更 |
|---|---|
| src/routes/+page.svelte | 单一UI、业务适配、原生保存/迁移/错误处理、设置和密码接线 |
| src/lib/ClipboardApp.svelte | Reference UI正式化、有限像素修正、真实状态/图片/合集/设置交互 |
| src-tauri/src/main.rs | 原生自启动读写命令，移除启动强制enable；保留已有合约 |
| src/app.html | 正式标题Clipboard |
| package.json、scripts/sanitize-build.mjs | 构建排除私人恢复文件副本 |
| .gitignore | 忽略含真实数据风险的本地快照/output |
| scripts/pixel-qa.config.json、capture-pixel-qa.js、pixel-diff.py、measure-pixel-qa.py、inspect-pixel-qa.js | PNG捕获、测量、差分 |
| scripts/native-*.js、production-smoke.js、tauri-qa.config.json、fixtures/passwords-qa.json | 原生E2E及隔离合成数据 |
| scripts/collect-qa-evidence.py | 无损文件校验、测试结果汇总 |
| design-qa.md、design-qa/final/、design-qa/baseline/ | 报告及证据 |

已有Cargo、窗口配置、依赖锁文件等未提交改动不全部属于本阶段；没有覆盖或回退这些工作，没有提交/上传。旧exe备份在output/phase2-source-backup，现有数据安全副本在仓库外 E:/clipboard/phase2-safety-backup-20260831；后者是启动后的存储核对副本，不宣称为迁移前完整备份。

## Final verdict

**PASS WITH ASSET LIMITATIONS**

四页无损PNG对比、有限视觉修正、核心原生业务与重启检查通过；不宣称100%逐像素一致，也不把未测项目计为通过。

最后一处长图片名称修正已重新通过check及正式Tauri release构建。最终交付exe已普通启动并前置：PID **35804**，路径 `E:/clipboard/my-clipboard/src-tauri/target/release/my-clipboard.exe`。实际窗口显示新Recent UI，30条真实记录仍在，无存储错误提示，stderr为0字节。隔离QA进程已关闭，最终启动未启用远程调试。

最终exe SHA-256：`6D0CA995EDE5851F41E8D16794974C6D44B3947DD33B9936EC55D415B5ABDB19`。运行证据见 `design-qa/final/release-verification.json`。
