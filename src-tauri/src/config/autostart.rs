use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

/// 自动启动配置
pub struct AutostartConfig {
    pub launcher: MacosLauncher,
    pub args: Option<Vec<&'static str>>,
}

impl Default for AutostartConfig {
    fn default() -> Self {
        Self {
            launcher: MacosLauncher::LaunchAgent,
            args: Some(vec!["--flag1", "--flag2"]),
        }
    }
}

/// 初始化自动启动插件
pub fn init_autostart_plugin(app: &AppHandle, config: &AutostartConfig) {
    #[cfg(desktop)]
    {
        app.app_handle().plugin(tauri_plugin_autostart::init(
            config.launcher,
            config.args.clone(),
        ));
    }
}

/// 设置自动启动
pub fn setup_autostart(
    app: &AppHandle,
    enabled: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let autostart_manager = app.autolaunch();
    
    if enabled {
        autostart_manager.enable()?;
    } else {
        autostart_manager.disable()?;
    }
    
    let is_enabled = autostart_manager.is_enabled()?;
    println!("registered for autostart? {}", is_enabled);
    
    Ok(is_enabled)
}
