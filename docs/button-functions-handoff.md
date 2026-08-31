# 按钮功能交付与手动验收

日期：2026-08-31。状态：功能已实现，最终桌面和视觉验收由用户执行；不沿用 Phase 2 的 PASS 结论。

## 启动与数据注意事项

- 正式构建使用 `npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc`，避免覆盖正在运行的旧版 exe。
- 新产物：`src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe`。
- 验收前在托盘选择旧 APP 的 **Quit**。关闭窗口只是隐藏到托盘；两个版本同时运行会争用快捷键及同一份存储。
- 实现阶段未启动新 exe，未操作真实密码、系统剪贴板或执行重置。没有自动提交或推送。
- v2 密码库继续使用原主密码和加密算法。读取 v1 后，首次成功保存升级到 v2；旧程序会拒绝 v2，不会静默删除分组。请先自行保管可用的加密备份。
- 导出的 JSON **包含明文密码**。测试请使用下述合成文件，不要把真实导出文件加入 Git 或发给他人。

## 隔离验收（尤其是完整重置）

不要在正式数据上测试破坏性操作。提供了独立 identifier：`com.lenovo.my-clipboard.qa-buttons`。

在项目根目录执行：

```powershell
npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc --config scripts/buttons-qa.config.json
```

该命令把同一目标路径的 exe 替换为隔离 QA 构建，不修改源码中的正式 identifier。QA 版与正式版使用不同应用存储，但共用 Windows 剪贴板和全局快捷键，不能同时运行。需要回到正式构建时，重新执行不带 `--config` 的构建命令。以上 QA 构建及运行没有在本轮代替用户执行。

## 功能入口与期望结果

| 功能 | 手动步骤 | 期望 |
|---|---|---|
| 密码筛选、排序 | 建库后导入 `scripts/fixtures/buttons-v2.json`；点 Password vault 右侧筛选图标 | 全部/有用户名/无用户名正确；最新/最早/标题 A–Z 正确；与搜索及当前分组共同生效 |
| 每行操作 | 点击某行 `…`，复制、编辑、删除 | 操作只作用于该条；菜单按钮不触发行选择；编辑聚焦 Title |
| 分组创建 | 点击 Collections `＋`，创建新分组；尝试空名和重名 | 有效名称保存，空名和重名被拒绝并保留输入 |
| 移动、重命名、删除 | 行菜单或详情 `…` → Move to collection；右键侧栏分组 | 数量同步；重命名不影响密码；删除分组有确认，密码保留在 All items |
| 分组内新增 | 进入一个分组后 Add password；保存、锁库、解锁、重启 | 新密码默认在该组；分组及归属恢复；锁库后不显示名称或草稿 |
| 保存失败 | 仅在隔离环境模拟文件不可写 | 不显示成功；编辑草稿保留，显示保存错误 |
| JSON 兼容 | 导入既有 `scripts/fixtures/passwords-qa.json`，再导入 v2；导出后在隔离库导回 | 旧数组/v1 可读；v2 含分组；按分组名合并并重映射 ID；重复密码不改变已有归属 |
| 图片分组 | 复制两张测试图；在 Recent 分别分配 Collection，然后打开 Images | 侧栏数量为图片数；分组、日期、搜索同时过滤；无结果时预览清空 |
| Clear images | 在筛选后的 Images 中点击清空 | 明确确认清空所有图片，不只是当前筛选；取消不删除 |
| HTML 捕获 | 浏览器打开 `scripts/fixtures/buttons-format.html`，选择正文后复制 | Recent 出现纯文本预览，同时保留可用 HTML；不把外部 HTML 渲染进 APP |
| Markdown | 对上述记录点 Copy as Markdown，粘贴到文本编辑器或 Markdown 编辑器 | `#` 标题、列表、链接、引用、围栏代码和普通表格转换正确 |
| HTML 富文本 | Copy as HTML，分别粘贴到支持富文本的编辑器和记事本 | 富文本目标保留结构，记事本得到备用纯文本，不是 HTML 标签源码 |
| 旧纯文本、换行 | 普通文本含 `*`、`#`、`<`、CRLF；使用各复制按钮 | Markdown 转义特殊字符；HTML 安全转义；去换行不残留 CR；普通复制不改变内容 |
| 去重 | 同一文本带不同 HTML 格式重复复制，再从 APP 复制 | 不遗漏格式变化；自身复制不制造重复历史 |
| HTML 降级 | 仅在测试环境复制超过 1 MiB 的 HTML | 保留纯文本并提示降级；不丢掉整条记录 |

复杂 CSS、嵌入对象、脚本、远程图片不保留，也不会为了格式转换主动访问外部资源。只有纯文本时，不猜测它原本是否为 Markdown。

## 完整重置：只用隔离数据验收

1. 在隔离版创建测试历史、图片、密码和分组，修改保留时间、快捷键并开启该 QA 版自启动。
2. 准备一个应用外部的测试导出文件；记录系统剪贴板当前内容。
3. Settings 输入 `DELETE` 后点 Delete。确认历史、密码库、分组、应用图片缓存及自动备份消失。
4. 确认 Capture On、24 小时、Alt+C、自启动 Off；外部导出文件、工作区恢复目录和系统当前剪贴板保留。
5. 不再次复制时历史应保持空；复制新内容后才出现新记录。重启后旧内容不能复活。
6. 仅在隔离环境测试失败：默认 Alt+C 被占用时，应在删除数据前报错；重置中断后应进入 Settings 提示重试，不加载旧数据；排除错误后再次输入 DELETE 完成。

重置使用原生标记、应用范围的文件清单和前端完成确认，不递归删除整个 AppData/WebView 目录。失败不宣称全部成功；标记未完成时阻止继续修改。

## 视觉验收

在 1586×992 对比 `reference/` 四页。沿用已有字体、颜色、间距与弹窗样式；不调整常驻布局。新增交互只有筛选菜单、行菜单、分组弹窗和删除确认。正式数据的分组、数量及重置说明会与参考示例文字不同。

### Recent 分类菜单遮挡修复（2026-08-31）

- 原因：分类栏的 `transform` 形成独立层叠上下文，后绘制的正文 `transform` 层盖住菜单并截获点击。使用长文本复现时，8 项中有 7 项被遮挡。
- 修复：仅为 `.clip-control-row > label` 设置 `z-index: 1`，不改布局、字体、背景色或正文样式；继续使用已有分类保存回调。
- 浏览器回归：`scripts/collection-picker-qa.js` 在 Reference 合成数据中逐项验证 8 个分类的点击命中、选中值更新及菜单关闭，全部通过。
- 截图：1586×992、DPR 1、缩放 100%、PNG；`output/playwright/collection-before.png` 和 `collection-after.png` 已目视比较。菜单关闭时，修复前后 PNG 字节完全一致。浏览器控制台 0 errors / 0 warnings。
- 复跑：启动本地开发服务后，用 Playwright CLI 的 `scripts/pixel-qa.config.json` 打开 Reference Recent，再运行 `run-code --filename scripts/collection-picker-qa.js`。
- 未操作真实历史或密码。原生保存及重启后分类保留仍由用户验收：选一条测试记录 → 更换分类 → 进入目标分类确认 → 重启再次确认。

## 本轮检查记录

### 自定义快捷键修复（2026-08-31）

- 设置改为只读按键录制器：点击入口自动聚焦，直接读取 keydown 的实际键码与修饰键；不接收打字、粘贴或文本插入。按下修饰键立即显示，松开完整组合后保留结果；重新打开开始新的录制，尚未录到主键时 Apply 禁用。
- 独立草稿只有在 Apply 后、原生注册与保存成功时才更新当前快捷键。Esc/再次点击入口取消，Tab/Shift+Tab 用于焦点导航；失败保留录制结果及原生错误详情。
- 录制与注册分开：可以识别 WebView 收到的非字母键，但本次未扩大后端允许范围（Ctrl/Alt + 字母或数字，可加 Shift，支持三修饰键）。不符合范围或被占用的组合由原生校验报错；系统已截获、未传给 WebView 的按键不能保证录到。
- 原生规范化修饰键顺序；同一组合键不重复注册。若状态中记录了快捷键、但本进程没有实际注册，则重新注册，避免直接返回成功。注册失败保留 Windows/插件错误详情。
- `scripts/shortcut-editor-qa.js` 在实际页面和业务控制器上注入隔离模拟 IPC：20 项通过。用按下/松开事件验证只读、自动聚焦、修饰键显示、完整组合保留、非字母键识别、逐键替换而非拼字、拒绝文本插入及焦点导航，并覆盖此前的保存/错误/重试流程。**这不是 Windows 原生热键/落盘测试**。旧 `scripts/native-e2e.js` 同步改用 `press('Alt+V')`，没有执行原生 E2E。
- 1586×992、DPR 1、100% 缩放下，`output/playwright/shortcut-settings-before.png` 与 `shortcut-settings-after.png` 哈希相同；错误弹出层截图 `shortcut-error.png` 已检查。未改变 CSS；遵循 web-design-engineer 技能复用现有样式。
- 只读录制版的 `shortcut-recorder-settings.png` 与上述修改前 PNG 哈希仍相同；`shortcut-recorder.png` 展示直接按 Alt+Shift+K 得到的录制结果。
- `npm run check`：0 errors / 0 warnings；`cargo check --manifest-path src-tauri/Cargo.toml --tests`：通过，未执行原生测试。没有修改用户快捷键配置或关闭正在运行的旧版。
- 桌面验收：托盘 Quit 旧版，打开最新 EXE；更改为未占用的组合键并 Apply → 关闭窗口到托盘 → 按新快捷键唤出窗口 → 检查旧快捷键不再由本 APP 响应 → Quit/重启后复验。如失败，请保留弹出层的完整错误及所用组合键。

- `npm run check`：0 errors、0 warnings。
- `cargo check --manifest-path src-tauri/Cargo.toml --tests`：通过，只编译测试目标，未执行测试。
- 正式 Tauri release 构建：通过，包含锁库、异步导入、图片选中状态、分类菜单遮挡、自定义快捷键修正及只读按键录制。
- 最终 exe：13,258,752 字节；构建时间 2026-08-31 14:32（本机时区）；SHA-256：`F5AF7A1158D8350A084B8D711A1B2B459AC71F4C10C44B7E14A70C12FB648006`。
- 发布目录确认不含私人 `recovery-import.json`；旧版进程未关闭，新版未启动。
- 桌面 E2E、像素截图、真实粘贴、密码升级、重置：**待用户验收**。
- 安装依赖时 npm 报告 6 条安全告警；生产依赖审计指出现有 `svelte`、`devalue` 共 2 个告警包。未为清告警升级框架。
- 原 `design-qa.md` 的 Phase 2 测试数字和二进制哈希不适用于本次新增功能。

## 主要改动

- `src/lib/ClipboardApp.svelte`：密码筛选/菜单/分组、图片分类数量及清空确认，保留原视觉结构。
- `src/routes/+page.svelte`：完整快照保存、分组 CRUD、v2 导入导出、富文本监听/复制、重置握手。
- `src-tauri/src/main.rs`：v1/v2 密码库、HTML 快照和多格式写入、存储操作锁、可重试完整重置。
- `src/lib/clipboard-format.ts`、`src/turndown-plugin-gfm.d.ts`：安全 HTML 转换、Markdown、格式敏感指纹及类型声明。
- 依赖和锁文件、隔离配置与合成素材；两份旧 QA 脚本仅同步密码快照返回结构，本轮未运行。

工作区已有的敏感恢复逻辑、私人备份和未追踪材料未删除、未提交，不作为测试数据。

## 后续交付：多窗口尺寸适配（2026-08-31）

最新构建与上文按钮/快捷键阶段记录分开：本次 EXE 为 2026-08-31 16:19:07 构建、13,262,848 字节，SHA-256 为 `0764792251F284CD60EE92CCFE724A2E3051C37FD5AC8B7F0383F4112F9D62D8`，路径不变。

详见 [responsive-qa.md](responsive-qa.md)：四页基准 PNG 零像素变化；增加窄屏导航、上下布局、短窗口滚动和浮层避让。76 项布局组合、116 项交互检查、9 项连续调整尺寸检查、26 项实际入口合成 IPC 检查、原 20 项快捷键检查通过；Windows 实际窗口行为仍待用户验收。

## 后续修改：主密码长度（2026-08-31）

- 新建及更换主密码改为 **8–16 个字符，含 8 和 16**；前端和 Rust 均按 Unicode 字符计数，不新增字符种类要求，不截断输入。
- 新建表单沿用现有样式，仅在输入框中显示长度提示。长度不符在调用原生接口前提示，两次输入仍需相同；原生创建入口也校验。
- 旧密码库解锁、备份恢复不套用新密码长度上限，以免旧长密码失效；更换后的新密码必须符合新规则。
- 5 项 Rust 主密码单元测试通过：长度边界、加密往返、错误密码、轮换及旧长密码兼容。只用合成数据，未访问真实密码库。
- `scripts/master-password-length-qa.js`：实际页面加隔离模拟 IPC，21 项检查通过。覆盖 7/8/16/17 字符、中文及非 BMP 字符、确认不一致、原生错误反馈、窄窗口和旧密码解锁提交；没有执行真实建库。
- 1586×992 PNG 比较：显示/隐藏长度提示的差异仅在两个输入框内，框外变化像素为 0；360×400 截图已查看。证据位于 `output/playwright/master-length-*.png`。
- 旧隔离 E2E 脚本改用符合新规则的建库密码，并兼容原有隔离测试库；本轮未执行这些原生 E2E。
- `npm run check`：0 errors / 0 warnings。桌面建库的实际使用由用户验收。
- 正式 Tauri 构建通过，2026-08-31 16:54:30，13,262,848 字节；SHA-256：`0648E95F374EE3918099C80C9A48E98372DB03CF5F29661E6C6E6E95FD8D804D`。EXE 路径同上。
- 已按项目约定退出旧实例并启动新版本，进程路径正确且响应正常；没有创建或修改正式密码库。

## 后续修改：Characters 与 Images 分离（2026-08-31）

- 导航及列表标题从 Recent 改为 Characters；仅显示非图片剪贴板记录，搜索、置顶筛选、分类计数和详情选中规则同步排除图片。
- Images 专门显示图片，保留原记录、图片文件、分类关联、预览、复制及删除能力。未迁移磁盘目录、未清空历史、未修改原保留策略。
- 图片缩略图右键可更改 Collection，复用现有浮层样式；在筛选分组内移走最后一张图片时清空预览。Images 底部总数按图片统计，点击后仅清除筛选，不跳回 Characters。
- 内部 `recent` 状态键及旧 `?reference=recent` 地址保留兼容；增加 `?reference=characters` 别名，不保留第二套正式 UI。
- 修改业务/UI 文件：`src/routes/+page.svelte`、`src/lib/ClipboardApp.svelte`。旧验收脚本 `native-e2e.js`、`production-smoke.js`、`responsive-native-ui-qa.js` 同步名称/空状态；未运行会操作真实数据的原生验收脚本。
- `scripts/characters-images-qa.js`：33 项隔离浏览器检查通过。覆盖混合历史分离、图片排除计数/搜索/置顶、仅图片历史的文字空状态、原记录完整保留、图片分类保存及页面重载、实时模拟捕获、选中态、菜单退出和大小窗口可达性。所有 IPC 都是合成实现，不连接桌面实例、不读写系统剪贴板或真实密码。
- 1586×992、DPR 1、100% 缩放 PNG 对比：四页分栏位置/尺寸不变。Characters 变化 7,760 像素，仅在改名及移出图片的列表区域；Images、Passwords、Settings 各变化 577 像素，仅在导航文字区域。上述预期区域外变化均为 0。Images 初次前截图存在图片未解码完成的问题，像素对比改用 2026-08-31 16:19:17 的完整 `responsive-after-images.png`；不使用不完整截图判定回归。
- 截图位于 `output/playwright/characters-*.png`；1586×992 和 360×400 的实际入口截图、右键菜单已查看。按 web-design-engineer 技能沿用现有布局和视觉样式，仅增加操作时的图片分类浮层。
- `npm run check`：0 errors / 0 warnings；限定修改文件的 `git diff --check` 无空白错误，Git 仅提示现有 LF/CRLF 转换策略。
- 用户桌面验收：确认 Characters 只有文字、Images 中原图片仍在；分别复制新文字/图片检查归属；右键图片换分类后重启复查。浏览器检查不替代 Windows 原生剪贴板及磁盘持久化验收。
- 正式构建 `npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc` 通过；2026-08-31 17:19:52，13,262,848 字节；SHA-256：`72428381960148CCE24349F115EC74C9258E471492849C5D87B73F4709B04588`。产物：`src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe`。
- 已自动重启到该产物，17:20:19 启动，PID 41432，路径及响应状态正常且只有一个 APP 实例；构建目录不含私人恢复 JSON。未提交、未推送。

## 后续修改：打开密码本是否需要密码（2026-08-31）

### 使用方式

- Settings → **Password vault access**。On（默认）：需要主密码；Off：当前 Windows 登录账户可直接打开 Passwords。
- 首次仍需在 Passwords 创建密码本。开关只改变打开方式，不删除或重设主密码。两个方向的切换都要求输入当前主密码；原有长主密码仍可用于验证。
- 关闭后，后台不再执行五分钟自动锁库，手动 Lock vault 按钮禁用并提示开启密码要求。重新开启后立即锁库并清除前端条目、分组和编辑草稿。
- 未创建密码本、状态未加载或不支持本机解锁的平台会禁用开关并显示原因。错误密码、保存失败不会改变开关；异步保存中不能重复确认或取消。
- 本机凭据不可用时，保留原密码库并提示输入主密码，不创建空库、不持续重试。仍需妥善保留主密码用于恢复与加密备份。

### 原生实现与安全范围

- 原有 Argon2id + AES-GCM 密码库文件不变。Windows 用户级 DPAPI 保护解密密钥，保存至本应用数据目录的 `password-vault-unlock.bin`；不保存明文主密码、明文密钥或明文密码快照，不使用整机共享标志。
- 本机凭据绑定应用 identifier 和密码库 salt；读取有大小限制，解密后仍检查密码库认证。设置切换经过主密码验证并串行写入；不存在凭据文件时默认需要密码。临时文件不作为有效开关状态。
- 密码库创建、更换主密码、恢复加密备份、删除及完整重置都清除本机凭据；更换/恢复后下次启动默认需密码。凭据不进入导出的加密备份。
- 免密码打开的边界是当前登录账户：任何能使用该 Windows 会话的人都可打开密码本。这不是 Windows Hello 或另一层身份验证。DPAPI 的用户范围及释放规则依据 [Microsoft CryptProtectData](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata) 和 [CryptUnprotectData](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata)。
- 修改：`src-tauri/src/local_vault_unlock.rs`（新增）、`src-tauri/src/main.rs`、`src-tauri/Cargo.toml` / `Cargo.lock`、`src/routes/+page.svelte`、`src/lib/ClipboardApp.svelte`。

### 验证与待验收

- `cargo test --manifest-path src-tauri/Cargo.toml --bin my-clipboard vault -- --nocapture`：5 项通过，含 DPAPI 往返、上下文绑定/损坏拒绝、临时凭据替换/删除、原加密库往返/错误密码、切换必须验证主密码且不改变加密库内容。测试仅使用生成的临时文件及合成密码，不读取真实库。
- 新增 `scripts/vault-access-qa.js`：27 项隔离浏览器检查通过，覆盖默认值、确认/取消、错误密码、保存错误、保存等待状态、模拟重启、自动打开、分组、重新开启锁定和内存清理、旧主密码继续使用、凭据失败回退、窄窗口及未创建库。模拟 IPC 不代表实际 Windows 端到端验收。
- `npm run check`：0 errors / 0 warnings。浏览器无 runtime error；Chrome 有密码表单 autocomplete/username 的 VERBOSE 建议，不是运行异常。
- 按 web-design-engineer 技能复用原设置行、开关及弹窗样式，不修改原 CSS。1586×992、DPR 1、100% 缩放的四页 reference 模式 PNG 与上一版逐像素相同；正式 Settings 增加一行。查看了正式设置截图和 800×600 / 360×400 确认弹窗；修复成功提示遮挡新弹窗，最终截图 `output/playwright/vault-access-dialog-final-360x400.png`。其余截图为 `output/playwright/vault-access-*.png`。
- 用户验收：先建库 → Settings 关闭开关并验证主密码 → 重启 → 打开 Passwords 不再输入；再开启开关 → 确认立即锁定、重启后需主密码；核对原条目/分组不变。真实密码库、实际重置及备份恢复的操作由用户验收，本轮没有代为执行。
- 正式 Tauri 构建通过：2026-08-31 18:03:05，13,282,304 字节；`src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe`，SHA-256 `6AF9EB395D3B75D14432947D27B57A9633C636330C05749381928405B6DA5E04`。
- 已自动退出旧实例并启动该产物；18:03:52 启动，PID 42132，路径及响应状态正常，只有一个 APP 实例。未更改用户真实密码本的开关状态，未自动提交或推送。

## 后续修改：Characters 导航图标（2026-08-31）

- `src/lib/ClipboardApp.svelte`：Characters 从时钟改为现有 Lucide `Type` 文字图标。沿用 web-design-engineer 技能要求的现有样式，24×24 尺寸、1.55 线宽、颜色、间距及位置不变；时间元数据仍使用时钟。
- `npm run check`：0 errors / 0 warnings；限定文件 `git diff --check` 无空白错误，仅提示现有换行转换策略。
- 1586×992、DPR 1、100% 缩放 PNG 前后比较：218 个变化像素全部位于原图标 24×24 区域，区域外变化为 0。已查看 `output/playwright/characters-icon-before.png`、`characters-icon-after.png` 及 `characters-icon-narrow.png`；360×400 下抽屉图标可见且页面切换正常。浏览器使用参考数据，没有读写真实密码或剪贴板。
- 正式 Tauri 构建通过：2026-08-31 19:57:21，13,282,816 字节；产物路径仍为 `src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe`，SHA-256 `CC44A8157F658C543F91B4250F027C973F4AA3CF2CCE0E2768F0DFA49FED2AA4`。
- 已自动重启新版本：19:57:47 启动，PID 8252，路径正确、响应正常，只有一个 APP 实例；构建目录不含私人恢复 JSON。没有提交或推送。
