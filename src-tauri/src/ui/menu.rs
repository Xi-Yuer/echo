use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Manager};

/// 格式化菜单项文本，通过添加空格来调整菜单宽度
///
/// # 参数
/// - `text`: 原始文本
/// - `min_width`: 最小宽度（字符数），如果文本长度小于此值，会在右侧添加空格
///
/// # 示例
/// ```
/// format_menu_text("打开", 8) // 返回 "打开      " (6个空格)
/// ```
fn format_menu_text(text: &str, min_width: usize) -> String {
    let current_width = text.chars().count();
    if current_width < min_width {
        let spaces = min_width - current_width;
        format!("{}{}", text, " ".repeat(spaces))
    } else {
        text.to_string()
    }
}

/// 创建并设置应用菜单
pub fn setup_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 创建菜单项
    let open_text = format_menu_text("打开窗口", 10);
    let close_text = format_menu_text("关闭窗口 ", 10);
    let quit_text = format_menu_text("退出应用", 10);
    let about_text = format_menu_text("关于应用", 10);

    // 创建菜单项并添加快捷键
    let open_item = MenuItem::with_id(app, "open", &open_text, true, Some("CmdOrCtrl+O"))?;
    let close_item = MenuItem::with_id(app, "close", &close_text, true, Some("CmdOrCtrl+W"))?;
    let quit_item = MenuItem::with_id(app, "quit", &quit_text, true, Some("CmdOrCtrl+Q"))?;
    let about_item = MenuItem::with_id(app, "about", &about_text, true, Some("CmdOrCtrl+I"))?;

    // 在 macOS 上，第一个子菜单会自动显示为应用名称
    let app_submenu = Submenu::with_items(
        app,
        "", // 空字符串，macOS 会自动使用应用名称
        true,
        &[],
    )?;

    // 文件菜单 - 包含所有主要功能
    let file_submenu =
        Submenu::with_items(app, "文件", true, &[&open_item, &close_item, &quit_item])?;

    // 关于菜单
    let about_submenu = Submenu::with_items(app, "帮助", true, &[&about_item])?;

    // 构建菜单栏 - 第一个菜单最小化，功能都在后面
    let menu = MenuBuilder::new(app)
        .items(&[&app_submenu, &file_submenu, &about_submenu])
        .build()?;

    app.set_menu(menu)?;

    app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
        match event.id().0.as_str() {
            "open" => {
                handle_open_event(app_handle);
            }
            "close" => {
                handle_close_event(app_handle);
            }
            "quit" => {
                app_handle.exit(0);
            }
            "about" => {
                handle_about_event(app_handle);
            }
            _ => {
                // PredefinedMenuItem 的 quit 事件可能使用不同的 ID
                println!("unexpected menu event: {:?}", event.id());
            }
        }
    });

    Ok(())
}

/// 处理打开菜单事件
fn handle_open_event(app_handle: &tauri::AppHandle) {
    // 在这里添加打开逻辑
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// 处理关闭菜单事件
fn handle_close_event(app_handle: &tauri::AppHandle) {
    // 在这里添加关闭逻辑
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
}

/// 处理关于菜单事件
/// 创建一个关于窗口显示应用信息（使用前端页面）
fn handle_about_event(app_handle: &tauri::AppHandle) {
    // 检查关于窗口是否已经存在
    if let Some(about_window) = app_handle.get_webview_window("about") {
        let _ = about_window.show();
        let _ = about_window.set_focus();
        return;
    }

    // 使用 WebviewWindowBuilder 创建关于窗口
    // 加载前端的 About 页面，使用路由路径 /about
    let _ = tauri::WebviewWindowBuilder::new(
        app_handle,
        "about",
        tauri::WebviewUrl::App("/about".into()),
    )
    .title("关于 Echo")
    .inner_size(520.0, 600.0)
    .resizable(false)
    .center()
    .build();
}
