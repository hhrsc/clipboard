#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat, RustImageData};
use clipboard_rs::common::RustImage;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::path::Path;
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

#[tauri::command]
fn set_clipboard_text(text: String) -> Result<(), String> {
    let ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    ctx.set_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_clipboard_data() -> Result<Option<(String, String)>, String> {
    let ctx = ClipboardContext::new().map_err(|e| e.to_string())?;

    if ctx.has(ContentFormat::Files) {
        if let Ok(files) = ctx.get_files() {
            if let Some(file_path) = files.first() {
                let path = Path::new(file_path);
                if path.is_file() {
                    if let Ok(img) = image::open(path) {
                        let mut buffer = std::io::Cursor::new(Vec::new());
                        if img.write_to(&mut buffer, image::ImageFormat::Png).is_ok() {
                            let base64_str = STANDARD.encode(buffer.into_inner());
                            return Ok(Some(("image".to_string(), format!("image|{}", base64_str))));
                        }
                    }
                }
            }
        }
    }

    if ctx.has(ContentFormat::Image) {
        if let Ok(image) = ctx.get_image() {
            if let Ok(png_data) = image.to_png() {
                let base64_str = STANDARD.encode(png_data.get_bytes());
                return Ok(Some(("image".to_string(), format!("image|{}", base64_str))));
            }
        }
    }

    if ctx.has(ContentFormat::Text) {
        if let Ok(text) = ctx.get_text() {
            if !text.is_empty() {
                return Ok(Some(("text".to_string(), text)));
            }
        }
    }

    Ok(None)
}

#[tauri::command]
fn set_clipboard_image(base64: String) -> Result<(), String> {
    let ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    let image_bytes = STANDARD.decode(base64).map_err(|e| e.to_string())?;
    let img_data = RustImageData::from_bytes(&image_bytes).map_err(|e| e.to_string())?;
    ctx.set_image(img_data).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // ==========================================
            // 1. 初始化并开启【开机自启动】
            // ==========================================
            app.handle().plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec![]),
            ))?;
            // 强行把当前软件写入系统的开机启动项中
            let _ = app.autolaunch().enable();

            // ==========================================
            // 2. 注册【全局快捷键 Alt + V】
            // ==========================================
            let alt_v_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyC);
            let alt_v_clone = alt_v_shortcut.clone();

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        if shortcut == &alt_v_clone {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    if let Some(window) = app.get_webview_window("main") {
                                        let is_visible = window.is_visible().unwrap_or(false);
                                        let is_focused = window.is_focused().unwrap_or(false);
                                        
                                        // 如果窗口看得见并且正在用，按快捷键就隐藏它
                                        if is_visible && is_focused {
                                            let _ = window.hide();
                                        } else {
                                            // 否则就把它弹出来并把焦点拉过来
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    })
                    .build()
            )?;
            // 真正把快捷键挂载到系统上
            app.global_shortcut().register(alt_v_shortcut)?;

            // ==========================================
            // 3. 配置【右下角系统托盘】
            // ==========================================
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // 左键点击托盘图标也弹窗
                    if let TrayIconEvent::Click { .. } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                });

            if let Some(icon) = app.default_window_icon().cloned() {
                tray_builder = tray_builder.icon(icon);
            }

            tray_builder.build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_clipboard_text,
            get_clipboard_data,
            set_clipboard_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}