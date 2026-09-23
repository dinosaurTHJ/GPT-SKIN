use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use tauri::{
    AppHandle, Manager, State,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

const TRAY_OPEN_ID: &str = "open";
const TRAY_RESTORE_ID: &str = "restore";
const TRAY_QUIT_ID: &str = "quit";
const TRAY_ID: &str = "chatgpt-skin";
const DEFAULT_IMAGE_LIBRARY_ROOT: &str = r"C:\image";
const IMAGE_LIBRARY_CONFIG_FILE: &str = "image-library-root.txt";
const IMAGE_OPACITY_CONFIG_FILE: &str = "image-opacity.txt";
const DEFAULT_IMAGE_OPACITY: f64 = 0.8;
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif"];
const SUPPORTED_VIDEO_EXTENSIONS: &[&str] = &["mp4", "webm"];
const DREAM_SKIN_PORT: i32 = 9335;

struct TrayLabels {
    open: &'static str,
    restore: &'static str,
    quit: &'static str,
}

fn tray_labels(locale: &str) -> TrayLabels {
    if locale.starts_with("zh") {
        TrayLabels {
            open: "打开 ChatGPT Skin Studio",
            restore: "恢复官方主题",
            quit: "退出 ChatGPT Skin Studio",
        }
    } else {
        TrayLabels {
            open: "Open ChatGPT Skin Studio",
            restore: "Restore Official Theme",
            quit: "Quit ChatGPT Skin Studio",
        }
    }
}

#[derive(Default)]
struct LiveThemeState {
    image_path: Mutex<Option<String>>,
    theme_id: Mutex<Option<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LiveThemeStatus {
    active: bool,
    image_path: Option<String>,
    theme_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageAsset {
    id: String,
    name: String,
    category: String,
    path: String,
    size: u64,
    extension: String,
}

fn canonical_path_is_inside(root: &Path, candidate: &Path) -> bool {
    let root = root.to_string_lossy().to_ascii_lowercase();
    let candidate = candidate.to_string_lossy().to_ascii_lowercase();
    candidate == root || candidate.starts_with(&format!("{root}\\"))
}

fn image_id(path: &Path) -> String {
    let mut digest = Sha256::new();
    digest.update(path.to_string_lossy().as_bytes());
    let full = format!("{:x}", digest.finalize());
    full[..16].to_owned()
}

fn image_library_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| format!("无法定位图片目录配置：{error}"))?;
    fs::create_dir_all(&config_dir).map_err(|error| format!("无法创建图片目录配置：{error}"))?;
    Ok(config_dir.join(IMAGE_LIBRARY_CONFIG_FILE))
}

fn stored_image_library_root(app: &AppHandle) -> Result<PathBuf, String> {
    let config_path = image_library_config_path(app)?;
    let configured = match fs::read_to_string(config_path) {
        Ok(value) => value.trim().to_owned(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(format!("无法读取图片目录配置：{error}")),
    };
    Ok(if configured.is_empty() {
        PathBuf::from(DEFAULT_IMAGE_LIBRARY_ROOT)
    } else {
        PathBuf::from(configured)
    })
}

fn image_opacity_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| format!("无法定位透明度配置：{error}"))?;
    fs::create_dir_all(&config_dir).map_err(|error| format!("无法创建透明度配置：{error}"))?;
    Ok(config_dir.join(IMAGE_OPACITY_CONFIG_FILE))
}

fn normalize_image_opacity(value: f64) -> Result<f64, String> {
    if !value.is_finite() || !(0.1..=1.0).contains(&value) {
        return Err("透明度必须在 10% 到 100% 之间".into());
    }
    Ok((value * 1000.0).round() / 1000.0)
}

fn stored_image_opacity(app: &AppHandle) -> Result<f64, String> {
    let config_path = image_opacity_config_path(app)?;
    let configured = match fs::read_to_string(config_path) {
        Ok(value) => value.trim().parse::<f64>().ok(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(format!("无法读取透明度配置：{error}")),
    };
    Ok(configured
        .and_then(|value| normalize_image_opacity(value).ok())
        .unwrap_or(DEFAULT_IMAGE_OPACITY))
}

fn canonical_image_library_root(app: &AppHandle) -> Result<PathBuf, String> {
    let configured = stored_image_library_root(app)?;
    if !configured.is_absolute() || !configured.is_dir() {
        return Err(format!("图片目录不存在：{}", configured.display()));
    }
    fs::canonicalize(&configured)
        .map_err(|error| format!("无法读取图片目录 {}：{error}", configured.display()))
}

fn allow_image_library_root(app: &AppHandle, root: &Path) -> Result<(), String> {
    app.asset_protocol_scope()
        .allow_directory(root, true)
        .map_err(|error| format!("无法授权图片目录 {}：{error}", root.display()))
}

fn validate_image_path(raw_path: &str, app: &AppHandle) -> Result<(PathBuf, String, u64), String> {
    let root = canonical_image_library_root(app)?;
    let path = fs::canonicalize(raw_path).map_err(|error| format!("背景资源路径无效：{error}"))?;
    if !canonical_path_is_inside(&root, &path) {
        return Err(format!(
            "只能使用 {} 目录及其子目录中的背景资源",
            root.display()
        ));
    }
    let metadata = fs::metadata(&path).map_err(|error| format!("无法读取背景资源：{error}"))?;
    if !metadata.is_file() {
        return Err("选择的路径不是背景资源文件".into());
    }
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| "背景资源缺少扩展名".to_owned())?;
    if !SUPPORTED_IMAGE_EXTENSIONS.contains(&extension.as_str())
        && !SUPPORTED_VIDEO_EXTENSIONS.contains(&extension.as_str())
    {
        return Err("仅支持 JPG、PNG、WebP、GIF、MP4 和 WebM".into());
    }
    if metadata.len() == 0 {
        return Err("背景资源不能为空".into());
    }
    Ok((path, extension, metadata.len()))
}

fn image_category(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .ok()
        .and_then(|relative| relative.parent())
        .and_then(|parent| parent.to_str())
        .filter(|parent| !parent.is_empty())
        .unwrap_or("未分类")
        .replace('\\', " / ")
}

fn runtime_root(app: &AppHandle) -> Result<PathBuf, String> {
    let mut candidates = Vec::new();
    if let Ok(resource_dir) = app.path().resource_dir() {
        candidates.push(resource_dir.join(r"resources\dream-skin-runtime"));
        candidates.push(resource_dir.join("dream-skin-runtime"));
    }
    if let Ok(executable) = std::env::current_exe() {
        if let Some(parent) = executable.parent() {
            candidates.push(parent.join(r"resources\dream-skin-runtime"));
            candidates.push(parent.join("dream-skin-runtime"));
        }
    }
    candidates
        .push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(r"resources\dream-skin-runtime"));
    candidates
        .into_iter()
        .find(|candidate| {
            candidate.join(r"scripts\manager-bridge.ps1").is_file()
                && candidate.join(r"scripts\start-dream-skin.ps1").is_file()
        })
        .ok_or_else(|| "本地换肤运行时缺失，请重新安装 ChatGPT Skin Studio。".into())
}

#[cfg(target_os = "windows")]
fn powershell_compatible_path(path: &Path) -> PathBuf {
    let path_text = path.to_string_lossy();
    if let Some(unc_path) = path_text.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{unc_path}"));
    }
    if let Some(normal_path) = path_text.strip_prefix(r"\\?\") {
        return PathBuf::from(normal_path);
    }
    path.to_path_buf()
}

fn last_output_lines(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    text.lines()
        .rev()
        .take(8)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned()
}

fn run_dream_skin_bridge(
    app: &AppHandle,
    action: &str,
    image_path: Option<&Path>,
    opacity: Option<f64>,
) -> Result<(), String> {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (app, action, image_path, opacity);
        return Err("当前版本仅支持 Windows 官方 ChatGPT 桌面端。".into());
    }

    #[cfg(target_os = "windows")]
    {
        let root = runtime_root(app)?;
        // PowerShell treats a `\\?\`-prefixed -File path as requiring a signed
        // script even when RemoteSigned is selected. Tauri may return resource
        // paths with that prefix on Windows, so pass a normal Win32 path.
        let script = powershell_compatible_path(&root.join(r"scripts\manager-bridge.ps1"));
        let mut command = Command::new("powershell.exe");
        command.creation_flags(0x08000000);
        command
            .arg("-NoLogo")
            .arg("-NoProfile")
            .arg("-NonInteractive")
            .arg("-ExecutionPolicy")
            .arg("RemoteSigned")
            .arg("-File")
            .arg(&script)
            .arg("-Action")
            .arg(action)
            .arg("-Port")
            .arg(DREAM_SKIN_PORT.to_string())
            .current_dir(&root)
            .env("DREAMSKIN_LANG", "zh-CN")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(image_path) = image_path {
            command
                .arg("-ImagePath")
                .arg(powershell_compatible_path(image_path));
        }
        if let Some(opacity) = opacity {
            command.arg("-Opacity").arg(format!("{opacity:.3}"));
        }
        let output = command
            .output()
            .map_err(|error| format!("无法启动本地换肤服务：{error}"))?;
        if !output.status.success() {
            let details = last_output_lines(&output.stderr);
            let fallback = last_output_lines(&output.stdout);
            let details = if details.is_empty() {
                fallback
            } else {
                details
            };
            return Err(if details.is_empty() {
                format!("本地换肤操作失败：{action}")
            } else {
                details
            });
        }
        Ok(())
    }
}

fn clear_live_state(state: &LiveThemeState) -> Result<(), String> {
    *state
        .image_path
        .lock()
        .map_err(|_| "本地主题状态已损坏".to_owned())? = None;
    *state
        .theme_id
        .lock()
        .map_err(|_| "本地主题状态已损坏".to_owned())? = None;
    Ok(())
}

fn set_live_state(state: &LiveThemeState, image_path: &Path) -> Result<LiveThemeStatus, String> {
    let image_path = image_path.to_string_lossy().to_string();
    let theme_id = format!("local.image.{}", image_id(Path::new(&image_path)));
    *state
        .image_path
        .lock()
        .map_err(|_| "本地主题状态已损坏".to_owned())? = Some(image_path.clone());
    *state
        .theme_id
        .lock()
        .map_err(|_| "本地主题状态已损坏".to_owned())? = Some(theme_id.clone());
    Ok(LiveThemeStatus {
        active: true,
        image_path: Some(image_path),
        theme_id: Some(theme_id),
    })
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn should_hide_main_window(label: &str, close_requested: bool) -> bool {
    label == "main" && close_requested
}

fn restore_theme(app: &AppHandle) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let _ = tauri::async_runtime::spawn_blocking({
            let app = app.clone();
            move || run_dream_skin_bridge(&app, "restore", None, None)
        })
        .await;
    });
}

fn setup_tray(app: &mut tauri::App) -> tauri::Result<()> {
    let labels = tray_labels("zh-CN");
    let open = MenuItemBuilder::with_id(TRAY_OPEN_ID, labels.open).build(app)?;
    let restore = MenuItemBuilder::with_id(TRAY_RESTORE_ID, labels.restore).build(app)?;
    let quit = MenuItemBuilder::with_id(TRAY_QUIT_ID, labels.quit).build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&open, &restore, &quit])
        .build()?;

    TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("ChatGPT Skin Studio")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            TRAY_OPEN_ID => show_main_window(app),
            TRAY_RESTORE_ID => restore_theme(app),
            // 退出管理程序不应恢复皮肤，也不能影响正在运行的 ChatGPT。
            TRAY_QUIT_ID => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .icon(tauri::include_image!("icons/tray-windows.png"))
        .build(app)?;
    Ok(())
}

#[tauri::command]
fn get_image_library_root(app: AppHandle) -> Result<String, String> {
    Ok(stored_image_library_root(&app)?
        .to_string_lossy()
        .to_string())
}

#[tauri::command]
fn set_image_library_root(path: String, app: AppHandle) -> Result<String, String> {
    let raw_path = path.trim();
    if raw_path.is_empty() {
        return Err("图片目录不能为空".into());
    }
    let selected = PathBuf::from(raw_path);
    if !selected.is_absolute() || !selected.is_dir() {
        return Err(format!("图片目录不存在：{raw_path}"));
    }
    let root = fs::canonicalize(&selected)
        .map_err(|error| format!("无法读取图片目录 {raw_path}：{error}"))?;
    let config_path = image_library_config_path(&app)?;
    fs::write(&config_path, root.to_string_lossy().as_bytes())
        .map_err(|error| format!("无法保存图片目录配置：{error}"))?;
    allow_image_library_root(&app, &root)?;
    Ok(root.to_string_lossy().to_string())
}

#[tauri::command]
fn get_image_opacity(app: AppHandle) -> Result<f64, String> {
    stored_image_opacity(&app)
}

#[tauri::command]
async fn set_image_opacity(opacity: f64, app: AppHandle) -> Result<f64, String> {
    let opacity = normalize_image_opacity(opacity)?;
    let config_path = image_opacity_config_path(&app)?;
    fs::write(&config_path, format!("{opacity:.3}").as_bytes())
        .map_err(|error| format!("无法保存透明度配置：{error}"))?;
    tauri::async_runtime::spawn_blocking(move || {
        run_dream_skin_bridge(&app, "set-opacity", None, Some(opacity))
    })
    .await
    .map_err(|error| format!("透明度应用任务失败：{error}"))??;
    Ok(opacity)
}

fn list_image_assets_sync(app: AppHandle) -> Result<Vec<ImageAsset>, String> {
    let root = canonical_image_library_root(&app)?;
    allow_image_library_root(&app, &root)?;
    let mut directories = vec![root.clone()];
    let mut assets = Vec::new();
    while let Some(directory) = directories.pop() {
        let entries = fs::read_dir(&directory)
            .map_err(|error| format!("无法扫描图片目录 {}：{error}", directory.display()))?;
        for entry in entries {
            let entry = entry.map_err(|error| format!("读取图片目录项失败：{error}"))?;
            let file_type = entry
                .file_type()
                .map_err(|error| format!("读取图片类型失败：{error}"))?;
            if file_type.is_symlink() {
                continue;
            }
            let path = entry.path();
            if file_type.is_dir() {
                directories.push(path);
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            let path = match fs::canonicalize(path) {
                Ok(path) if canonical_path_is_inside(&root, &path) => path,
                _ => continue,
            };
            let metadata =
                fs::metadata(&path).map_err(|error| format!("读取图片信息失败：{error}"))?;
            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .map(|value| value.to_ascii_lowercase());
            let Some(extension) = extension else { continue };
            if !SUPPORTED_IMAGE_EXTENSIONS.contains(&extension.as_str())
                && !SUPPORTED_VIDEO_EXTENSIONS.contains(&extension.as_str())
            {
                continue;
            }
            if metadata.len() == 0 {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|value| value.to_str())
                .filter(|value| !value.is_empty())
                .unwrap_or("未命名背景")
                .to_owned();
            assets.push(ImageAsset {
                id: image_id(&path),
                name,
                category: image_category(&root, &path),
                path: path.to_string_lossy().to_string(),
                size: metadata.len(),
                extension,
            });
        }
    }
    assets.sort_by(|left, right| {
        left.category
            .cmp(&right.category)
            .then_with(|| left.name.cmp(&right.name))
    });
    Ok(assets)
}

#[tauri::command]
async fn list_image_assets(app: AppHandle) -> Result<Vec<ImageAsset>, String> {
    tauri::async_runtime::spawn_blocking(move || list_image_assets_sync(app))
        .await
        .map_err(|error| format!("图片扫描任务失败：{error}"))?
}

#[tauri::command]
async fn open_chatgpt_window(app: AppHandle) -> Result<bool, String> {
    tauri::async_runtime::spawn_blocking(move || {
        run_dream_skin_bridge(&app, "ensure-session", None, None)
    })
    .await
    .map_err(|error| format!("启动 ChatGPT 会话失败：{error}"))??;
    Ok(true)
}

#[tauri::command]
async fn apply_live_image_theme(
    image_path: String,
    _locale: String,
    requested_opacity: Option<f64>,
    app: AppHandle,
    state: State<'_, LiveThemeState>,
) -> Result<LiveThemeStatus, String> {
    let (source, _, _) = validate_image_path(&image_path, &app)?;
    let opacity = if let Some(requested) = requested_opacity {
        let normalized = normalize_image_opacity(requested)?;
        let config_path = image_opacity_config_path(&app)?;
        fs::write(&config_path, format!("{normalized:.3}").as_bytes())
            .map_err(|error| format!("无法保存透明度配置：{error}"))?;
        normalized
    } else {
        stored_image_opacity(&app)?
    };
    let source_for_bridge = source.clone();
    tauri::async_runtime::spawn_blocking(move || {
        run_dream_skin_bridge(&app, "apply-image", Some(&source_for_bridge), Some(opacity))
    })
    .await
    .map_err(|error| format!("应用主题任务失败：{error}"))??;
    set_live_state(&state, &source)
}

#[tauri::command]
async fn restore_live_theme(
    app: AppHandle,
    state: State<'_, LiveThemeState>,
) -> Result<bool, String> {
    tauri::async_runtime::spawn_blocking(move || {
        run_dream_skin_bridge(&app, "restore", None, None)
    })
    .await
    .map_err(|error| format!("恢复主题任务失败：{error}"))??;
    clear_live_state(&state)?;
    Ok(true)
}

#[tauri::command]
fn get_live_theme_status(state: State<'_, LiveThemeState>) -> Result<LiveThemeStatus, String> {
    let image_path = state
        .image_path
        .lock()
        .map_err(|_| "本地主题状态已损坏".to_owned())?
        .clone();
    let theme_id = state
        .theme_id
        .lock()
        .map_err(|_| "本地主题状态已损坏".to_owned())?
        .clone();
    Ok(LiveThemeStatus {
        active: theme_id.is_some(),
        image_path,
        theme_id,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .manage(LiveThemeState::default())
        .setup(|app| {
            setup_tray(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            let close_requested = matches!(event, tauri::WindowEvent::CloseRequested { .. });
            if should_hide_main_window(window.label(), close_requested)
                && let tauri::WindowEvent::CloseRequested { api, .. } = event
            {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_image_library_root,
            set_image_library_root,
            get_image_opacity,
            set_image_opacity,
            list_image_assets,
            open_chatgpt_window,
            apply_live_image_theme,
            restore_live_theme,
            get_live_theme_status,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run ChatGPT Skin Studio");
}

#[cfg(test)]
mod tests {
    use super::{TrayLabels, should_hide_main_window, tray_labels};

    #[test]
    fn localizes_tray_labels() {
        let labels: TrayLabels = tray_labels("zh-CN");
        assert_eq!(labels.restore, "恢复官方主题");
        assert_eq!(tray_labels("en").restore, "Restore Official Theme");
    }

    #[test]
    fn hides_only_the_main_window_on_close() {
        assert!(should_hide_main_window("main", true));
        assert!(!should_hide_main_window("main", false));
        assert!(!should_hide_main_window("chatgpt", true));
    }
}
