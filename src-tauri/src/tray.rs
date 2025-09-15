use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager, WebviewWindowBuilder,
};

use crate::commands::get_app_config::get_app_config;

pub fn create_tray(app: &App) {
    let quit_i = MenuItem::with_id(app, "quit", "退 出", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();

    let _ = TrayIconBuilder::new()
        .tooltip("Quick Launcher")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    let app_config = get_app_config().unwrap();
                    // dbg!(app_config);

                    let window =
                        WebviewWindowBuilder::from_config(app, &app.config().app.windows[0])
                            .unwrap()
                            .build()
                            .unwrap();
                    window
                        .set_position(tauri::PhysicalPosition {
                            x: app_config.main_window_position_x,
                            y: app_config.main_window_position_y,
                        })
                        .unwrap();

                    let _ = window.show();
                    let _ = window.set_focus();

                    // dbg!(&app.webview_windows());

                    // WebviewWindowBuilder::new(&app, "main", tauri::WebviewUrl::App("index.html".into()));
                }
            }
            _ => {}
        })
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .show_menu_on_left_click(true)
        .build(app);
}
