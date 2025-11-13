use std::path::PathBuf;
use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};

/// 加载自定义托盘图标
///
/// # 参数
/// - `app`: 应用句柄
/// - `icon_path`: 图标文件路径（相对于 src-tauri 目录）
///
/// # 返回
/// 如果成功加载自定义图标，返回该图标；否则返回 None
fn load_tray_icon(
    app: &AppHandle,
    icon_path: Option<&str>,
) -> Option<tauri::image::Image<'static>> {
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
    let main_i = MenuItem::with_id(app, "main", "主窗口", true, Some("CmdOrCtrl+M"))?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, Some("CmdOrCtrl+Q"))?;
    let menu = Menu::with_items(app, &[&main_i, &quit_i])?;

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
            position,
            ..
        } => {
            println!("托盘图标被点击，位置: {:?}", position);
            let app = tray.app_handle();

            // 获取或创建 popover 窗口
            let popover = if let Some(window) = app.get_webview_window("popover") {
                window
            } else {
                // 如果窗口不存在，创建一个新的（这种情况不应该发生，因为配置中已经定义了）
                return;
            };

            // 切换 popover 的显示状态
            if let Ok(is_visible) = popover.is_visible() {
                if is_visible {
                    // 如果已经显示，则隐藏
                    let _ = popover.hide();
                } else {
                    // 如果隐藏，则显示并定位到托盘图标下方
                    show_popover_near_tray(&app, &popover, Some(position));
                }
            } else {
                // 如果无法获取可见状态，直接显示
                show_popover_near_tray(&app, &popover, Some(position));
            }
        }
        _ => {
            println!("unhandled event {event:?}");
        }
    }
}

/// 在托盘图标下方显示 popover
fn show_popover_near_tray(
    app: &AppHandle,
    popover: &tauri::WebviewWindow,
    tray_position: Option<tauri::PhysicalPosition<f64>>,
) {
    // 获取 popover 窗口的尺寸
    if let Ok(size) = popover.inner_size() {
        let width = size.width as f64;
        let height = size.height as f64;

        // 如果有托盘位置信息，使用它来定位 popover
        if let Some(tray_pos) = tray_position {
            // 托盘图标的位置
            let tray_x = tray_pos.x;
            let tray_y = tray_pos.y;

            // 计算 popover 的位置（在托盘图标下方，居中对齐）
            // 在 macOS 上，托盘图标在顶部菜单栏，popover 显示在下方
            #[cfg(target_os = "macos")]
            {
                // macOS: 托盘在顶部，popover 显示在托盘图标正下方
                // 居中对齐：popover 的中心对齐到托盘图标的中心
                let popover_x = tray_x - width / 2.0;
                // 托盘图标通常在菜单栏中，高度约为 22px，所以 popover 应该在下方约 30px 处
                let popover_y = tray_y + 30.0;
                let _ = popover.set_position(tauri::PhysicalPosition::new(
                    popover_x as i32,
                    popover_y as i32,
                ));
            }

            // 在其他平台上，也显示在托盘图标下方
            #[cfg(not(target_os = "macos"))]
            {
                // 其他平台：托盘通常在任务栏，popover 显示在上方或下方
                // 假设托盘在底部，popover 显示在上方
                let popover_x = tray_x - width / 2.0;
                let popover_y = tray_y - height - 10.0; // 托盘图标上方 10px
                let _ = popover.set_position(tauri::PhysicalPosition::new(
                    popover_x as i32,
                    popover_y as i32,
                ));
            }
        } else {
            // 如果没有托盘位置信息，使用屏幕右上角（macOS）或右下角（其他平台）
            if let Ok(Some(monitor)) = app.primary_monitor() {
                let screen_size = monitor.size();
                let screen_width = screen_size.width as f64;
                let screen_height = screen_size.height as f64;

                #[cfg(target_os = "macos")]
                {
                    // macOS: 显示在右上角下方
                    let popover_x = screen_width - width - 20.0;
                    let popover_y = 40.0;
                    let _ = popover.set_position(tauri::PhysicalPosition::new(
                        popover_x as i32,
                        popover_y as i32,
                    ));
                }

                #[cfg(not(target_os = "macos"))]
                {
                    // 其他平台: 显示在右下角
                    let popover_x = screen_width - width - 20.0;
                    let popover_y = screen_height - height - 20.0;
                    let _ = popover.set_position(tauri::PhysicalPosition::new(
                        popover_x as i32,
                        popover_y as i32,
                    ));
                }
            }
        }
    }

    // 显示并聚焦 popover
    let _ = popover.show();
    let _ = popover.set_focus();
}

/// 在 macOS 上隐藏 Dock 栏图标
#[cfg(target_os = "macos")]
pub fn hide_dock_icon() {
    unsafe {
        let app = NSApplication::sharedApplication(cocoa::base::nil);
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
    }
}

/// 在 macOS 上显示 Dock 栏图标
#[cfg(target_os = "macos")]
pub fn show_dock_icon() {
    unsafe {
        let app = NSApplication::sharedApplication(cocoa::base::nil);
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        );
    }
}

/// 在 macOS 上显示 Dock 栏图标并激活应用
#[cfg(target_os = "macos")]
pub fn show_dock_icon_and_activate() {
    unsafe {
        let app = NSApplication::sharedApplication(cocoa::base::nil);
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        );
        app.activateIgnoringOtherApps_(cocoa::base::YES);
    }
}

#[cfg(not(target_os = "macos"))]
pub fn hide_dock_icon() {}

#[cfg(not(target_os = "macos"))]
pub fn show_dock_icon() {}

/// 在 macOS 上显示 Dock 栏图标并激活应用（带应用句柄）
#[cfg(target_os = "macos")]
fn show_dock_icon_and_activate_with_app(_app_handle: &AppHandle) {
    use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};

    unsafe {
        let app = NSApplication::sharedApplication(cocoa::base::nil);

        // 恢复为常规应用（显示 Dock 图标）
        // 使用 cocoa crate 提供的类型安全 API
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        );

        // 激活应用（确保图标正确显示）
        // 这会强制 macOS 重新读取应用 bundle 中的图标信息
        app.activateIgnoringOtherApps_(cocoa::base::YES);
    }
}

/// 处理托盘菜单事件
fn handle_tray_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id.as_ref() {
        "main" => {
            println!("main menu item was clicked");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                // 隐藏 popover
                if let Some(popover) = app.get_webview_window("popover") {
                    let _ = popover.hide();
                }
                // 激活应用
                show_dock_icon_and_activate_with_app(app);
                // 显示 dock 图标
                show_dock_icon();
            }
        }
        "quit" => {
            println!("quit menu item was clicked");
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}
