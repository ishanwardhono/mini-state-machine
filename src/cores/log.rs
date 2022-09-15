pub fn init_log() {
    let env_log_level = std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string());

    let log_level = match env_log_level.to_lowercase().as_str() {
        "error" => tracing::Level::ERROR,
        "warn" => tracing::Level::WARN,
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        _ => tracing::Level::INFO,
    };

    tracing_subscriber::fmt()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(log_level)
        .with_span_list(false)
        .init();

    tracing::debug!("Start tracing on {} level", log_level);
}
