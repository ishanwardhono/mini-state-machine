pub fn init_log() {
    tracing_subscriber::fmt()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
