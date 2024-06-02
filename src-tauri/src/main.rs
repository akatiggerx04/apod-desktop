// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, Manager};

fn main() {
    // let show = CustomMenuItem::new("show".to_string(), "Show APOD");
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let tray_menu = SystemTrayMenu::new()
    //     .add_item(show)
    //     .add_native_item(SystemTrayMenuItem::Separator)
    //     .add_item(quit);
    // let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        // .system_tray(tray)
        // .on_system_tray_event(|app, event| match event {
        //     tauri::SystemTrayEvent::LeftClick {
        //         position: _,
        //         size: _,
        //         ..
        //     } => {
        //         if let Some(window) = app.get_window("main") {
        //             window.show().unwrap();
        //             window.set_focus().unwrap();
        //         }
        //     }
        //     tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        //         "show" => {
        //             if let Some(window) = app.get_window("main") {
        //                 window.show().unwrap();
        //                 window.set_focus().unwrap();
        //             }
        //         }
        //         "quit" => {
        //             app.exit(0);
        //         }
        //         _ => {}
        //     },
        //     _ => {}
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
