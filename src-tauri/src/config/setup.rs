#![allow(unexpected_cfgs)] // 抑制来自 objc crate 宏的警告

use crate::config::autostart::{init_autostart_plugin, setup_autostart, AutostartConfig};
use crate::core::events::EventEmitter;
use crate::ui::menu::setup_menu;
use crate::ui::tray::{setup_tray, hide_dock_icon};
use tauri::{App, Manager};

#[cfg(target_os = "macos")]
use cocoa::appkit::NSWindow;
#[cfg(target_os = "macos")]
use cocoa::base::id;

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

    // 监听 popover 窗口失焦事件，点击外部区域时隐藏
    if let Some(popover_window) = app.get_webview_window("popover") {
        let popover_clone = popover_window.clone();
        
        // 在 macOS 上设置窗口圆角
        #[cfg(target_os = "macos")]
        {
            set_window_rounded_corners(&popover_window);
        }
        
        popover_window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                // 窗口失去焦点时隐藏 popover
                let _ = popover_clone.hide();
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

/// 在 macOS 上设置窗口圆角
#[cfg(target_os = "macos")]
fn set_window_rounded_corners(window: &tauri::WebviewWindow) {
    use objc::*;
    
    unsafe {
        // 获取窗口的 NSWindow 对象
        // ns_window() 返回 *mut c_void，需要转换为 id
        if let Ok(ns_window_ptr) = window.ns_window() {
            let ns_window: id = ns_window_ptr as id;
            
            if !ns_window.is_null() {
                // 获取窗口的 contentView
                let content_view: id = msg_send![ns_window, contentView];
                if !content_view.is_null() {
                    // 启用 layer-backed view
                    let _: () = msg_send![content_view, setWantsLayer: cocoa::base::YES];
                    
                    // 获取或创建 layer
                    let layer: id = msg_send![content_view, layer];
                    if !layer.is_null() {
                        // 设置圆角半径（16.0 对应 CSS 中的 16px）
                        let corner_radius: f64 = 16.0;
                        let _: () = msg_send![layer, setCornerRadius: corner_radius];
                        
                        // 启用遮罩到边界，确保内容被裁剪到圆角边界内
                        let _: () = msg_send![layer, setMasksToBounds: cocoa::base::YES];
                    }
                }
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn set_window_rounded_corners(_window: &tauri::WebviewWindow) {
    // 其他平台不需要特殊处理
}
