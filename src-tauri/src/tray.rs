use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager, WebviewWindowBuilder,
};
use tauri_plugin_pinia::ManagerExt;

pub fn create_tray(app: &App) {
    let settings_i = MenuItem::with_id(app, "settings", "设 置", true, None::<&str>).unwrap();
    let quit_i = MenuItem::with_id(app, "quit", "退 出", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&settings_i, &quit_i]).unwrap();

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
                    let main_window_position_x = app
                        .pinia()
                        .get::<i32>("appConfig", "mainWindowPositionX")
                        .unwrap();
                    let main_window_position_y = app
                        .pinia()
                        .get::<i32>("appConfig", "mainWindowPositionY")
                        .unwrap();

                    let window =
                        WebviewWindowBuilder::from_config(app, &app.config().app.windows[0])
                            .unwrap()
                            .build()
                            .unwrap();
                    window
                        .set_position(tauri::PhysicalPosition {
                            x: main_window_position_x,
                            y: main_window_position_y,
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
            "settings" => {
                // 检查设置窗口是否已存在
                if let Some(window) = app.get_webview_window("setting") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    let setting_window_position_x = app
                        .pinia()
                        .get::<i32>("appConfig", "settingWindowPositionX")
                        .unwrap();
                    let setting_window_position_y = app
                        .pinia()
                        .get::<i32>("appConfig", "settingWindowPositionY")
                        .unwrap();

                    // 创建新的设置窗口
                    let settings_window =
                        WebviewWindowBuilder::from_config(app, &app.config().app.windows[2])
                            .unwrap()
                            .build()
                            .unwrap();

                    if setting_window_position_x != 0 || setting_window_position_y != 0 {
                        settings_window
                            .set_position(tauri::PhysicalPosition {
                                x: setting_window_position_x,
                                y: setting_window_position_y,
                            })
                            .unwrap();
                    }

                    let _ = settings_window.show();
                    let _ = settings_window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .show_menu_on_left_click(true)
        .build(app);
}
