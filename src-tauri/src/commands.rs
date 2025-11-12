#![allow(unused)]
use crate::events::EventEmitter;
use serde::{Deserialize, Serialize};
use tauri::State;

// 定义命令参数和返回值的类型
#[derive(Debug, Serialize, Deserialize)]
pub struct GreetParams {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetResponse {
    pub message: String,
}

// 命令实现
#[tauri::command]
pub fn greet(emitter: State<EventEmitter>, params: GreetParams) -> Result<GreetResponse, String> {
    emitter.emit_data_changed("".to_string(), 0);
    Ok(GreetResponse {
        message: format!("Hello, {}! You've been greeted from Rust!", params.name),
    })
}

// 可以添加更多命令...
#[tauri::command]
pub fn get_user_info(user_id: u32) -> Result<String, String> {
    Ok(format!("User info for ID: {}", user_id))
}

// 导出所有命令处理器
// 在 Tauri 2.0 中，generate_handler! 宏可以直接使用
pub fn get_command_handlers(
) -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + Clone + 'static {
    tauri::generate_handler![greet, get_user_info]
}
