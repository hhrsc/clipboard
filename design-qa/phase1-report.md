# Clipboard visual QA

验收范围：四张参考图对应的桌面静态状态，1586 × 992 viewport。不是全平台、所有窗口尺寸或原生数据流程的验收。

final result: blocked

四页实现、最终构建与视觉对比已完成，主要布局达到高度一致；严格逐像素验收仍未通过。最终核验发现浏览器截图通道实际返回有损 JPEG，且原始字体、SVG 路径未提供，因此不能诚实地宣称 100% pixel-perfect。这里的 blocked 指严格视觉验收，不是构建或页面运行失败。

## 视觉事实来源与运行入口

- `reference/recent.png`
- `reference/images.png`
- `reference/passwords.png`
- `reference/settings.png`
- `docs/visual-spec.md`
- 实现：`src/lib/ReferenceClipboard.svelte`，由 `src/routes/+page.svelte` 接入已有业务处理器。
- 预览：`http://127.0.0.1:1420/?reference=recent`。query 值可替换为 `images`、`passwords`、`settings`。
- 不带 reference 参数时接入真实历史记录和原生密码库；带参数时使用仅存在于内存中的参考数据，不把截图示例写入真实数据存储。

## 归一化与状态

| 项目 | 值 |
|---|---|
| 参考 PNG | 每页 1586 × 992 px |
| 浏览器 CSS viewport | 1586 × 992 CSS px |
| 浏览器 DPR | 1.0000000298，约等于 1 |
| 实现截图 | 每页 1586 × 992 px；原始捕获实际为 JPEG，保留为 `*-final.capture.jpg`，解码后另存真实 PNG 用于对比 |
| 并排图 | 3172 × 992 px；左为 REFERENCE，右为 IMPLEMENTATION |
| 归一化 | 不拉伸参考，不缩放截图，不包含浏览器工具栏；JPEG → PNG 不恢复压缩损失 |
| Recent | All clips、首条选中、无搜索、无置顶 |
| Images | All time、Today 8 张、Yesterday 4 张、首张选中 |
| Passwords | 已解锁参考状态、Work Email 选中、密码隐藏 |
| Settings | Capture On，DELETE 确认框为空，删除按钮禁用 |

一组误回到 1280 × 720 的截图被尺寸检查拒绝，已在重新设置 viewport 后覆盖；该组不作为验收依据。

### 最终证据索引

| 页面 | REFERENCE | IMPLEMENTATION | 整页并排 | 局部并排 |
|---|---|---|---|---|
| Recent | `reference/recent.png` | `artifacts/visual-qa/recent-final.png` | `artifacts/visual-qa/recent-compare-final.png` | `artifacts/visual-qa/focused-recent-final.png` |
| Images | `reference/images.png` | `artifacts/visual-qa/images-final.png` | `artifacts/visual-qa/images-compare-final.png` | `artifacts/visual-qa/focused-images-final.png` |
| Passwords | `reference/passwords.png` | `artifacts/visual-qa/passwords-final.png` | `artifacts/visual-qa/passwords-compare-final.png` | `artifacts/visual-qa/focused-passwords-final.png` |
| Settings | `reference/settings.png` | `artifacts/visual-qa/settings-final.png` | `artifacts/visual-qa/settings-compare-final.png` | `artifacts/visual-qa/focused-settings-final.png` |

以上四页已逐一并排查看，包括局部字体、字段、照片与按钮。截图来源压缩损失不能当成 CSS 模糊问题去锐化照片。

## 迭代记录

| 轮次 | 当轮问题 | 修正 | 复核证据 |
|---|---|---|---|
| 1 | P1：原布局、组件和间距不符合参考；P2：标签拉伸、正文换行和图网格有偏差 | 创建共享外壳与四页，采用截图中的真实照片区域 | `artifacts/visual-qa/*-pass1.png`、`recent-compare-pass1.png` |
| 2 | P2：内容框、按钮、字段和 Yesterday 区域位置不齐 | 校准分栏和逐页关键坐标 | `*-pass2.png`、`recent-compare-pass2.png` |
| 3 | P1：旧全局 CSS 强制覆盖圆角；P2：标题宽度不符 | 将旧样式限制在 legacy-shell，调整标题字面宽度 | `*-compare-pass3.png` 与对应 focused 图 |
| 4 | P2：Images/Settings 错误高亮 All clips，Recent 缩略图错误、行高和正文断行不符 | 修正选中态、提取真实缩略图、按参考逐行设置高度和断行 | `*-compare-pass4.png` |
| 5 | P2：图标轮廓、代码、字段字体、设置分隔线长度不符 | 换用更接近原图的图标轮廓，修正输入字体与分隔线长度 | `*-compare-pass5.png`；依赖热更新导致的一次空白捕获已重新加载后覆盖 |
| 6 | P2：Georgia 字形/宽度、各处标签光学大小偏差 | 对比 Times New Roman，按文字像素边界调整，不采用统一常见字号 | `*-compare-pass6.png` |
| 7 | P2：滚动条遗漏、字段复制按钮偏窄、正文与标题边界偏移 | 补齐参考滚动条状态，修正按钮宽度、文字位置与行内边界 | `*-compare-pass7.png`、像素测量表 |
| Final | 剩余小幅坐标与颜色差异，以及 P2：UI 字体角色、PNG 标签对齐、图片尺寸重采样 | 内容框位置、密码字段间距、主按钮分页面采样色校准；区分 UI 无衬线与展示衬线字体；校准遮蔽圆点与 PNG 标签；重新构建和截图 | `*-compare-final.png`、`focused-*-final.png` |

最终局部复核还修正了照片的尺寸重采样：Today 素材及 CSS 均为 174 × 215，大图素材及 CSS 均为 427 × 480，Yesterday 按素材原尺寸显示。保留源图文字中的 `n- Research`，通过像素列统计确认 Recent 密码条目为 18 个圆点、密码详情为 17 个圆点。导航、密码条目、图片文件名采用 UI 无衬线字体；品牌、展示标题等采用衬线字体。

Final 的第二次局部复核发现：密码列表圆点在 y=238，而参考为 y=243；Images 元数据字面宽度 44px，而参考为 49px；搜索文字字面宽度 151px，而参考为 161px。追加修正了圆点行偏移及字距、搜索框字体与间隔、元数据字体、按钮字体和 Danger zone 标题偏移。修正依据为上述像素差，不是审美调整。

`scripts/visual-compare.ps1` 生成原尺寸并排图与局部图，并拒绝错误 viewport；`scripts/measure-visual.ps1` 测量指定区域内文字的深色像素边界。

## 五项视觉检查

### 1. 字体、字号与排版

- 衬线字体使用 Times New Roman，保留 Georgia fallback；这是经过截图比较的实现候选，不声称找到了原图的确切字体。
- UI 无衬线使用 Segoe UI；代码使用 Consolas。
- 品牌、Recent 大标题、正文、列表标题、设置标题分别测量，不能共用未经校准的字号。
- 最终测量（阈值为 RGB 平均值小于 145，单位 px）：

| 区域 | 参考 x/y/w/h | 实现 x/y/w/h |
|---|---|---|
| Clipboard 品牌 | 33 / 52 / 161 / 39 | 33 / 52 / 161 / 39 |
| Recent 导航文字 | 77 / 147 / 52 / 13 | 77 / 147 / 51 / 13 |
| Common phrases | 72 / 490 / 119 / 16 | 72 / 490 / 118 / 15 |
| Recent clips 标题 | 341 / 117 / 100 / 19 | 341 / 116 / 100 / 19 |
| 选中正文大标题首行 | 818 / 183 / 470 / 30 | 819 / 183 / 470 / 29 |
| Clipboard capture 标题 | 339 / 230 / 151 / 21 | 339 / 230 / 151 / 20 |
| 密码列表圆点 | 342 / 243 / 118 / 4 | 342 / 243 / 118 / 4 |
| 搜索占位文字 | 397 / 42 / 161 / 16 | 397 / 43 / 160 / 16 |
| Images Copied 标签 | 1165 / 610 / 49 / 16 | 1165 / 610 / 50 / 17 |

这些数字是局部文字边界，不是“整体相似度百分比”。

### 2. 布局与间距

- 全部页面共用 306px 侧栏；三页顶栏 92px，Images 不增加顶栏。
- Recent / Passwords / Images 的第二条分栏分别在约 x=784 / 807 / 1101。
- Recent 内容框、Quick actions、右下删除按钮；Passwords 底部固定操作区；Settings 分隔线与 Danger zone，均有并排图复核。
- 参考预览使用原图可见滚动条状态，避免凭空编造屏外数据；真实列表仍按实际数据滚动。
- 未加入原图没有的卡片、页面、阴影、动画或装饰。

### 3. 色彩、边框、圆角

- CSS 变量保存背景、侧栏、激活色、分隔线、危险色和字体。
- 主背景采样为 RGB 250/249/247，与实现该点一致。
- 主按钮按页面分别采样非文字区域，使用平均实色：Recent 186/93/65；Images 192/90/57；Passwords 181/77/54。
- 不凭空增加渐变去模拟源位图不规则的明暗噪点。

### 4. 图片与图标

- Today、Yesterday、选中大图和 Recent 缩略图均来自用户提供截图中的真实图像区域，没有换成网络图、AI 重绘或占位图。
- 图像资产在 `static/reference-assets/`，参考整图在 `reference/`。
- 选中大图资产与参考 x=1127/y=98 区域的采样 RGB 差为 0；并未通过锐化或重绘修改照片。局部显示柔化仍受到浏览器截图采样和边缘栅格化影响。
- 图标使用 Lucide，Markdown 使用 Tabler 的现有图形；调整尺寸、线宽、旋转和边框显示以接近参考。
- 未将整页参考图铺成背景伪装成可交互 UI。

### 5. 文案与内容

- 四页默认参考数据、标签、分组、计数、标题、按钮文字来自原图。
- 原图 Recent 元数据标为 94 characters，而可见示例正文实际为 77 字符：参考状态忠实显示 94，真实记录按内容长度计算。
- 真实数据页面不使用参考计数或示例密码冒充用户数据。

## 交互回归与构建

- 四个主导航切换已测试。
- Recent 搜索 notion 返回 1 条；HTML 复制内容正确；置顶后筛选得到该条记录。
- Passwords 参考数据添加后 8→9 条；复制用户名得到 `qa@example.com`；修改标题可保存；删除测试项后回到 8 条。
- Images All time 为 12 张，Yesterday 为 4 张；选择图片会更新右侧。
- Settings Capture 切换后显示 Off；空确认框不能删除，输入 DELETE 后按钮启用。未执行真实清空。
- 真实模式接回已有密码库创建/解锁/锁定、保存、导入导出、分类、复制、删除、快捷键及外部链接处理器。
- 最终检查窗口的控制台错误和警告列表为空。旧检查页曾留下依赖热更新错误，不作为最终版本运行错误。
- `npm run check`：0 errors，68 个旧页面未使用 CSS selector 警告。新组件无类型错误。不把“有警告”称为全净检查。
- `npm run build`：最终静态构建成功，耗时 56.55s，adapter-static 写入 build；构建后已重新捕获四页。

## 剩余差异与验证边界

- P2（验收证据）：System.Drawing RawFormat 检查确认，浏览器截图虽然最初以 `.png` 命名，实际格式为 JPEG；参考和照片资产是真 PNG。已保留原始 JPEG、将最终实现图解码保存为 PNG，并在对比脚本中自动识别该情况。照片纹理、细线、字形边缘受压缩影响，不能据此得出无损逐像素通过结论；需要能返回无损 PNG 的同 viewport 捕获通道复验。
- P3：原图没有提供字体文件，栅格字形、字重和抗锯齿仍有约 1–2px 的局部差别；仅凭位图不能确定真实 font family / font-weight。
- P3：源图的背景和按钮存在不规则色调、噪点及柔化，实色 CSS 无法逐像素重建这些信息；未擅自添加纹理或渐变。
- P3：部分图标的原始矢量路径未知，现有库的局部轮廓仍有小差别。需要原始 SVG 才能保证路径级一致。
- 原始独立照片未提供；已还原截图可见裁切，但不能保证放大或改变预览比例后有原图之外的内容与分辨率。
- 仅验收 1586 × 992。小窗口、不同系统缩放和不同操作系统字体栅格器不在本次一致性结论中。
- 解锁、错误、空数据、新建分类等交互状态没有提供设计图；保留业务入口，不声称这些状态达到参考图 1:1。
- 原生密码库/系统快捷键/Chrome 启动/窗口按钮未做桌面端端到端验证，不能把浏览器参考状态的通过等同于原生功能通过。
- 正在运行的旧 `my-clipboard.exe` 没有被停止或替换。本次没有重新打包发布桌面程序。
- npm 安装输出报告 6 项依赖漏洞（2 moderate、4 high）；未在视觉任务中执行可能改变其他依赖的自动升级。
- 旧页面 DOM/CSS 暂时封存在 `.legacy-shell`，以保留既有处理器和导入输入节点；这也保留了旧 CSS 警告。未修改 Rust 业务代码及原有未提交工作。

## 实现清单

- [x] 读取四张参考和视觉规范。
- [x] 完成四页静态 UI 与共享 token。
- [x] 多轮同尺寸并排对比及像素测量。
- [x] 接回核心处理器并执行参考数据交互回归。
- [x] 最终构建后再次捕获四页，并复核 full-view 和 focused evidence。
- [x] 更新最终验收结论；严格逐像素验收保留 blocked，不将高一致度冒充完全一致。

## 修改文件清单

- `src/lib/ReferenceClipboard.svelte`：共享外壳、四页、token、参考数据和交互适配。
- `src/routes/+page.svelte`：接入新组件，保留已有业务处理器，隔离旧 UI/CSS，添加密码与分类适配。
- `src-tauri/tauri.conf.json`：1586 × 992 默认窗口、自绘标题栏。
- `src-tauri/capabilities/default.json`：最小化、最大化、关闭、拖动窗口权限。
- `package.json`、`package-lock.json`：图标依赖。
- `docs/visual-spec.md`：补充经过截图测量的字体候选结论。
- `reference/{recent,images,passwords,settings}.png`：保存用户原始设计图。
- `static/reference-assets/images/{today-1..8,yesterday-1..4,selected}.png`、`static/reference-assets/recent-thumbnail.png`：从设计图提取的真实照片区域。
- `scripts/visual-compare.ps1`、`scripts/measure-visual.ps1`：可重复执行的对比与测量。
- `artifacts/visual-qa/`：各轮截图、并排图、局部图。
- `design-qa.md`：本验收记录。

`src-tauri/src/main.rs`、`Cargo.toml`、`Cargo.lock` 等已有未提交更改不是本轮修改，未覆盖。
