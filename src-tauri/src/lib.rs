// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

use commands::add_launch::add_launch;
use commands::get_app_config::get_app_config;
use commands::get_file_info::get_file_info;
use commands::get_launch::get_launch;
use commands::open_path::open_path;
use commands::run_launch::run_launch;
use commands::run_launch_as_admin::run_launch_as_admin;
use commands::save_app_config::save_app_config;
use commands::search::exe_command::exe_command;
use commands::search_launch::search_launch;

use tray::create_tray;

mod commands;
mod db;
mod models;

mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建数据库连接 并放入线程池中共享
    let _ = db::connection::init_db();

    tauri::Builder::default()
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
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
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
            get_file_info,
            exe_command,
            save_app_config,
            get_app_config,
            open_path
        ])
        .setup(|app| {
            // 初始化数据库连接
            // let app_data_dir = app.path().app_data_dir()?;

            // dbg!(app_data_dir.to_str());
            // let conn = db::connection::init_db(app_data_dir)?;
            // // 将数据库连接存储在应用状态中
            // app.manage(Mutex::new(conn));

            // 打开开发者工具
            // app.get_webview_window("main").unwrap().open_devtools();
            #[cfg(debug_assertions)]
            {
                // 只有开发模式下执行
                if let Some(main_window) = app.get_webview_window("main") {
                    main_window.open_devtools();
                }
            }

            // 创建系统托盘
            create_tray(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
