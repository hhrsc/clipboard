#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod local_vault_unlock;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use argon2::{Algorithm, Argon2, Params, Version};
use clipboard_rs::common::RustImage;
use clipboard_rs::{Clipboard, ClipboardContent, ClipboardContext, ContentFormat, RustImageData};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use zeroize::{Zeroize, Zeroizing};

const MAX_IMAGE_FILE_BYTES: u64 = 20 * 1024 * 1024;
const MAX_IMAGE_DATA_BYTES: usize = 20 * 1024 * 1024;
const MAX_IMAGE_DIMENSION: u32 = 8192;
const MAX_IMAGE_PIXELS: u64 = 20_000_000;
const IMAGE_HISTORY_DIR_NAME: &str = "history-images";
const MAX_HISTORY_IMAGE_FILES: usize = 10;
const MAX_HISTORY_IMAGE_TOTAL_BYTES: u64 = 80 * 1024 * 1024;
const VAULT_FILE_NAME: &str = "password-vault.json";
const VAULT_LOCAL_UNLOCK_FILE: &str = "password-vault-unlock.bin";
const VAULT_VERSION: u32 = 2;
const MAX_HTML_BYTES: usize = 1024 * 1024;
const RESET_MARKER: &str = "reset-in-progress.json";
const VAULT_SALT_BYTES: usize = 16;
const VAULT_NONCE_BYTES: usize = 12;
const VAULT_KEY_BYTES: usize = 32;
const VAULT_ARGON_MEMORY_KIB: u32 = 19_456;
const VAULT_ARGON_ITERATIONS: u32 = 2;
const VAULT_ARGON_PARALLELISM: u32 = 1;
const MAX_VAULT_PASSWORDS: usize = 1_000;
const MAX_VAULT_BACKUP_BYTES: u64 = 8 * 1024 * 1024;
const SHORTCUT_FILE_NAME: &str = "shortcut-preference.json";
const DEFAULT_SHORTCUT: &str = "Alt+C";
const CLIPBOARD_STORE_FILE_NAME: &str = "clipboard-store.json";
const CLIPBOARD_STORE_VERSION: u32 = 1;
const ALL_CATEGORY_ID: &str = "all";
const MAX_CLIPBOARD_ITEMS: usize = 1_000;
const MAX_CLIPBOARD_CATEGORIES: usize = 100;

struct VaultRuntime {
    key: Mutex<Option<Zeroizing<[u8; VAULT_KEY_BYTES]>>>,
}

#[derive(Default)]
struct MutationRuntime(Mutex<()>);

impl Default for VaultRuntime {
    fn default() -> Self {
        Self { key: Mutex::new(None) }
    }
}

struct ShortcutRuntime {
    shortcut: Mutex<String>,
}

impl Default for ShortcutRuntime {
    fn default() -> Self {
        Self { shortcut: Mutex::new(DEFAULT_SHORTCUT.to_string()) }
    }
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultPassword {
    id: u64,
    title: String,
    username: String,
    password: String,
    #[serde(default)]
    collection_id: Option<String>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
struct VaultCollection {
    id: String,
    name: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultPayload {
    version: u32,
    passwords: Vec<VaultPassword>,
    #[serde(default)]
    collections: Vec<VaultCollection>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultKdf {
    algorithm: String,
    memory_kib: u32,
    iterations: u32,
    parallelism: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultFile {
    version: u32,
    kdf: VaultKdf,
    salt: String,
    nonce: String,
    ciphertext: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultStatus {
    exists: bool,
    unlocked: bool,
    require_password: bool,
    auto_unlock_available: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultBackupStatus {
    passwords: usize,
}

#[derive(Deserialize, Serialize)]
struct ShortcutPreference {
    version: u32,
    shortcut: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ShortcutStatus {
    shortcut: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardCategory {
    id: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardRecord {
    #[serde(rename = "type")]
    kind: String,
    content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    id: u64,
    timestamp: String,
    category_id: String,
    #[serde(default)]
    restored: bool,
    #[serde(default)]
    is_pinned: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyClipboardRecord {
    #[serde(rename = "type")]
    kind: String,
    content: String,
    id: u64,
    timestamp: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    restored: bool,
    #[serde(default)]
    is_pinned: bool,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardPreferences {
    capture_enabled: bool,
    retention_hours: u16,
    official_website: String,
}

impl Default for ClipboardPreferences {
    fn default() -> Self {
        Self {
            capture_enabled: true,
            retention_hours: 24,
            official_website: "https://www.gov.cn/".to_string(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardStore {
    version: u32,
    categories: Vec<ClipboardCategory>,
    records: Vec<ClipboardRecord>,
    preferences: ClipboardPreferences,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardStoreStatus {
    exists: bool,
    version: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClipboardSnapshot {
    #[serde(rename = "type")]
    kind: String,
    content: String,
    html: Option<String>,
    warning: Option<String>,
}

fn reset_marker_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(app.path().app_data_dir().map_err(|e| e.to_string())?.join(RESET_MARKER))
}

fn mutate<T>(app: &tauri::AppHandle, operation: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    let state = app.state::<MutationRuntime>();
    let _guard = state.0.lock().map_err(|_| "application storage is busy".to_string())?;
    if reset_marker_path(app)?.exists() {
        return Err("An app reset is pending. Complete the reset before making changes.".to_string());
    }
    operation()
}

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

fn history_image_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut dir = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    dir.push(IMAGE_HISTORY_DIR_NAME);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn cleanup_history_image_dir(dir: &Path) -> Result<(), String> {
    let mut files: Vec<(PathBuf, SystemTime, u64)> = Vec::new();

    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
        files.push((path, modified, metadata.len()));
    }

    files.sort_by(|a, b| b.1.cmp(&a.1));

    let mut total_bytes = 0_u64;
    for (idx, (path, _, len)) in files.iter().enumerate() {
        total_bytes = total_bytes.saturating_add(*len);
        if idx >= MAX_HISTORY_IMAGE_FILES || total_bytes > MAX_HISTORY_IMAGE_TOTAL_BYTES {
            let _ = std::fs::remove_file(path);
        }
    }

    Ok(())
}

fn normalize_history_image_path(raw: &str) -> Option<PathBuf> {
    if raw.trim().is_empty() {
        return None;
    }
    let cleaned = raw.strip_prefix("file|").unwrap_or(raw).trim();
    if cleaned.is_empty() {
        None
    } else {
        Some(PathBuf::from(cleaned))
    }
}

fn path_is_within_dir(path: &Path, dir: &Path) -> bool {
    match (path.canonicalize(), dir.canonicalize()) {
        (Ok(p), Ok(d)) => p.starts_with(d),
        _ => false,
    }
}

fn vault_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join(VAULT_FILE_NAME))
}

fn vault_backup_path(path: &Path) -> PathBuf {
    path.with_extension("bak")
}

fn vault_local_unlock_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    Ok(vault_path(app)?.with_file_name(VAULT_LOCAL_UNLOCK_FILE))
}

fn vault_local_unlock_context(app: &tauri::AppHandle, vault: &VaultFile) -> Vec<u8> {
    format!("{}:vault-unlock:{}", app.config().identifier, vault.salt).into_bytes()
}

fn shortcut_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join(SHORTCUT_FILE_NAME))
}

fn normalize_shortcut(raw: &str) -> Result<String, String> {
    let parts = raw
        .split('+')
        .map(|part| part.trim().to_ascii_uppercase())
        .collect::<Vec<_>>();
    if !(2..=4).contains(&parts.len()) {
        return Err("shortcut must contain a modifier and one letter or number".to_string());
    }

    let key = parts.last().cloned().unwrap_or_default();
    if key.len() != 1 || !key.chars().all(|character| character.is_ascii_alphanumeric()) {
        return Err("shortcut key must be a single letter or number".to_string());
    }

    let mut modifiers = Vec::new();
    for modifier in &parts[..parts.len() - 1] {
        let normalized = match modifier.as_str() {
            "ALT" => "Alt",
            "CTRL" | "CONTROL" => "Ctrl",
            "SHIFT" => "Shift",
            _ => return Err("shortcut uses an unsupported modifier".to_string()),
        };
        if modifiers.contains(&normalized) {
            return Err("shortcut repeats a modifier".to_string());
        }
        modifiers.push(normalized);
    }
    if !modifiers.iter().any(|modifier| *modifier == "Alt" || *modifier == "Ctrl") {
        return Err("shortcut must include Alt or Ctrl".to_string());
    }

    modifiers.sort_by_key(|modifier| match *modifier { "Ctrl" => 0, "Alt" => 1, _ => 2 });
    let mut normalized = modifiers.join("+");
    normalized.push('+');
    normalized.push_str(&key);
    normalized
        .parse::<Shortcut>()
        .map_err(|_| "shortcut is not available on this system".to_string())?;
    Ok(normalized)
}

fn load_shortcut_preference(app: &tauri::AppHandle) -> String {
    let Ok(path) = shortcut_path(app) else {
        return DEFAULT_SHORTCUT.to_string();
    };
    let backup = path.with_extension("bak");
    let candidate = if path.exists() { path } else { backup };
    let Ok(bytes) = std::fs::read(candidate) else {
        return DEFAULT_SHORTCUT.to_string();
    };
    let Ok(preference) = serde_json::from_slice::<ShortcutPreference>(&bytes) else {
        return DEFAULT_SHORTCUT.to_string();
    };
    normalize_shortcut(&preference.shortcut).unwrap_or_else(|_| DEFAULT_SHORTCUT.to_string())
}

fn write_shortcut_preference(app: &tauri::AppHandle, shortcut: &str) -> Result<(), String> {
    let path = shortcut_path(app)?;
    let preference = ShortcutPreference { version: 1, shortcut: shortcut.to_string() };
    let bytes = serde_json::to_vec(&preference).map_err(|_| "could not save shortcut preference".to_string())?;
    let temp = path.with_extension("tmp");
    let backup = path.with_extension("bak");
    let mut file = std::fs::File::create(&temp).map_err(|_| "could not prepare shortcut preference".to_string())?;
    file.write_all(&bytes).map_err(|_| "could not save shortcut preference".to_string())?;
    file.sync_all().map_err(|_| "could not finalize shortcut preference".to_string())?;
    if path.exists() {
        std::fs::copy(&path, &backup).map_err(|_| "could not safeguard shortcut preference".to_string())?;
        std::fs::remove_file(&path).map_err(|_| "could not replace shortcut preference".to_string())?;
    }
    if std::fs::rename(&temp, &path).is_err() {
        if backup.exists() {
            let _ = std::fs::rename(&backup, &path);
        }
        return Err("could not replace shortcut preference".to_string());
    }
    let _ = std::fs::remove_file(backup);
    Ok(())
}

fn default_clipboard_store() -> ClipboardStore {
    ClipboardStore {
        version: CLIPBOARD_STORE_VERSION,
        categories: vec![ClipboardCategory { id: ALL_CATEGORY_ID.to_string(), name: "全部".to_string() }],
        records: Vec::new(),
        preferences: ClipboardPreferences::default(),
    }
}

fn validate_clipboard_store(store: &ClipboardStore) -> Result<(), String> {
    if store.version != CLIPBOARD_STORE_VERSION
        || store.categories.is_empty()
        || store.categories.len() > MAX_CLIPBOARD_CATEGORIES
        || store.records.len() > MAX_CLIPBOARD_ITEMS
        || !(1..=8_760).contains(&store.preferences.retention_hours)
        || !store.preferences.official_website.starts_with("https://")
    {
        return Err("clipboard store is invalid".to_string());
    }

    let mut category_ids = HashSet::new();
    for category in &store.categories {
        if category.id.is_empty()
            || category.id.len() > 64
            || category.name.trim().is_empty()
            || category.name.len() > 120
            || !category_ids.insert(category.id.clone())
        {
            return Err("clipboard store categories are invalid".to_string());
        }
    }
    if !category_ids.contains(ALL_CATEGORY_ID) {
        return Err("clipboard store is missing the default category".to_string());
    }

    let mut record_ids = HashSet::new();
    for record in &store.records {
        let content_limit = if record.kind == "text" { 200_000 } else { 4_096 };
        if record.id == 0
            || (record.kind != "text" && record.kind != "image")
            || record.content.is_empty()
            || record.content.len() > content_limit
            || record.html.as_ref().is_some_and(|html| record.kind != "text" || html.len() > MAX_HTML_BYTES)
            || record.timestamp.len() > 200
            || !category_ids.contains(&record.category_id)
            || !record_ids.insert(record.id)
        {
            return Err("clipboard store records are invalid".to_string());
        }
    }
    Ok(())
}

fn clipboard_store_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join(CLIPBOARD_STORE_FILE_NAME))
}

fn read_clipboard_store(app: &tauri::AppHandle) -> Result<ClipboardStore, String> {
    let path = clipboard_store_path(app)?;
    let backup = path.with_extension("bak");
    let candidate = if path.exists() { path } else { backup };
    let bytes = std::fs::read(candidate).map_err(|_| "clipboard store is unavailable".to_string())?;
    let store: ClipboardStore = serde_json::from_slice(&bytes).map_err(|_| "clipboard store is invalid".to_string())?;
    validate_clipboard_store(&store)?;
    Ok(store)
}

fn write_clipboard_store(app: &tauri::AppHandle, store: &ClipboardStore) -> Result<(), String> {
    validate_clipboard_store(store)?;
    let path = clipboard_store_path(app)?;
    let backup = path.with_extension("bak");
    let temp = path.with_extension("tmp");
    let bytes = serde_json::to_vec(store).map_err(|_| "could not serialize clipboard store".to_string())?;
    let mut file = std::fs::File::create(&temp).map_err(|_| "could not prepare clipboard store".to_string())?;
    file.write_all(&bytes).map_err(|_| "could not write clipboard store".to_string())?;
    file.sync_all().map_err(|_| "could not finalize clipboard store".to_string())?;
    if path.exists() {
        std::fs::copy(&path, &backup).map_err(|_| "could not safeguard clipboard store".to_string())?;
        std::fs::remove_file(&path).map_err(|_| "could not replace clipboard store".to_string())?;
    }
    if std::fs::rename(&temp, &path).is_err() {
        if backup.exists() {
            let _ = std::fs::rename(&backup, &path);
        }
        return Err("could not replace clipboard store".to_string());
    }
    let _ = std::fs::remove_file(backup);
    Ok(())
}

fn migrate_legacy_clipboard_store(
    legacy_records: Vec<LegacyClipboardRecord>,
    legacy_categories: Vec<String>,
    preferences: Option<ClipboardPreferences>,
) -> Result<ClipboardStore, String> {
    if legacy_records.len() > MAX_CLIPBOARD_ITEMS || legacy_categories.len() > MAX_CLIPBOARD_CATEGORIES {
        return Err("legacy clipboard data exceeds supported limits".to_string());
    }
    let mut store = default_clipboard_store();
    store.preferences = preferences.unwrap_or_default();
    let mut name_to_id = std::collections::HashMap::from([("全部".to_string(), ALL_CATEGORY_ID.to_string())]);

    for name in legacy_categories {
        let name = name.trim().to_string();
        if name.is_empty() || name == "全部" || name_to_id.contains_key(&name) {
            continue;
        }
        if store.categories.len() >= MAX_CLIPBOARD_CATEGORIES {
            return Err("legacy clipboard data exceeds supported category limit".to_string());
        }
        let id = format!("category-{}", store.categories.len());
        name_to_id.insert(name.clone(), id.clone());
        store.categories.push(ClipboardCategory { id, name });
    }

    for legacy in legacy_records {
        let category_name = legacy.category.trim();
        let category_id = name_to_id
            .get(category_name)
            .cloned()
            .unwrap_or_else(|| ALL_CATEGORY_ID.to_string());
        store.records.push(ClipboardRecord {
            kind: legacy.kind,
            content: legacy.content,
            html: None,
            id: legacy.id,
            timestamp: legacy.timestamp,
            category_id,
            restored: legacy.restored,
            is_pinned: legacy.is_pinned,
        });
    }
    validate_clipboard_store(&store)?;
    Ok(store)
}

fn vault_exists(app: &tauri::AppHandle) -> Result<bool, String> {
    let path = vault_path(app)?;
    Ok(path.exists() || vault_backup_path(&path).exists())
}

fn read_vault_file(app: &tauri::AppHandle) -> Result<VaultFile, String> {
    let path = vault_path(app)?;
    let backup = vault_backup_path(&path);
    let candidate = if path.exists() { path } else { backup };
    let bytes = std::fs::read(candidate).map_err(|_| "password vault is unavailable".to_string())?;
    serde_json::from_slice(&bytes).map_err(|_| "password vault is invalid".to_string())
}

fn write_vault_file(app: &tauri::AppHandle, vault: &VaultFile) -> Result<(), String> {
    let path = vault_path(app)?;
    let backup = vault_backup_path(&path);
    let bytes = serde_json::to_vec(vault).map_err(|_| "could not serialize password vault".to_string())?;
    let temp = path.with_extension("tmp");

    let mut file = std::fs::File::create(&temp).map_err(|_| "could not prepare password vault".to_string())?;
    file.write_all(&bytes).map_err(|_| "could not write password vault".to_string())?;
    file.sync_all().map_err(|_| "could not finalize password vault".to_string())?;

    if path.exists() {
        std::fs::copy(&path, &backup).map_err(|_| "could not safeguard password vault".to_string())?;
        std::fs::remove_file(&path).map_err(|_| "could not replace password vault".to_string())?;
    }

    if let Err(_) = std::fs::rename(&temp, &path) {
        if backup.exists() {
            let _ = std::fs::rename(&backup, &path);
        }
        return Err("could not replace password vault".to_string());
    }

    let _ = std::fs::remove_file(backup);
    Ok(())
}

fn read_backup_vault(path: &str) -> Result<VaultFile, String> {
    let path = PathBuf::from(path);
    if !path.is_absolute() {
        return Err("backup path must be absolute".to_string());
    }
    let metadata = std::fs::metadata(&path).map_err(|_| "backup file is unavailable".to_string())?;
    if !metadata.is_file() || metadata.len() > MAX_VAULT_BACKUP_BYTES {
        return Err("backup file is invalid or too large".to_string());
    }
    let bytes = std::fs::read(path).map_err(|_| "backup file is unavailable".to_string())?;
    serde_json::from_slice(&bytes).map_err(|_| "backup file is invalid".to_string())
}

fn export_backup_file(path: &str, vault: &VaultFile) -> Result<(), String> {
    let path = PathBuf::from(path);
    if !path.is_absolute() || path.exists() {
        return Err("backup destination must be a new absolute path".to_string());
    }
    let parent = path.parent().ok_or_else(|| "backup destination is invalid".to_string())?;
    if !parent.is_dir() {
        return Err("backup destination folder is unavailable".to_string());
    }
    let bytes = serde_json::to_vec(vault).map_err(|_| "could not prepare encrypted backup".to_string())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|_| "could not create encrypted backup".to_string())?;
    if file.write_all(&bytes).is_err() || file.sync_all().is_err() {
        let _ = std::fs::remove_file(path);
        return Err("could not finalize encrypted backup".to_string());
    }
    Ok(())
}

fn validate_master_password(master_password: &str) -> Result<(), String> {
    if !(8..=16).contains(&master_password.chars().count()) {
        return Err("master password must contain 8-16 characters".to_string());
    }
    Ok(())
}

fn validate_passwords(passwords: &[VaultPassword]) -> Result<(), String> {
    if passwords.len() > MAX_VAULT_PASSWORDS {
        return Err("password vault exceeds the supported entry limit".to_string());
    }
    for item in passwords {
        if item.title.trim().is_empty()
            || item.title.len() > 500
            || item.username.len() > 500
            || item.password.is_empty()
            || item.password.len() > 2_000
        {
            return Err("password vault contains an invalid entry".to_string());
        }
    }
    Ok(())
}

fn validate_vault_payload(payload: &VaultPayload) -> Result<(), String> {
    validate_passwords(&payload.passwords)?;
    let mut ids = HashSet::new();
    let mut names = HashSet::new();
    if payload.collections.len() > 100 {
        return Err("Too many password collections".to_string());
    }
    for collection in &payload.collections {
        if collection.id.is_empty() || collection.id.len() > 64
            || collection.name.trim().is_empty() || collection.name.len() > 120
            || collection.name != collection.name.trim()
            || !ids.insert(collection.id.clone()) || !names.insert(collection.name.to_lowercase()) {
            return Err("Invalid or duplicate password collection".to_string());
        }
    }
    let mut password_ids = HashSet::new();
    for password in &payload.passwords {
        if !password_ids.insert(password.id) || password.collection_id.as_ref().is_some_and(|id| !ids.contains(id)) {
            return Err("Invalid password collection reference or duplicate ID".to_string());
        }
    }
    Ok(())
}

fn derive_vault_key(master_password: &str, salt: &[u8], kdf: &VaultKdf) -> Result<[u8; VAULT_KEY_BYTES], String> {
    // 长度策略只限制新密码，已有密码库仍按原密码派生密钥。
    if master_password.is_empty() {
        return Err("master password is required".to_string());
    }
    if kdf.algorithm != "argon2id"
        || salt.len() != VAULT_SALT_BYTES
        || kdf.memory_kib < VAULT_ARGON_MEMORY_KIB
        || kdf.memory_kib > 262_144
        || kdf.iterations < VAULT_ARGON_ITERATIONS
        || kdf.iterations > 10
        || kdf.parallelism == 0
        || kdf.parallelism > 4
    {
        return Err("password vault uses unsupported security parameters".to_string());
    }

    let params = Params::new(
        kdf.memory_kib,
        kdf.iterations,
        kdf.parallelism,
        Some(VAULT_KEY_BYTES),
    )
    .map_err(|_| "password vault security parameters are invalid".to_string())?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0_u8; VAULT_KEY_BYTES];
    argon2
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .map_err(|_| "could not derive password vault key".to_string())?;
    Ok(key)
}

fn encrypt_payload(payload: &VaultPayload, key: &[u8; VAULT_KEY_BYTES], kdf: VaultKdf, salt: &[u8]) -> Result<VaultFile, String> {
    let plaintext = serde_json::to_vec(payload).map_err(|_| "could not serialize password vault".to_string())?;
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "could not initialize password vault".to_string())?;
    let mut nonce = [0_u8; VAULT_NONCE_BYTES];
    OsRng.fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| "could not encrypt password vault".to_string())?;

    Ok(VaultFile {
        version: VAULT_VERSION,
        kdf,
        salt: STANDARD.encode(salt),
        nonce: STANDARD.encode(nonce),
        ciphertext: STANDARD.encode(ciphertext),
    })
}

fn decrypt_payload(vault: &VaultFile, key: &[u8; VAULT_KEY_BYTES]) -> Result<VaultPayload, String> {
    if !(1..=VAULT_VERSION).contains(&vault.version) {
        return Err("password vault version is unsupported".to_string());
    }
    let nonce = STANDARD.decode(&vault.nonce).map_err(|_| "password vault is invalid".to_string())?;
    if nonce.len() != VAULT_NONCE_BYTES {
        return Err("password vault is invalid".to_string());
    }
    let ciphertext = STANDARD.decode(&vault.ciphertext).map_err(|_| "password vault is invalid".to_string())?;
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "could not initialize password vault".to_string())?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| "master password is incorrect or password vault is damaged".to_string())?;
    let payload: VaultPayload = serde_json::from_slice(&plaintext).map_err(|_| "password vault is invalid".to_string())?;
    if !(1..=VAULT_VERSION).contains(&payload.version) {
        return Err("password vault version is unsupported".to_string());
    }
    validate_vault_payload(&payload)?;
    Ok(payload)
}

fn create_vault_file(master_password: &str, passwords: Vec<VaultPassword>) -> Result<(VaultFile, [u8; VAULT_KEY_BYTES]), String> {
    create_vault_snapshot(master_password, VaultPayload { version: VAULT_VERSION, passwords, collections: Vec::new() })
}

fn create_vault_snapshot(master_password: &str, mut payload: VaultPayload) -> Result<(VaultFile, [u8; VAULT_KEY_BYTES]), String> {
    validate_master_password(master_password)?;
    validate_vault_payload(&payload)?;
    payload.version = VAULT_VERSION;
    let mut salt = [0_u8; VAULT_SALT_BYTES];
    OsRng.fill_bytes(&mut salt);
    let kdf = VaultKdf {
        algorithm: "argon2id".to_string(),
        memory_kib: VAULT_ARGON_MEMORY_KIB,
        iterations: VAULT_ARGON_ITERATIONS,
        parallelism: VAULT_ARGON_PARALLELISM,
    };
    let key = derive_vault_key(master_password, &salt, &kdf)?;
    let vault = encrypt_payload(&payload, &key, kdf, &salt)?;
    Ok((vault, key))
}

fn rotate_vault_master_password(
    vault: &VaultFile,
    current_master_password: &str,
    next_master_password: &str,
) -> Result<(VaultFile, [u8; VAULT_KEY_BYTES]), String> {
    let salt = STANDARD.decode(&vault.salt).map_err(|_| "password vault is invalid".to_string())?;
    let current_key = derive_vault_key(current_master_password, &salt, &vault.kdf)?;
    let payload = decrypt_payload(vault, &current_key)?;
    create_vault_snapshot(next_master_password, payload)
}

fn active_vault_key(state: &tauri::State<VaultRuntime>) -> Result<Zeroizing<[u8; VAULT_KEY_BYTES]>, String> {
    let guard = state.key.lock().map_err(|_| "password vault is unavailable".to_string())?;
    guard
        .as_ref()
        .map(|key| Zeroizing::new(**key))
        .ok_or_else(|| "password vault is locked".to_string())
}

fn store_vault_key(state: &tauri::State<VaultRuntime>, key: [u8; VAULT_KEY_BYTES]) -> Result<(), String> {
    let mut guard = state.key.lock().map_err(|_| "password vault is unavailable".to_string())?;
    if let Some(existing) = guard.as_mut() {
        existing.zeroize();
    }
    *guard = Some(Zeroizing::new(key));
    Ok(())
}

#[tauri::command]
fn vault_status(app: tauri::AppHandle, state: tauri::State<VaultRuntime>) -> Result<VaultStatus, String> {
    let unlocked = state
        .key
        .lock()
        .map_err(|_| "password vault is unavailable".to_string())?
        .is_some();
    Ok(VaultStatus {
        exists: vault_exists(&app)?, unlocked,
        require_password: !vault_local_unlock_path(&app)?.try_exists().map_err(|e| e.to_string())?,
        auto_unlock_available: cfg!(windows),
    })
}

#[tauri::command]
fn vault_set_require_password(
    app: tauri::AppHandle,
    state: tauri::State<VaultRuntime>,
    require_password: bool,
    master_password: String,
) -> Result<(), String> {
    let master_password = Zeroizing::new(master_password);
    mutate(&app, || {
        let vault = read_vault_file(&app)?;
        let mut active = state.key.lock().map_err(|_| "password vault is unavailable".to_string())?;
        configure_vault_access(&vault_local_unlock_path(&app)?, &vault, &master_password, require_password, &vault_local_unlock_context(&app, &vault))?;
        if require_password { *active = None; }
        Ok(())
    })
}

fn configure_vault_access(path: &Path, vault: &VaultFile, master_password: &str, require_password: bool, context: &[u8]) -> Result<(), String> {
    let salt = STANDARD.decode(&vault.salt).map_err(|_| "password vault is invalid".to_string())?;
    let key = Zeroizing::new(derive_vault_key(master_password, &salt, &vault.kdf)?);
    decrypt_payload(vault, &key)?;
    if require_password { local_vault_unlock::remove(path) }
    else { local_vault_unlock::save(path, &key, context) }
}

#[tauri::command]
fn vault_auto_unlock(app: tauri::AppHandle, state: tauri::State<VaultRuntime>) -> Result<VaultPayload, String> {
    mutate(&app, || {
        let vault = read_vault_file(&app)?;
        let key = local_vault_unlock::read(&vault_local_unlock_path(&app)?, &vault_local_unlock_context(&app, &vault))?;
        let payload = decrypt_payload(&vault, &key)?;
        store_vault_key(&state, *key)?;
        Ok(payload)
    })
}

#[tauri::command]
fn vault_setup(
    app: tauri::AppHandle,
    state: tauri::State<VaultRuntime>,
    master_password: String,
    legacy_passwords: Vec<VaultPassword>,
) -> Result<VaultPayload, String> {
    mutate(&app, || {
    if vault_exists(&app)? {
        return Err("password vault already exists".to_string());
    }
    let (vault, key) = create_vault_file(&master_password, legacy_passwords.clone())?;
    local_vault_unlock::remove(&vault_local_unlock_path(&app)?)?;
    write_vault_file(&app, &vault)?;
    let verified = decrypt_payload(&read_vault_file(&app)?, &key)?;
    if verified.passwords != legacy_passwords {
        return Err("password vault verification failed".to_string());
    }
    store_vault_key(&state, key)?;
    Ok(verified)
    })
}

#[tauri::command]
fn vault_unlock(
    app: tauri::AppHandle,
    state: tauri::State<VaultRuntime>,
    master_password: String,
) -> Result<VaultPayload, String> {
    mutate(&app, || {
    let vault = read_vault_file(&app)?;
    let salt = STANDARD.decode(&vault.salt).map_err(|_| "password vault is invalid".to_string())?;
    let key = derive_vault_key(&master_password, &salt, &vault.kdf)?;
    let payload = decrypt_payload(&vault, &key)?;
    store_vault_key(&state, key)?;
    Ok(payload)
    })
}

#[tauri::command]
fn vault_replace_passwords(
    app: tauri::AppHandle,
    state: tauri::State<VaultRuntime>,
    passwords: Vec<VaultPassword>,
    collections: Option<Vec<VaultCollection>>,
) -> Result<(), String> {
    mutate(&app, || {
    let key = active_vault_key(&state)?;
    let current = read_vault_file(&app)?;
    let existing = decrypt_payload(&current, &key)?;
    if collections.is_none() && !existing.collections.is_empty() {
        return Err("A complete password vault snapshot is required".to_string());
    }
    let salt = STANDARD.decode(&current.salt).map_err(|_| "password vault is invalid".to_string())?;
    let payload = VaultPayload { version: VAULT_VERSION, passwords, collections: collections.unwrap_or_default() };
    validate_vault_payload(&payload)?;
    let encrypted = encrypt_payload(&payload, &key, current.kdf, &salt)?;
    write_vault_file(&app, &encrypted)
    })
}

#[tauri::command]
fn vault_change_master_password(
    app: tauri::AppHandle,
    state: tauri::State<VaultRuntime>,
    current_master_password: String,
    next_master_password: String,
) -> Result<(), String> {
    mutate(&app, || {
    let current = read_vault_file(&app)?;
    let (rotated, next_key) = rotate_vault_master_password(
        &current,
        &current_master_password,
        &next_master_password,
    )?;
    local_vault_unlock::remove(&vault_local_unlock_path(&app)?)?;
    write_vault_file(&app, &rotated)?;
    let verified = read_vault_file(&app)?;
    decrypt_payload(&verified, &next_key)?;
    store_vault_key(&state, next_key)
    })
}

#[tauri::command]
fn vault_export_encrypted_backup(
    app: tauri::AppHandle,
    state: tauri::State<VaultRuntime>,
    destination_path: String,
) -> Result<VaultBackupStatus, String> {
    mutate(&app, || {
    let key = active_vault_key(&state)?;
    let vault = read_vault_file(&app)?;
    let payload = decrypt_payload(&vault, &key)?;
    export_backup_file(&destination_path, &vault)?;
    Ok(VaultBackupStatus { passwords: payload.passwords.len() })
    })
}

#[tauri::command]
fn vault_restore_encrypted_backup(
    app: tauri::AppHandle,
    state: tauri::State<VaultRuntime>,
    source_path: String,
    master_password: String,
    replace_existing: bool,
) -> Result<VaultPayload, String> {
    mutate(&app, || {
    if vault_exists(&app)? && !replace_existing {
        return Err("password vault already exists; explicit replacement is required".to_string());
    }
    let backup = read_backup_vault(&source_path)?;
    let salt = STANDARD.decode(&backup.salt).map_err(|_| "backup file is invalid".to_string())?;
    let key = derive_vault_key(&master_password, &salt, &backup.kdf)?;
    let payload = decrypt_payload(&backup, &key)?;
    let upgraded = VaultPayload { version: VAULT_VERSION, ..payload.clone() };
    let upgraded_file = encrypt_payload(&upgraded, &key, backup.kdf, &salt)?;
    local_vault_unlock::remove(&vault_local_unlock_path(&app)?)?;
    write_vault_file(&app, &upgraded_file)?;
    let verified = decrypt_payload(&read_vault_file(&app)?, &key)?;
    if verified != upgraded {
        return Err("encrypted backup verification failed".to_string());
    }
    store_vault_key(&state, key)?;
    Ok(verified)
    })
}

#[tauri::command]
fn vault_lock(state: tauri::State<VaultRuntime>) -> Result<(), String> {
    let mut guard = state.key.lock().map_err(|_| "password vault is unavailable".to_string())?;
    if let Some(key) = guard.as_mut() {
        key.zeroize();
    }
    *guard = None;
    Ok(())
}

#[tauri::command]
fn vault_delete(app: tauri::AppHandle, state: tauri::State<VaultRuntime>) -> Result<(), String> {
    mutate(&app, || {
    local_vault_unlock::remove(&vault_local_unlock_path(&app)?)?;
    vault_lock(state)?;
    let path = vault_path(&app)?;
    let backup = vault_backup_path(&path);
    for candidate in [path.clone(), backup, path.with_extension("tmp")] {
        if candidate.exists() {
            std::fs::remove_file(candidate).map_err(|_| "could not remove password vault".to_string())?;
        }
    }
    Ok(())
    })
}

#[tauri::command]
fn clipboard_store_status(app: tauri::AppHandle) -> Result<ClipboardStoreStatus, String> {
    let path = clipboard_store_path(&app)?;
    let exists = path.exists() || path.with_extension("bak").exists();
    let version = if exists { Some(read_clipboard_store(&app)?.version) } else { None };
    Ok(ClipboardStoreStatus { exists, version })
}

#[tauri::command]
fn clipboard_store_load(app: tauri::AppHandle) -> Result<ClipboardStore, String> {
    read_clipboard_store(&app)
}

#[tauri::command]
fn clipboard_store_replace(app: tauri::AppHandle, store: ClipboardStore) -> Result<(), String> {
    mutate(&app, || write_clipboard_store(&app, &store))
}

#[tauri::command]
fn clipboard_store_migrate_legacy(
    app: tauri::AppHandle,
    legacy_records: Vec<LegacyClipboardRecord>,
    legacy_categories: Vec<String>,
    preferences: Option<ClipboardPreferences>,
) -> Result<ClipboardStore, String> {
    mutate(&app, || {
    let path = clipboard_store_path(&app)?;
    if path.exists() || path.with_extension("bak").exists() {
        return read_clipboard_store(&app);
    }
    let store = migrate_legacy_clipboard_store(legacy_records, legacy_categories, preferences)?;
    write_clipboard_store(&app, &store)?;
    read_clipboard_store(&app)
    })
}

#[tauri::command]
fn clipboard_store_delete(app: tauri::AppHandle) -> Result<(), String> {
    mutate(&app, || {
    let path = clipboard_store_path(&app)?;
    for candidate in [path.clone(), path.with_extension("bak"), path.with_extension("tmp")] {
        if candidate.exists() {
            std::fs::remove_file(candidate).map_err(|_| "could not remove clipboard store".to_string())?;
        }
    }
    Ok(())
    })
}

#[tauri::command]
fn shortcut_status(state: tauri::State<ShortcutRuntime>) -> Result<ShortcutStatus, String> {
    let shortcut = state
        .shortcut
        .lock()
        .map_err(|_| "shortcut settings are unavailable".to_string())?
        .clone();
    Ok(ShortcutStatus { shortcut })
}

#[tauri::command]
fn update_global_shortcut(
    app: tauri::AppHandle,
    state: tauri::State<ShortcutRuntime>,
    shortcut: String,
) -> Result<ShortcutStatus, String> {
    mutate(&app, || update_shortcut_inner(&app, &state, shortcut))
}

fn update_shortcut_inner(app: &tauri::AppHandle, state: &ShortcutRuntime, shortcut: String) -> Result<ShortcutStatus, String> {
    let next = normalize_shortcut(&shortcut)?;
    let mut active = state
        .shortcut
        .lock()
        .map_err(|_| "shortcut settings are unavailable".to_string())?;
    let previous = active.clone();
    let previous_shortcut = previous
        .parse::<Shortcut>()
        .map_err(|_| "current shortcut preference is invalid".to_string())?;
    let next_shortcut = next
        .parse::<Shortcut>()
        .map_err(|_| "shortcut is not available on this system".to_string())?;
    let was_registered = app.global_shortcut().is_registered(previous_shortcut);
    if previous_shortcut == next_shortcut && was_registered {
        if previous != next { write_shortcut_preference(app, &next)?; }
        *active = next.clone();
        return Ok(ShortcutStatus { shortcut: next });
    }

    app.global_shortcut()
        .register(next_shortcut.clone())
        .map_err(|error| format!("Could not register {next}: {error}"))?;
    if was_registered && app.global_shortcut().unregister(previous_shortcut).is_err() {
        let _ = app.global_shortcut().unregister(next_shortcut);
        return Err("could not replace the current shortcut".to_string());
    }
    if write_shortcut_preference(&app, &next).is_err() {
        if was_registered { let _ = app.global_shortcut().register(previous_shortcut); }
        let _ = app.global_shortcut().unregister(next_shortcut);
        return Err("could not save shortcut preference".to_string());
    }
    *active = next.clone();
    Ok(ShortcutStatus { shortcut: next })
}

fn fit_initial_window(app: &tauri::AppHandle) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window("main") else { return Ok(()) };
    let Some(monitor) = window.current_monitor()?.or(window.primary_monitor()?) else { return Ok(()) };
    let area = monitor.work_area();
    let inner = window.inner_size()?;
    let outer = window.outer_size()?;
    let frame_width = outer.width.saturating_sub(inner.width);
    let frame_height = outer.height.saturating_sub(inner.height);
    // 使用物理像素与工作区，兼容系统缩放并避开任务栏。
    let size = tauri::PhysicalSize::new(
        inner.width.min(area.size.width.saturating_sub(frame_width)).max(1),
        inner.height.min(area.size.height.saturating_sub(frame_height)).max(1),
    );
    if size != inner { window.set_size(size)?; }
    let position = window.outer_position()?;
    let max_x = area.position.x + area.size.width.saturating_sub(size.width + frame_width) as i32;
    let max_y = area.position.y + area.size.height.saturating_sub(size.height + frame_height) as i32;
    let fitted = tauri::PhysicalPosition::new(
        position.x.clamp(area.position.x, max_x),
        position.y.clamp(area.position.y, max_y),
    );
    if position != fitted { window.set_position(fitted)?; }
    Ok(())
}

fn toggle_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let is_visible = window.is_visible().unwrap_or(false);
        let is_minimized = window.is_minimized().unwrap_or(false);
        if is_visible && !is_minimized {
            let _ = window.hide();
        } else {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
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
    read_clipboard_data(&ctx)
}

fn read_clipboard_data(ctx: &ClipboardContext) -> Result<Option<(String, String)>, String> {

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
fn get_clipboard_snapshot() -> Result<Option<ClipboardSnapshot>, String> {
    let ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    let rich_text = if ctx.has(ContentFormat::Html) {
        ctx.get_text().ok().filter(|text| !text.is_empty()).map(|text| ("text".to_string(), text))
    } else { None };
    let data = match rich_text { Some(data) => Some(data), None => read_clipboard_data(&ctx)? };
    let Some((kind, content)) = data else { return Ok(None) };
    let mut snapshot = ClipboardSnapshot { kind, content, html: None, warning: None };
    if snapshot.kind == "text" && ctx.has(ContentFormat::Html) {
        match ctx.get_html() {
            Ok(html) if html.len() <= MAX_HTML_BYTES => snapshot.html = Some(html),
            Ok(_) => snapshot.warning = Some("HTML exceeds 1 MiB; only plain text was saved.".to_string()),
            Err(_) => snapshot.warning = Some("HTML could not be read; only plain text was saved.".to_string()),
        }
        if ctx.get_text().ok().as_deref() != Some(snapshot.content.as_str()) {
            return Ok(None);
        }
    }
    Ok(Some(snapshot))
}

#[tauri::command]
fn set_clipboard_rich_text(text: String, html: String) -> Result<(), String> {
    if html.len() > MAX_HTML_BYTES { return Err("HTML exceeds 1 MiB".to_string()) }
    let ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    ctx.set(vec![ClipboardContent::Text(text), ClipboardContent::Html(html)]).map_err(|e| e.to_string())
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

#[tauri::command]
fn set_clipboard_image_from_path(path: String) -> Result<(), String> {
    let path = PathBuf::from(path);
    let image_bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
    validate_image_bytes(&image_bytes)?;

    let ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    let img_data = RustImageData::from_bytes(&image_bytes).map_err(|e| e.to_string())?;
    ctx.set_image(img_data).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn persist_history_image(app: tauri::AppHandle, base64: String) -> Result<String, String> {
    mutate(&app, || {
    let estimated_len = estimate_base64_decoded_len(&base64)?;
    if estimated_len > MAX_IMAGE_DATA_BYTES {
        return Err(format!(
            "image payload exceeds size limit (max {} bytes)",
            MAX_IMAGE_DATA_BYTES
        ));
    }

    let image_bytes = STANDARD.decode(base64).map_err(|e| e.to_string())?;
    validate_image_bytes(&image_bytes)?;

    let dir = history_image_dir(&app)?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let file_name = format!("img-{}-{}.png", stamp, std::process::id());
    let file_path = dir.join(file_name);

    std::fs::write(&file_path, image_bytes).map_err(|e| e.to_string())?;
    cleanup_history_image_dir(&dir)?;
    Ok(file_path.to_string_lossy().to_string())
    })
}

#[tauri::command]
fn delete_history_images(app: tauri::AppHandle, paths: Vec<String>) -> Result<(), String> {
    mutate(&app, || {
    let dir = history_image_dir(&app)?;
    for raw in paths {
        let Some(path) = normalize_history_image_path(&raw) else {
            continue;
        };
        if path.exists() && path_is_within_dir(&path, &dir) {
            std::fs::remove_file(path).map_err(|e| e.to_string())?;
        }
    }
    cleanup_history_image_dir(&dir)?;
    Ok(())
    })
}

#[tauri::command]
fn clear_history_images(app: tauri::AppHandle) -> Result<(), String> {
    mutate(&app, || {
    let dir = history_image_dir(&app)?;
    for entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_file() && path_is_within_dir(&path, &dir) {
            std::fs::remove_file(path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
    })
}

#[tauri::command]
fn open_in_chrome(url: String) -> Result<(), String> {
    if !matches!(url.split_once("://"), Some(("http" | "https", _))) {
        return Err("only HTTP(S) links can be opened".to_string());
    }

    let chrome = ["PROGRAMFILES", "PROGRAMFILES(X86)", "LOCALAPPDATA"]
        .iter()
        .filter_map(|name| std::env::var_os(name))
        .map(std::path::PathBuf::from)
        .map(|path| path.join("Google\\Chrome\\Application\\chrome.exe"))
        .find(|path| path.is_file())
        .ok_or_else(|| "Google Chrome is not installed".to_string())?;

    std::process::Command::new(chrome)
        .arg(url)
        .spawn()
        .map_err(|e| format!("could not start Google Chrome: {e}"))?;
    Ok(())
}

#[tauri::command]
fn autostart_status(app: tauri::AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_autostart(app: tauri::AppHandle, enabled: bool) -> Result<bool, String> {
    mutate(&app, || {
    if enabled { app.autolaunch().enable() } else { app.autolaunch().disable() }
        .map_err(|e| e.to_string())?;
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
    })
}

#[tauri::command]
fn app_reset_status(app: tauri::AppHandle) -> Result<bool, String> {
    Ok(reset_marker_path(&app)?.exists())
}

fn write_reset_stage(path: &Path, stage: &str) -> Result<(), String> {
    let mut file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(stage.as_bytes()).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())
}

#[tauri::command]
fn reset_app_data(app: tauri::AppHandle, confirmation: String) -> Result<ClipboardStore, String> {
    if confirmation != "DELETE" { return Err("Type DELETE to confirm".to_string()) }
    let mutation = app.state::<MutationRuntime>();
    let _guard = mutation.0.lock().map_err(|_| "application storage is busy".to_string())?;
    let shortcut = app.state::<ShortcutRuntime>();
    let default = DEFAULT_SHORTCUT.parse::<Shortcut>().map_err(|e| e.to_string())?;
    // 先确认默认快捷键可用，再标记并删除应用管理的数据。
    if !app.global_shortcut().is_registered(default.clone()) {
        app.global_shortcut().register(default.clone()).map_err(|_| "Alt+C is in use; no app data was deleted".to_string())?;
    }
    let marker = reset_marker_path(&app)?;
    std::fs::create_dir_all(marker.parent().ok_or("Invalid app data directory")?).map_err(|e| e.to_string())?;
    write_reset_stage(&marker, "clearing")?;
    app.autolaunch().disable().map_err(|e| e.to_string())?;
    {
        let mut active = shortcut.shortcut.lock().map_err(|_| "shortcut settings are unavailable".to_string())?;
        if *active != DEFAULT_SHORTCUT {
            if let Ok(previous) = active.parse::<Shortcut>() { app.global_shortcut().unregister(previous).map_err(|e| e.to_string())?; }
        }
        *active = DEFAULT_SHORTCUT.to_string();
    }
    vault_lock(app.state::<VaultRuntime>())?;
    for path in [clipboard_store_path(&app)?, vault_path(&app)?, vault_local_unlock_path(&app)?, shortcut_path(&app)?] {
        for candidate in [path.clone(), path.with_extension("bak"), path.with_extension("tmp")] {
            match std::fs::remove_file(&candidate) {
                Ok(()) => (),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
                Err(error) => return Err(format!("Reset could not remove {}: {error}", candidate.file_name().unwrap_or_default().to_string_lossy())),
            }
        }
    }
    let cache = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    let images = cache.join(IMAGE_HISTORY_DIR_NAME);
    if images.exists() {
        if !path_is_within_dir(&images, &cache) { return Err("Image cache is outside app storage".to_string()) }
        for entry in std::fs::read_dir(&images).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.file_type().map_err(|e| e.to_string())?.is_dir() { return Err("Unexpected directory in image cache; reset paused".to_string()) }
            std::fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
        }
    }
    let store = default_clipboard_store();
    write_clipboard_store(&app, &store)?;
    write_shortcut_preference(&app, DEFAULT_SHORTCUT)?;
    write_reset_stage(&marker, "ready")?;
    Ok(store)
}

#[tauri::command]
fn complete_app_reset(app: tauri::AppHandle) -> Result<(), String> {
    let mutation = app.state::<MutationRuntime>();
    let _guard = mutation.0.lock().map_err(|_| "application storage is busy".to_string())?;
    let marker = reset_marker_path(&app)?;
    if std::fs::read_to_string(&marker).map_err(|e| e.to_string())? != "ready" {
        return Err("App reset is incomplete; retry reset".to_string());
    }
    std::fs::remove_file(marker).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(VaultRuntime::default())
        .manage(ShortcutRuntime::default())
        .manage(MutationRuntime::default())
        .on_window_event(|window, event| {
            if window.label() == "main" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .setup(|app| {
            if let Err(error) = fit_initial_window(app.handle()) {
                eprintln!("Could not fit initial window to monitor: {error}");
            }
            // ==========================================
            // 1. 初始化并开启【开机自启动】
            // ==========================================
            app.handle().plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec![]),
            ))?;

            let stored_shortcut = load_shortcut_preference(app.handle());
            {
                let shortcut_runtime = app.state::<ShortcutRuntime>();
                let mut active = shortcut_runtime
                    .shortcut
                    .lock()
                    .map_err(|_| "shortcut settings are unavailable")?;
                *active = stored_shortcut.clone();
            }
            let initial_shortcut = stored_shortcut
                .parse::<Shortcut>()
                .map_err(|_| "shortcut preference is invalid")?;

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        let matches_active = app
                            .state::<ShortcutRuntime>()
                            .shortcut
                            .lock()
                            .ok()
                            .and_then(|active| active.parse::<Shortcut>().ok())
                            .is_some_and(|active| shortcut == &active);
                        if matches_active && event.state() == ShortcutState::Pressed {
                            toggle_main_window(app);
                        }
                    })
                    .build(),
            )?;
            if let Err(error) = app.global_shortcut().register(initial_shortcut) {
                if !reset_marker_path(app.handle())?.exists() { return Err(error.into()) }
            }

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
            get_clipboard_snapshot,
            set_clipboard_rich_text,
            app_reset_status,
            reset_app_data,
            complete_app_reset,
            set_clipboard_image,
            set_clipboard_image_from_path,
            persist_history_image,
            delete_history_images,
            clear_history_images,
            vault_status,
            vault_set_require_password,
            vault_auto_unlock,
            vault_setup,
            vault_unlock,
            vault_replace_passwords,
            vault_change_master_password,
            vault_export_encrypted_backup,
            vault_restore_encrypted_backup,
            vault_lock,
            vault_delete,
            clipboard_store_status,
            clipboard_store_load,
            clipboard_store_replace,
            clipboard_store_migrate_legacy,
            clipboard_store_delete,
            shortcut_status,
            update_global_shortcut,
            autostart_status,
            set_autostart,
            open_in_chrome
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_password() -> VaultPassword {
        VaultPassword {
            id: 1,
            title: "Example".to_string(),
            username: "user@example.com".to_string(),
            password: "correct-horse-battery-staple".to_string(),
            collection_id: None,
        }
    }

    #[test]
    fn master_password_length_boundaries() {
        for length in [0, 7, 17] {
            assert!(create_vault_file(&"a".repeat(length), vec![]).is_err());
        }
        for length in [8, 16] {
            assert!(validate_master_password(&"a".repeat(length)).is_ok());
            assert!(validate_master_password(&"密".repeat(length)).is_ok());
            assert!(validate_master_password(&"🔒".repeat(length)).is_ok());
        }
    }

    #[cfg(windows)]
    #[test]
    fn vault_access_requires_verification_and_preserves_encrypted_data() {
        let (vault, key) = create_vault_file("Test-master-1234", vec![sample_password()]).unwrap();
        let original = serde_json::to_vec(&vault).unwrap();
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("clipboard-access-policy-test-{stamp}.bin"));
        let context = format!("synthetic:{}", vault.salt);
        assert!(configure_vault_access(&path, &vault, "incorrect", false, context.as_bytes()).is_err());
        assert!(!path.exists());
        configure_vault_access(&path, &vault, "Test-master-1234", false, context.as_bytes()).unwrap();
        let cached = local_vault_unlock::read(&path, context.as_bytes()).unwrap();
        assert_eq!(*cached, key);
        assert_eq!(decrypt_payload(&vault, &cached).unwrap().passwords.len(), 1);
        assert!(configure_vault_access(&path, &vault, "incorrect", true, context.as_bytes()).is_err());
        assert!(path.exists());
        configure_vault_access(&path, &vault, "Test-master-1234", true, context.as_bytes()).unwrap();
        assert!(!path.exists());
        assert!(original == serde_json::to_vec(&vault).unwrap());
        assert!(local_vault_unlock::read(&path, context.as_bytes()).is_err());
    }

    #[test]
    fn master_password_boundaries_round_trip() {
        let (vault, key) = create_vault_file("12345678", vec![sample_password()]).unwrap();
        assert_eq!(decrypt_payload(&vault, &key).unwrap().passwords.len(), 1);
        let (rotated, next_key) = rotate_vault_master_password(&vault, "12345678", "1234567890123456").unwrap();
        assert_eq!(decrypt_payload(&rotated, &next_key).unwrap().passwords.len(), 1);
        assert!(rotate_vault_master_password(&vault, "12345678", "12345678901234567").is_err());
    }

    #[test]
    fn legacy_long_master_password_still_unlocks() {
        let (template, _) = create_vault_file("Test-master-1234", vec![]).unwrap();
        let salt = STANDARD.decode(&template.salt).unwrap();
        let legacy_password = "legacy master password longer than sixteen";
        let legacy_key = derive_vault_key(legacy_password, &salt, &template.kdf).unwrap();
        let payload = VaultPayload { version: VAULT_VERSION, passwords: vec![sample_password()], collections: vec![] };
        let vault = encrypt_payload(&payload, &legacy_key, template.kdf, &salt).unwrap();
        let key = derive_vault_key(legacy_password, &salt, &vault.kdf).unwrap();
        assert_eq!(decrypt_payload(&vault, &key).unwrap().passwords.len(), 1);
        let (rotated, next_key) = rotate_vault_master_password(&vault, legacy_password, "New-pass-123").unwrap();
        assert_eq!(decrypt_payload(&rotated, &next_key).unwrap().passwords.len(), 1);
        assert!(create_vault_file(legacy_password, vec![]).is_err());
    }

    #[test]
    fn encrypted_vault_round_trip_preserves_passwords() {
        let (vault, key) = create_vault_file("Test-master-1234", vec![sample_password()])
            .expect("vault should be created");
        let payload = decrypt_payload(&vault, &key).expect("vault should decrypt");

        assert_eq!(payload.passwords.len(), 1);
        assert_eq!(payload.passwords[0].title, "Example");
    }

    #[test]
    fn encrypted_vault_rejects_another_master_password() {
        let (vault, _) = create_vault_file("Test-master-1234", vec![sample_password()])
            .expect("vault should be created");
        let salt = STANDARD.decode(&vault.salt).expect("salt should decode");
        let wrong_key = derive_vault_key("another long master password", &salt, &vault.kdf)
            .expect("key derivation should work");

        assert!(decrypt_payload(&vault, &wrong_key).is_err());
    }

    #[test]
    fn shortcut_normalization_requires_a_safe_modifier() {
        assert_eq!(normalize_shortcut("ctrl + shift + c").as_deref(), Ok("Ctrl+Shift+C"));
        assert_eq!(normalize_shortcut("shift + alt + ctrl + v").as_deref(), Ok("Ctrl+Alt+Shift+V"));
        assert_eq!(normalize_shortcut("alt + control + 9").as_deref(), Ok("Ctrl+Alt+9"));
        assert!(normalize_shortcut("Ctrl++V").is_err());
        assert!(normalize_shortcut("Ctrl+Ctrl+V").is_err());
        assert!(normalize_shortcut("Shift+C").is_err());
        assert!(normalize_shortcut("Alt+F1").is_err());
    }

    #[test]
    fn master_password_rotation_invalidates_the_previous_password() {
        let (vault, _) = create_vault_file("Test-master-1234", vec![sample_password()])
            .expect("vault should be created");
        let (rotated, next_key) = rotate_vault_master_password(
            &vault,
            "Test-master-1234",
            "Next-master-5678",
        )
        .expect("vault should rotate");
        let salt = STANDARD.decode(&rotated.salt).expect("salt should decode");
        let old_key = derive_vault_key("Test-master-1234", &salt, &rotated.kdf)
            .expect("old key derivation should work");

        assert!(decrypt_payload(&rotated, &old_key).is_err());
        assert_eq!(decrypt_payload(&rotated, &next_key).unwrap().passwords.len(), 1);
    }

    #[test]
    fn encrypted_backup_never_contains_the_plaintext_password() {
        let (vault, _) = create_vault_file("Test-master-1234", vec![sample_password()])
            .expect("vault should be created");
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("my-clipboard-vault-{stamp}.json"));
        export_backup_file(path.to_str().expect("temp path should be utf-8"), &vault)
            .expect("encrypted backup should export");
        let bytes = std::fs::read(&path).expect("backup should be readable");
        let restored = read_backup_vault(path.to_str().expect("temp path should be utf-8"))
            .expect("backup should parse");
        let _ = std::fs::remove_file(path);

        assert!(!String::from_utf8_lossy(&bytes).contains("correct-horse-battery-staple"));
        assert_eq!(restored.version, VAULT_VERSION);
    }

    #[test]
    fn legacy_clipboard_migration_uses_stable_category_ids() {
        let store = migrate_legacy_clipboard_store(
            vec![
                LegacyClipboardRecord {
                    kind: "text".to_string(),
                    content: "project note".to_string(),
                    id: 1,
                    timestamp: "2026-08-29".to_string(),
                    category: "Projects".to_string(),
                    restored: false,
                    is_pinned: true,
                },
                LegacyClipboardRecord {
                    kind: "text".to_string(),
                    content: "general note".to_string(),
                    id: 2,
                    timestamp: "2026-08-29".to_string(),
                    category: String::new(),
                    restored: false,
                    is_pinned: false,
                },
            ],
            vec!["全部".to_string(), "Projects".to_string()],
            None,
        )
        .expect("legacy clipboard data should migrate");

        assert_eq!(store.categories[0].id, ALL_CATEGORY_ID);
        assert_eq!(store.records[0].category_id, "category-1");
        assert_eq!(store.records[1].category_id, ALL_CATEGORY_ID);
        assert!(store.records[0].is_pinned);
    }
}
