use crate::config::autostart::{init_autostart_plugin, setup_autostart, AutostartConfig};
use crate::core::events::EventEmitter;
use crate::ui::menu::setup_menu;
use crate::ui::tray::{setup_tray, hide_dock_icon};
use tauri::{App, Manager};

/// 应用初始化配置
pub struct AppConfig {
    pub autostart_enabled: bool,
    pub autostart_config: AutostartConfig,
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            autostart_enabled: false, // 默认不启用自动启动
            autostart_config: AutostartConfig::default(),
        }
    }
}


/// 执行应用初始化设置
pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::default();

    // 设置菜单
    setup_menu(app.handle())?;

    // 初始化自动启动插件
    init_autostart_plugin(app.handle(), &config.autostart_config);

    // 设置自动启动（根据配置决定是否启用）
    if config.autostart_enabled {
        let _ = setup_autostart(app.handle(), true)?;
    }

    // 设置系统托盘
    setup_tray(app.handle())?;

    // 监听主窗口关闭事件，隐藏窗口而不是退出应用
    if let Some(window) = app.get_webview_window("main") {
        let window_clone = window.clone();
        let app_handle = app.handle().clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // 点击关闭按钮时隐藏窗口而不是退出应用
                // 这样应用会在后台继续运行（配合系统托盘）
                window_clone.hide().unwrap();
                
                // 在 macOS 上隐藏 Dock 栏图标
                #[cfg(target_os = "macos")]
                {
                    hide_dock_icon();
                }
                
                api.prevent_close();
            }
        });
    }

    // 初始化事件发射器
    let emitter = EventEmitter::new(app.handle().clone());
    app.manage(emitter);

    // 可以在这里启动后台任务，定期发送事件
    // let app_handle = app.handle().clone();
    // tauri::async_runtime::spawn(async move {
    //     // 后台任务逻辑
    // });

    Ok(())
}
