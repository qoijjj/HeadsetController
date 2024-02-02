// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn is_headset_found() -> bool {
    let output = Command::new("headsetcontrol")
    .output()
    .expect("Failed to execute command");
    
    let output_str = String::from_utf8_lossy(&output.stderr);
    return !output_str.contains("No supported headset found");
}

#[tauri::command]
fn read_config() -> bool {
    let output = Command::new("headsetcontrol")
    .output()
    .expect("Failed to execute command");
    
    let output_str = String::from_utf8_lossy(&output.stderr);
    return !output_str.contains("No supported headset found");
}

#[tauri::command]
fn write_config() -> bool {
    let output = Command::new("headsetcontrol")
    .output()
    .expect("Failed to execute command");
    
    let output_str = String::from_utf8_lossy(&output.stderr);
    return !output_str.contains("No supported headset found");
}

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;
mod daemon;

#[tokio::main]
async fn main() {
    daemon::start();

    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_window_event(|event| match event.event() {
          tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
          }
          _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "quit" => {
                  std::process::exit(0);
                }
                "hide" => {
                  let window = app.get_window("main").unwrap();
                  window.hide().unwrap();
                }
                "show" => {
                  let window = app.get_window("main").unwrap();
                  window.show().unwrap();
                }
                _ => {}
              }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![is_headset_found])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
          tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
          }
          _ => {}
        });

    // Keep the main function running to allow the daemon to run concurrently
    loop {
      // Do other things if needed
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
