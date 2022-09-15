use crate::utils::common;

pub fn init_log() {
    let env_log_level = std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string());

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

    if common::StringToBool(
        std::env::var("LOG_IS_JSON")
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
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_file(with_file)
            .with_line_number(with_line_number)
            .with_target(with_target)
            .with_max_level(log_level)
            .init();
    }

    tracing::debug!("Start tracing on {} level", log_level);
}
