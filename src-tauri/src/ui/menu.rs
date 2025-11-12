use tauri::{AppHandle, Manager};
use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem, Submenu};

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
/// 在 macOS 上，菜单项必须组织到子菜单中，顶层项目会被忽略
/// 第一个菜单（应用菜单）保持最小化，所有功能菜单项都放在后面的菜单中
/// 
/// 注意：菜单宽度由系统根据文本内容自动计算，可以通过添加空格或快捷键标记来间接调整
pub fn setup_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 创建菜单项
    // 可以通过 format_menu_text 函数来调整菜单宽度
    // 例如：format_menu_text("打开", 10) 会确保菜单项至少有 10 个字符宽度
    let open_text = format_menu_text("打开窗口", 10);
    let close_text = format_menu_text("关闭窗口 ", 10);
    let quit_text = format_menu_text("退出应用", 10);
    let about_text = format_menu_text("关于应用", 10);
    
    // 创建菜单项并添加快捷键
    // accelerator 格式: "CmdOrCtrl+Key" 或 "Cmd+Key" (macOS) 或 "Ctrl+Key" (Windows/Linux)
    // 在 macOS 上，Cmd 会显示为 ⌘ 符号
    let open_item = MenuItem::with_id(app, "open", &open_text, true, Some("CmdOrCtrl+O"))?;
    let close_item = MenuItem::with_id(app, "close", &close_text, true, Some("CmdOrCtrl+W"))?;
    let quit_item = MenuItem::with_id(app, "quit", &quit_text, true, Some("CmdOrCtrl+Q"))?;
    let about_item = MenuItem::with_id(app, "about", &about_text, true, Some("CmdOrCtrl+I"))?;
    
    // 第一个菜单（应用菜单）- 只保留系统默认的 Quit，其他功能都放在后面
    // 在 macOS 上，第一个子菜单会自动显示为应用名称
    let app_submenu = Submenu::with_items(
        app, 
        "", // 空字符串，macOS 会自动使用应用名称
        true, 
        &[]
    )?;
    
    // 文件菜单 - 包含所有主要功能
    let file_submenu = Submenu::with_items(
        app, 
        "文件", 
        true, 
        &[&open_item, &close_item, &quit_item]
    )?;
    
    // 关于菜单
    let about_submenu = Submenu::with_items(
        app,
        "帮助",
        true,
        &[&about_item]
    )?;
    
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
fn handle_about_event(app_handle: &tauri::AppHandle) {
    println!("about event");
}