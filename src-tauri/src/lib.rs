use arboard::Clipboard;
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use base64::{Engine as _, engine::general_purpose};
// 👇 新增：引入图像处理模块
use image::{ImageBuffer, Rgba};
use std::io::Cursor;

#[tauri::command]
fn get_clipboard_data() -> Option<(String, String)> {
    let mut clipboard = Clipboard::new().ok()?;
    
    // 1. 优先获取文本
    if let Ok(text) = clipboard.get_text() {
        if !text.trim().is_empty() {
            return Some(("text".to_string(), text));
        }
    }

    // 2. 尝试获取图片 (⭐修复版：将裸像素编码为标准 PNG)
    if let Ok(image_data) = clipboard.get_image() {
        // 读取原生的 RGBA 像素数据
        if let Some(img_buffer) = ImageBuffer::<Rgba<u8>, _>::from_raw(
            image_data.width as u32,
            image_data.height as u32,
            image_data.bytes.into_owned()
        ) {
            // 在内存中把它渲染成一张真正的 PNG 图片
            let mut png_bytes: Vec<u8> = Vec::new();
            let mut cursor = Cursor::new(&mut png_bytes);
            if img_buffer.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
                // 把标准的 PNG 字节流转成 Base64 发给前端
                let base64_str = general_purpose::STANDARD.encode(&png_bytes);
                return Some(("image".to_string(), format!("{}:{}|{}", image_data.width, image_data.height, base64_str)));
            }
        }
    }
    None
}

#[tauri::command]
fn set_clipboard_text(text: String) {
    if let Ok(mut clipboard) = Clipboard::new() {
        let _ = clipboard.set_text(text);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![get_clipboard_data, set_clipboard_text])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let alt_v = Shortcut::new(Some(tauri_plugin_global_shortcut::Modifiers::ALT), tauri_plugin_global_shortcut::Code::KeyV);
            let _ = app.global_shortcut().on_shortcut(alt_v, move |_app, _shortcut, event| {
                if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    let window = app_handle.get_webview_window("main").unwrap();
                    if window.is_visible().unwrap_or(false) { let _ = window.hide(); } else { 
                        let _ = window.show(); 
                        let _ = window.set_focus(); 
                    }
                }
            });

            let quit_i = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let window = tray.app_handle().get_webview_window("main").unwrap();
                        let _ = window.show(); let _ = window.set_focus();
                    }
                })
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => app.exit(0),
                        "show" => { let window = app.get_webview_window("main").unwrap(); let _ = window.show(); },
                        _ => {}
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event { api.prevent_close(); let _ = window.hide(); }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}