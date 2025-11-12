#![allow(unused)]
use crate::core::events::EventEmitter;
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
    Ok(GreetResponse {
        message: format!("Hello, {}! You've been greeted from Rust!", params.name),
    })
}

// 导出所有命令处理器
pub fn get_command_handlers(
) -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + Clone + 'static {
    tauri::generate_handler![greet]
}
