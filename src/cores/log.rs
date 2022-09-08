use std::io::Write;

use chrono::Utc;

pub fn init_log() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("LOG_LEVEL").unwrap_or("Info".to_string()),
    );
    std::env::set_var("RUST_BACKTRACE", "6");
    env_logger::builder()
        .format(|buf, record| {
            let filename = record.file().unwrap_or_default();
            if filename.contains(&".cargo") {
                writeln!(
                    buf,
                    "{}: {}: {}",
                    Utc::now().to_rfc3339().trim_end_matches("+00:00"),
                    record.level(),
                    record.args().to_string()
                )
            } else {
                writeln!(
                    buf,
                    "{}: {}: {} {}",
                    Utc::now().to_rfc3339().trim_end_matches("+00:00"),
                    record.level(),
                    record.args(),
                    format!("({}:{})", filename, record.line().unwrap_or_default()).to_string()
                )
            }
        })
        .init();
}
