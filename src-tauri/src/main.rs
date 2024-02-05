use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;
mod daemon;
mod utils;

#[tauri::command]
fn is_headset_found() -> bool {
    return utils::is_headset_found();
}

#[tauri::command]
fn read_config() -> utils::HeadsetConfig {
    utils::read_config().unwrap_or_default()
}

#[tauri::command]
fn write_config(headset_config: utils::HeadsetConfig) {
    utils::write_config(headset_config);
}

#[tokio::main]
async fn main() {
    daemon::start();

    let tray = configure_systray();
    initialize_app(tray);

    // Keep the main function running to allow the daemon to run concurrently
    loop {
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

fn configure_systray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let toggle_visibility = CustomMenuItem::new("toggle_visibility".to_string(), "Toggle Visibility");
    let tray_menu = SystemTrayMenu::new()
        .add_item(toggle_visibility)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    return SystemTray::new().with_menu(tray_menu);

}

fn initialize_app(tray: SystemTray) {
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
          "toggle_visibility" => {
            let window = app.get_window("main").unwrap();
            if window.is_visible().expect("Error checking if window is visible") {
              window.hide().unwrap();
            } else {
              window.show().unwrap();
            }
          }
          _ => {}
        }
      }
      _ => {}
  })
  .invoke_handler(tauri::generate_handler![write_config, read_config, is_headset_found])
  .build(tauri::generate_context!())
  .expect("error while building tauri application")
  .run(|_app_handle, event| match event {
    tauri::RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  });
}