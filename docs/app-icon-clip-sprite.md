# 已采用：夹子精灵 A1 大圆角版

2026-08-31，用户提供 A1 原图并要求替换应用图标，随后要求更饱满的 Apple 风格圆角，并同意本地几何遮罩处理。本次保留原图的陶土橙背景、象牙色主体和深色夹头，不重新绘制或改变构图。

## 资源

- 原图：`assets/app-icon-clip-sprite.png`，1254×1254；SHA-256：`4d4c58df2550486aad55be2cf69ada5ddcbaf64831f5d59cde52f4e6b37369a0`，与用户附件一致。
- 当前图标源图：`assets/app-icon-clip-sprite-rounded.png`。采用五次超椭圆和 4 倍超采样，只有 alpha 通道改变，所有 RGB 像素与原图一致。未采用生成器返回的棋盘格背景图。
- 已替换：`src-tauri/icons/` 顶层桌面 PNG、ICO、ICNS，以及 `static/favicon.png`。
- 未改动：Android/iOS 目录、界面布局、业务代码、用户数据。
- 旧资源副本：最初图标位于 `output/app-icon-clip-sprite/previous/`，A1 直角版位于 `output/app-icon-clip-sprite-rounded/previous/`。此前未采用的 `assets/app-icon.png` 保留，仍不用于生成当前图标。
- 托盘继续使用 `src-tauri/src/main.rs` 中的 `default_window_icon()`；无需增加独立图标路径。

## 重新生成

在项目根目录执行。先生成到独立目录，再只复制桌面资源，避免覆盖移动端配置。

```powershell
python scripts/make-rounded-app-icon.py
npm run tauri -- icon assets/app-icon-clip-sprite-rounded.png --output output/app-icon-clip-sprite-rounded/generated --ios-color '#BD694F'
Get-ChildItem -LiteralPath 'output/app-icon-clip-sprite-rounded/generated' -File |
  Where-Object { $_.Extension -in '.png', '.ico', '.icns' } |
  Copy-Item -Destination 'src-tauri/icons'
Copy-Item -LiteralPath 'src-tauri/icons/32x32.png' -Destination 'static/favicon.png'
npm run check
npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc
python scripts/verify-exe-icon.py src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe src-tauri/icons/icon.ico output/app-icon-clip-sprite-rounded/verified.ico
```

运行中的旧 EXE 会锁定构建输出，构建前需核对并退出本项目旧实例。只有正式构建成功后才能启动 `src-tauri/target/x86_64-pc-windows-msvc/release/my-clipboard.exe`。

## 已验证

- `npm run check`：0 errors / 0 warnings。
- 圆角源图为 RGBA，四角 alpha 为 0；深浅背景预览位于 `output/app-icon-clip-sprite-rounded/preview.png`。
- ICO 包含 16、24、32、48、64、256 像素图层；32 像素图层与 PNG 像素一致。
- ICO 各层均具有透明度；24 像素层经 Tauri 缩放后角落 alpha 为 3/255，其余尺寸角落为 0。
- 顶层 PNG 均可解码；favicon 与 32×32 PNG 文件一致。
- 已查看 32 / 256 像素资源，与选定原图的构图、颜色一致。
- 旧 EXE 的六个图层均与新资源不同；其 `resource.lib` 时间仍是 15:10，而 EXE 已在 21:52 重新编译。实际构建输出没有跟踪 ICO 文件，导致重用了旧图标资源。
- 已在 `src-tauri/build.rs` 添加 `cargo:rerun-if-changed=icons/icon.ico`，后续图标修改会重新生成 Windows 资源。
- `scripts/verify-exe-icon.py` 只映射 EXE 的 PE 资源，不执行程序，不使用 Shell 图标缓存；逐尺寸比较完整 RGBA 字节，避免透明度差分掩盖 RGB 差异。
- 本次正式 Tauri 构建成功：`npm run tauri -- build --no-bundle --target x86_64-pc-windows-msvc`。EXE 时间为 2026-08-31 22:12:10，SHA-256 为 `5b922fe8fff0986e20c3f0a896248872ffe1a23b1b40ff5a646faeabbd16a1ab`。
- 从最终 EXE 直接导出的六个尺寸均与本次 ICO 的 RGBA 字节完全一致，验证记录与提取图标位于 `output/app-icon-clip-sprite-rounded/verified.ico` 和 `verified.png`。
- 22:12:53 自动启动本次新 EXE，PID 35160，路径与目标 release 路径一致，Responding=True；未发现遗留旧实例。
- 已恢复最小化窗口，并通过 computer-use 的运行截图确认应用正常显示。未操作真实密码、清空数据或执行提交/推送。
- 无边框主窗口没有通过 WM_GETICON 暴露独立窗口 HICON；托盘仍由现有代码读取默认图标。本次已核对 EXE 图标、生成资源和启动状态，未逐项验收托盘交互或其他桌面功能。
