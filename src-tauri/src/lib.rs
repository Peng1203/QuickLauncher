// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::models::app_config_state::AppConfigState;
use commands::add_category::add_category;
use commands::add_launch::add_launch;
use commands::add_or_update_autocomplete::add_or_update_autocomplete;
use commands::delete_launch::delete_launch;
use commands::exe_command::exe_command;
use commands::get_app_config::get_app_config;
use commands::get_autocomplete::get_autocomplete;
use commands::get_category::get_category;
use commands::get_file_info::get_file_info;
use commands::get_launch::get_launch;
use commands::get_local_icon_base64::get_local_icon_base64;
use commands::get_online_img_base64::get_online_img_base64;
use commands::get_website_info::get_website_info;
use commands::open_path::open_path;
use commands::rename_launch::rename_launch;
use commands::run_launch::run_launch;
use commands::run_launch_as_admin::run_launch_as_admin;
use commands::save_app_config::save_app_config;
use commands::search_launch::search_launch;
use commands::set_app_config::set_app_config;
use commands::update_category::update_category;
use commands::update_launch::update_launch;
use std::sync::Mutex;
use tauri::{Manager, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

use tray::create_tray;
mod commands;
mod common;
mod db;
mod models;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建数据库连接 并放入线程池中共享
    let _ = db::connection::init_db();

    tauri::Builder::default()
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    // tauri_plugin_log::TargetKind::Webview,
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let main_window = app.get_webview_window("main").unwrap();
            let _ = main_window.show();
            let _ = main_window.set_focus();
        }))
        .invoke_handler(tauri::generate_handler![
            run_launch,
            run_launch_as_admin,
            add_launch,
            get_launch,
            search_launch,
            rename_launch,
            delete_launch,
            update_launch,
            get_file_info,
            exe_command,
            save_app_config,
            get_app_config,
            open_path,
            get_website_info,
            add_category,
            get_category,
            set_app_config,
            update_category,
            get_local_icon_base64,
            get_online_img_base64,
            add_or_update_autocomplete,
            get_autocomplete
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            let search_window = app.get_webview_window("search").unwrap();
            let setting_window = app.get_webview_window("setting").unwrap();
            let transparent_drag_window = app.get_webview_window("transparentDrag").unwrap();

            main_window.on_window_event(|event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                }
            });
            search_window.on_window_event(|event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                }
            });
            setting_window.on_window_event(|event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                }
            });
            transparent_drag_window.on_window_event(|event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                }
            });
            // 初始化数据库连接
            // let app_data_dir = app.path().app_data_dir()?;

            // dbg!(app_data_dir.to_str());
            // let conn = db::connection::init_db(app_data_dir)?;
            // // 将数据库连接存储在应用状态中
            // app.manage(Mutex::new(conn));

            // 初始化应用配置
            app.manage(Mutex::new(AppConfigState::default()));

            #[cfg(debug_assertions)]
            {
                // 只有开发模式下执行
                if let Some(main_window) = app.get_webview_window("main") {
                    main_window.open_devtools();
                }
                if let Some(transparent_drag_window) = app.get_webview_window("transparentDrag") {
                    transparent_drag_window.open_devtools();
                }
            }

            // 创建系统托盘
            create_tray(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
