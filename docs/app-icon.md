# 应用图标草稿（2026-08-31，未采用）

> 本页仅记录旧草稿；当前已选用的夹子精灵见 [app-icon-clip-sprite.md](app-icon-clip-sprite.md)。

用户否决此方案，转为先讨论软件命名。已中断本次正式构建，恢复原 `src-tauri/icons/` 和 `static/favicon.png`；原桌面产物 SHA-256 仍为 `CC44A8157F658C543F91B4250F027C973F4AA3CF2CCE0E2768F0DFA49FED2AA4`，重新启动的是这个未改变图标的版本。

`assets/app-icon.png` 仅保留为未采用草稿，不被应用引用。以下为草稿生成和检查记录，不代表已交付或已启用；未确认前不要执行下面的重新生成命令。

## 范围

- 新图标：浅象牙底、陶土色剪贴板；用两条横线表示内容。
- 原图：`assets/app-icon.png`，1254×1254 RGBA。通过内置 imagegen 生成并清理透明背景，原始生成文件另行保留在工具输出目录；不是手工 SVG 或远程热链。
- `src-tauri/icons/`：由项目已安装的 Tauri CLI 统一生成 ICO、ICNS 及已有各平台 PNG。
- `static/favicon.png`：与 `src-tauri/icons/32x32.png` 相同。
- 沿用现有 `tauri.conf.json` 的 bundle 图标配置；现有托盘从 `default_window_icon()` 读取，因而无需修改 Rust 或页面代码。
- 本次未修改四个页面布局、导航图标、业务逻辑或用户数据，未提交或推送。

## 重新生成

在项目根目录执行：

```powershell
npm run tauri -- icon assets/app-icon.png --ios-color '#F7F4F0'
Copy-Item -LiteralPath src-tauri/icons/32x32.png -Destination static/favicon.png
npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc
```

## 图像生成记录

初始提示：为 Clipboard 剪贴板管理器制作单一、正面、居中的 Windows 应用图标；浅象牙色 `#F7F4F0` 圆角方块底，陶土色 `#B85C3D` 剪贴板轮廓及顶部夹子，内含两条浅色横线；小尺寸可识别，不含文字、星星、笔、剪刀、阴影或 3D。随后要求清理边缘、统一填充；第二次输出缺少 alpha，最终再执行背景提取。

最后一次编辑的完整提示词：

> Background extraction ONLY. Cut out the complete ivory rounded-square app tile with the orange clipboard mark from this image. Keep the entire ivory tile and orange symbol unchanged. Remove the outer white canvas beyond the ivory rounded-square shape, replace ONLY that outside region with TRUE TRANSPARENT ALPHA. The final file MUST be RGBA PNG with fully transparent corner pixels, not white, not black, not a checkerboard baked into the image. Preserve the square canvas and icon scale. Clean smooth anti-aliased rounded-square outline with no speckles or edge debris. Do not redesign or add anything.

## 验证

- `npm run check`：0 errors / 0 warnings。
- 48 个 PNG 均可解码。ICO 包含 16、24、32、48、64、256 像素图层，各层左上角 alpha 为 0。
- ICO 的 32 像素图层与同尺寸 PNG 像素一致；favicon 与该 PNG 文件内容一致。
- 已查看 16、32、48、128 像素效果，缩小后仍能辨认夹子和内容横线；图像生成原图在大尺寸下仍有轻微边缘不规则，不宣称矢量级几何精度。
- 16 / 48 像素的 Windows ICO 解码预览保存在 `output/playwright/app-icon-16.png` 和 `app-icon-48.png`。
- 未发现桌面或固定任务栏目录中名称包含 clipboard 的现有快捷方式；未新增或修改快捷方式，也未清理 Windows 图标缓存。其他安装目录中的旧 EXE 不属于本次替换范围。
