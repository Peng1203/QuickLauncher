use std::fs::OpenOptions;
use std::io::Write;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Setup panic hook BEFORE the Tauri builder.
/// Captures panics that happen during plugin init and early startup,
/// when the tracing subscriber is not yet initialized.
pub fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic payload".to_string()
        };

        // Always write directly to a fallback file (works even if tracing is not ready)
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("quicklauncher-panic.log")
        {
            let _ = writeln!(f, "[{}] PANIC at {}: {}", timestamp(), location, msg);
        }

        // Also emit via tracing (if subscriber is already initialized)
        tracing::error!(location = %location, message = %msg, "PANIC");
    }));
}

fn timestamp() -> String {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", t.as_secs())
}

/// Initialize the tracing subscriber with file + optional stdout output.
/// Must be called after setup_panic_hook().
pub fn init(app_data_dir: &std::path::Path) {
    let log_dir = app_data_dir.join("logs");
    std::fs::create_dir_all(&log_dir).ok();

    let file_appender =
        tracing_appender::rolling::RollingFileAppender::new(Rotation::DAILY, log_dir, "app.log");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    std::mem::forget(Box::new(guard));

    let file_layer = fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .with_ansi(false)
        .with_writer(non_blocking);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(
            fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .pretty()
                .with_writer(std::io::stdout),
        )
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .init();

    // Bridge log crate calls (from sea-orm, etc.) to tracing
    tracing_log::LogTracer::init().ok();
}
