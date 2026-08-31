# Product design source of truth

`/reference` = product design source of truth

When implementing UI:

Reference screenshots have absolute visual priority.

Never redesign the UI.

Never "improve" the design unless explicitly requested.

All layout, spacing, typography, color, radius, shadow,
image cropping and alignment must match reference screenshots.

For UI tasks use this loop:

inspect reference
→ implement
→ run
→ screenshot
→ compare
→ adjust
→ repeat

Do not declare a UI task complete after implementation alone.

A visual comparison must be performed before completion.

# Development handoff

完成本项目的实现或修复任务后，自动重启应用供用户验收，不再要求用户手动退出和启动。

- 先完成检查和正式 Tauri 构建，只启动本次成功构建的 EXE，不使用可能过期的产物。
- 重启前确认旧进程的完整路径，优先正常退出；必要时只结束本项目的旧实例，不影响其他应用。
- 启动后检查新进程的路径及运行状态，确认没有遗留旧实例；构建或启动失败必须如实报告。
- 重启不代表桌面功能验收通过，不操作真实密码、不清空数据，也不自动提交或推送。
