#![allow(unused)]
mod commands;
mod events;

use commands::get_command_handlers;
use events::EventEmitter;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(get_command_handlers())
        .setup(|app| {
            // 初始化事件发射器
            let emitter = EventEmitter::new(app.handle().clone());
            app.manage(emitter);
            
            // 可以在这里启动后台任务，定期发送事件
            // let app_handle = app.handle().clone();
            // tauri::async_runtime::spawn(async move {
            //     // 后台任务逻辑
            // });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}