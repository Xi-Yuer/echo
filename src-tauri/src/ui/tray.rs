use tauri::{AppHandle, Manager};
use tauri::menu::MenuItem;
use tauri::menu::Menu;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use std::path::PathBuf;

/// 加载自定义托盘图标
/// 
/// # 参数
/// - `app`: 应用句柄
/// - `icon_path`: 图标文件路径（相对于 src-tauri 目录）
/// 
/// # 返回
/// 如果成功加载自定义图标，返回该图标；否则返回 None
fn load_tray_icon(app: &AppHandle, icon_path: Option<&str>) -> Option<tauri::image::Image<'static>> {
    if let Some(path) = icon_path {
        // 尝试加载图标的辅助函数
        let try_load = |file_path: &std::path::Path| -> Option<tauri::image::Image<'static>> {
            if !file_path.exists() {
                return None;
            }
            
            // 读取文件
            let bytes = std::fs::read(file_path).ok()?;
            
            // 使用 image crate 解码图片
            let img = image::load_from_memory(&bytes).ok()?;
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            
            // 转换为 Tauri Image
            Some(tauri::image::Image::new_owned(
                rgba.into_raw(),
                width,
                height,
            ))
        };
        
        // 首先尝试从资源目录加载（生产环境）
        if let Ok(resource_dir) = app.path().resource_dir() {
            let icon_file = resource_dir.join(path);
            if let Some(icon) = try_load(&icon_file) {
                return Some(icon);
            }
        }
        
        // 尝试从项目目录加载（开发环境）
        let project_icon = PathBuf::from("src-tauri").join(path);
        if let Some(icon) = try_load(&project_icon) {
            return Some(icon);
        }
    }
    
    None
}

/// 创建并设置系统托盘
pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;
    
    // 尝试加载自定义托盘图标
    // 可以指定图标路径，例如: "icons/32x32.png" 或 "icons/icon.png"
    // 推荐使用 16x16 或 32x32 的 PNG 图标以获得最佳显示效果
    let tray_icon = if let Some(custom_icon) = load_tray_icon(app, Some("icons/32x32.png")) {
        custom_icon
    } else {
        // 如果加载失败，使用默认窗口图标
        app.default_window_icon()
            .ok_or("无法加载默认窗口图标")?
            .clone()
    };
    
    let tray = TrayIconBuilder::new()
        .on_tray_icon_event(handle_tray_icon_event)
        .on_menu_event(handle_tray_menu_event)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(tray_icon)
        .build(app)?;
    
    Ok(())
}

/// 处理托盘图标事件
fn handle_tray_icon_event(tray: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("left click pressed and released");
            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {
            println!("unhandled event {event:?}");
        }
    }
}

/// 处理托盘菜单事件
fn handle_tray_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            println!("quit menu item was clicked");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}
