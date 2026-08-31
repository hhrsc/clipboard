# 多窗口尺寸适配与验收

日期：2026-08-31。范围：现有 Clipboard 四页，不重新设计，不修改数据保存、密码、剪贴板或快捷键注册逻辑。

## 交付

- EXE：`src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe`
- 正式构建：`npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc`
- 时间：2026-08-31 16:19:07，本机时区；13,262,848 字节。
- SHA-256：`0764792251F284CD60EE92CCFE724A2E3051C37FD5AC8B7F0383F4112F9D62D8`
- 不替换或关闭正在运行的旧程序，不自动启动新程序；未提交或推送。
- `build/recovery-import.json` 不存在；原始恢复文件及私人备份保持原状。

退出旧版时请使用托盘 Quit，再打开上述 EXE。标题栏关闭按钮仍是隐藏到托盘，不等同于退出。

## 布局规则

尺寸均为 CSS / 逻辑像素，不是 Windows 缩放后的物理屏幕像素。

| 窗口条件 | 布局 |
|---|---|
| 1586×992 基准 | 四页保留本轮修改前的静态视觉结果 |
| 较小、较矮或更大的窗口 | 原多栏改为弹性列宽；内容按可用空间滚动，不整体缩放页面 |
| 宽度 ≤1100 | 侧栏收为左上角导航按钮；展开后可切页、访问分类，Esc/遮罩可关闭 |
| 宽度 ≤760 | Recent、Passwords 的列表与详情上下排列；Images 图库与预览上下排列 |
| 宽度 ≤480 | 图片搜索和日期控件分行，避免搜索框只剩几个字符 |
| 高度 ≤500 | 侧栏整体滚动，避免分类列表被压到无法操作 |
| 浮层 | 非基准布局中独立于滚动容器定位；按剩余空间向上或向下展开，并限制宽高 |

- 多行按钮、设置说明、密码复制按钮、底部操作区均允许换行或随内容滚动。
- 隐藏侧栏不可聚焦；抽屉展开时背景主区不可聚焦；扩大窗口恢复侧栏和主区交互。
- 快捷键仍是按键录制，不改回文本输入；切换窗口尺寸不改变其保存流程。
- Tauri 最小窗口为 360×400；默认仍为 1586×992。启动时将窗口物理尺寸及位置限制在当前显示器工作区，考虑非客户区，避免超出任务栏边界。查询失败记录错误，不阻止启动。
- 保留原配色、图标、圆角、素材和主要排版；按 web-design-engineer 技能复用已有设计，仅补充本次授权的响应式布局。

## 已执行检查

| 检查 | 结果 | 边界 |
|---|---|---|
| `npm run check` | 0 errors / 0 warnings | 最终 UI 类型检查 |
| `cargo check --manifest-path src-tauri/Cargo.toml --tests` | 通过 | 编译测试目标，未执行原生测试 |
| 正式 Tauri release | 通过 | 无安装包，交付独立 EXE |
| 四页 ×19 个尺寸 | 76/76 通过 | 检查主要布局横向溢出、视口边界、DPR、缩放；保存 PNG |
| 浮层、滚动和按钮可达性 | 116/116 通过 | 7 种尺寸；参考合成数据；复制/删除/保存等只检查可点击性，不执行真实操作 |
| 开着菜单连续调整尺寸 | 9/9 通过 | 浮层不重复、不越界；回到基准恢复原容器；关闭后无残留；抽屉恢复正常 |
| 实际 UI 入口，合成 IPC | 26/26 通过 | 空 Recent/Images、新建及解锁密码库表单、原生设置区；未访问桌面数据 |
| 已有快捷键回归 | 20/20 通过 | 合成 IPC 验证录制、错误、重试、确认后更新；不是系统热键验收 |
| 360×400 快捷键错误浮层 | 通过 | 长错误信息不越界；Esc 后无残留 |

19 种尺寸：1586×992、1366×768、1280×720、1024×768、800×600、640×480、360×640、360×400、1586×500、800×1200、1920×1080、2560×1440、760×700、761×700、1100×768、1101×768、1585×992、1699×1099、1700×1100。

浏览器为独立 headless Chrome，DPR=1，zoom=100%，截图 PNG，无额外缩放。测试未出现前端运行时异常；Chrome 仍有密码表单 autocomplete/username 的 VERBOSE 建议，未为了消除建议添加设计中不存在的字段。

## 视觉比较

以本轮修改前的实际 UI 为回归基线，而不是声称已经修复此前缺失字体等素材差异。

固定 1586×992、每页在该尺寸重新加载并等待字体/图片后截图，Pillow 逐通道比较 RGBA：

| 页面 | 变化像素 |
|---|---:|
| Recent | 0 |
| Images | 0 |
| Passwords | 0 |
| Settings | 0 |

证据保存在忽略提交的 `output/playwright/`：

- `responsive-before-{recent,images,passwords,settings}.png`：本轮修改前。
- `responsive-after-{recent,images,passwords,settings}.png`：最终基准截图。
- `responsive-{page}-{width}x{height}.png`：多尺寸矩阵。
- `responsive-drawer-*`、`responsive-recent-actions-*`、`responsive-password-actions-*`、`responsive-shortcut-*`：浮层、侧栏与底部操作。
- `responsive-vault-gate-*`、`responsive-native-settings-*`：合成 IPC 下的实际入口。

已查看代表性小、中、大、宽矮窗口截图，并修复分类菜单越界、快捷键浮层越界、窄屏弹窗溢出、图片搜索过窄和极矮窗口侧栏分类区不可达。没有用截图后缩放掩盖问题。

## 用户桌面验收

1. Quit 旧版，打开本次 EXE；小屏幕启动时确认窗口及关闭按钮没有落到屏幕外。
2. 四页分别拖动到大窗口、左右分屏、约 800×600、640×480、最小窗口；检查没有横向裁切，滚动可到达底部操作。
3. 窄窗口展开导航，切换四页并选择分类；矮窗口向下滚动侧栏查看最后一个分类。
4. 打开分类菜单、密码行菜单或快捷键录制，再拖动改变尺寸；确认菜单内所有项目可以操作，Esc 后没有浮层残留。
5. Recent 选中长文本或图片；Images 调整筛选；确认选中状态和预览在尺寸变化后保留。
6. Passwords 在小窗口检查用户名、密码复制按钮及保存按钮可达；不需要为本次布局验收操作真实密码。
7. Settings 展开 Desktop & storage，检查保留时间、自启动按钮和错误提示；不要点击清空数据。
8. Windows 125%/150% 缩放及多显示器之间移动后复验；最大化、还原、最小化、托盘隐藏/唤出复验。

尚未执行 Windows 原生启动/拖动/系统缩放/多显示器验收。浏览器测试只证明其列出的视口与交互结果，不等同于桌面 E2E 完成。

## 修改范围与复跑

- `src/lib/ClipboardApp.svelte`：响应式布局、导航抽屉、滚动和浮层接入。
- `src/lib/responsive-popover.ts`：滚动容器外浮层定位及清理；基准视口保留原位置。
- `src-tauri/src/main.rs`：启动窗口限制在显示器工作区。
- `src-tauri/tauri.conf.json`：最小窗口尺寸。
- `scripts/responsive-*.js`：本轮回归脚本；`scripts/shortcut-editor-qa.js` 复用未改。

开发服务运行于 1420 后，用 Playwright CLI 和 `scripts/pixel-qa.config.json` 启动独立会话，依次 `run-code --filename scripts/responsive-qa.js`、`responsive-interaction-qa.js`、`responsive-resize-qa.js`、`responsive-baseline-qa.js`。`responsive-native-ui-qa.js` 和 `shortcut-editor-qa.js` 各用独立会话，避免模拟 IPC 污染参考模式。

结论：实现、编译和上述浏览器回归通过；桌面实际体验待用户验收。不沿用旧阶段的 PASS 作为本轮原生验收结论。

后续启动记录：用户授权自动重启后，2026-08-31 16:29 已退出旧实例并启动本报告哈希对应的新版 EXE；确认仅一个应用进程、路径正确且 Responding=true。该记录仅确认进程启动，不替代上面的桌面功能与窗口体验验收。后续开发完成后自动重启的要求已写入 `AGENTS.md`。
