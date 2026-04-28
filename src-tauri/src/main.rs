#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::STANDARD, Engine as _};
use clipboard_rs::common::RustImage;
use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat, RustImageData};
use std::path::Path;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

const MAX_IMAGE_FILE_BYTES: u64 = 20 * 1024 * 1024;
const MAX_IMAGE_DATA_BYTES: usize = 20 * 1024 * 1024;
const MAX_IMAGE_DIMENSION: u32 = 8192;
const MAX_IMAGE_PIXELS: u64 = 20_000_000;

fn validate_image_dimensions(width: u32, height: u32) -> Result<(), String> {
    if width == 0 || height == 0 {
        return Err("image dimensions are invalid".to_string());
    }
    if width > MAX_IMAGE_DIMENSION || height > MAX_IMAGE_DIMENSION {
        return Err(format!(
            "image dimensions exceed limit (max {}x{})",
            MAX_IMAGE_DIMENSION, MAX_IMAGE_DIMENSION
        ));
    }
    let pixel_count = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or_else(|| "image dimensions overflow".to_string())?;
    if pixel_count > MAX_IMAGE_PIXELS {
        return Err(format!(
            "image pixel count exceeds limit (max {} pixels)",
            MAX_IMAGE_PIXELS
        ));
    }
    Ok(())
}

fn validate_image_file(path: &Path) -> Result<(), String> {
    let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;
    if metadata.len() > MAX_IMAGE_FILE_BYTES {
        return Err(format!(
            "image file exceeds size limit (max {} bytes)",
            MAX_IMAGE_FILE_BYTES
        ));
    }

    let reader = image::ImageReader::open(path).map_err(|e| e.to_string())?;
    let reader = reader.with_guessed_format().map_err(|e| e.to_string())?;
    let (width, height) = reader.into_dimensions().map_err(|e| e.to_string())?;
    validate_image_dimensions(width, height)
}

fn estimate_base64_decoded_len(base64: &str) -> Result<usize, String> {
    if base64.is_empty() {
        return Err("image payload is empty".to_string());
    }
    if base64.len() % 4 != 0 {
        return Err("invalid base64 payload length".to_string());
    }

    let padding = base64
        .as_bytes()
        .iter()
        .rev()
        .take_while(|&&b| b == b'=')
        .count();
    let decoded_len = base64
        .len()
        .checked_div(4)
        .and_then(|v| v.checked_mul(3))
        .and_then(|v| v.checked_sub(padding))
        .ok_or_else(|| "base64 payload is too large".to_string())?;

    Ok(decoded_len)
}

fn validate_image_bytes(bytes: &[u8]) -> Result<(), String> {
    if bytes.is_empty() {
        return Err("image payload is empty".to_string());
    }
    if bytes.len() > MAX_IMAGE_DATA_BYTES {
        return Err(format!(
            "image payload exceeds size limit (max {} bytes)",
            MAX_IMAGE_DATA_BYTES
        ));
    }

    let reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| e.to_string())?;
    let (width, height) = reader.into_dimensions().map_err(|e| e.to_string())?;
    validate_image_dimensions(width, height)
}

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
                    if validate_image_file(path).is_ok() {
                        if let Ok(img) = image::open(path) {
                            let mut buffer = std::io::Cursor::new(Vec::new());
                            if img.write_to(&mut buffer, image::ImageFormat::Png).is_ok() {
                                let png_bytes = buffer.into_inner();
                                if png_bytes.len() <= MAX_IMAGE_DATA_BYTES {
                                    let base64_str = STANDARD.encode(png_bytes);
                                    return Ok(Some((
                                        "image".to_string(),
                                        format!("image|{}", base64_str),
                                    )));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if ctx.has(ContentFormat::Image) {
        if let Ok(image) = ctx.get_image() {
            let (width, height) = image.get_size();
            if validate_image_dimensions(width, height).is_ok() {
                if let Ok(png_data) = image.to_png() {
                    let png_bytes = png_data.get_bytes();
                    if png_bytes.len() <= MAX_IMAGE_DATA_BYTES {
                        let base64_str = STANDARD.encode(png_bytes);
                        return Ok(Some(("image".to_string(), format!("image|{}", base64_str))));
                    }
                }
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
    let estimated_len = estimate_base64_decoded_len(&base64)?;
    if estimated_len > MAX_IMAGE_DATA_BYTES {
        return Err(format!(
            "image payload exceeds size limit (max {} bytes)",
            MAX_IMAGE_DATA_BYTES
        ));
    }

    let ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    let image_bytes = STANDARD.decode(base64).map_err(|e| e.to_string())?;
    validate_image_bytes(&image_bytes)?;
    let img_data = RustImageData::from_bytes(&image_bytes).map_err(|e| e.to_string())?;
    ctx.set_image(img_data).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|window, event| {
            if window.label() == "main" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
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
                                        let is_minimized = window.is_minimized().unwrap_or(false);

                                        // 只要窗口可见就允许隐藏，避免开机自启动时因未聚焦导致看起来无响应
                                        if is_visible && !is_minimized {
                                            let _ = window.hide();
                                        } else {
                                            // 从隐藏或最小化状态恢复并抢焦点
                                            let _ = window.unminimize();
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    })
                    .build(),
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
