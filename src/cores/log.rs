use crate::utils::common;
use std::env::var;

pub fn init_log() -> tracing_appender::non_blocking::WorkerGuard {
    let env_log_level = var("LOG_LEVEL").unwrap_or("INFO".to_string());

    let with_file = true;
    let with_line_number = true;
    let with_target = false;
    let log_level = match env_log_level.to_lowercase().as_str() {
        "error" => tracing::Level::ERROR,
        "warn" => tracing::Level::WARN,
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        _ => tracing::Level::INFO,
    };

    let log_file_dir = var("LOG_FILE").unwrap_or("".to_string());
    let (non_blocking, guard) = if log_file_dir.is_empty() {
        tracing_appender::non_blocking(std::io::stdout())
    } else {
        let file_appender = tracing_appender::rolling::daily(".log", log_file_dir);
        tracing_appender::non_blocking(file_appender)
    };

    if common::string_to_bool(
        var("LOG_IS_JSON")
            .unwrap_or("false".to_string())
            .to_lowercase(),
    )
    .unwrap_or_default()
    {
        tracing_subscriber::fmt()
            .json()
            .with_span_list(false)
            .with_file(with_file)
            .with_line_number(with_line_number)
            .with_target(with_target)
            .with_max_level(log_level)
            .with_writer(non_blocking)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_file(with_file)
            .with_line_number(with_line_number)
            .with_target(with_target)
            .with_max_level(log_level)
            .with_writer(non_blocking)
            .init();
    }

    tracing::debug!("Start tracing on {} level", log_level);
    guard
}
