#![allow(unused)]
mod config;
mod core;
mod ui;

use config::setup::setup_app;
use core::commands::get_command_handlers;
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
