#![allow(unused)]
mod core;
mod ui;
mod config;

use core::commands::get_command_handlers;
use config::setup::setup_app;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .invoke_handler(get_command_handlers())
        .setup(setup_app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
