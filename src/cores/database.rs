use crate::cores::env::ConfigDatabase;
use chrono::{NaiveDateTime, Utc};
use sqlx::pool::Pool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn init(cfg: ConfigDatabase) -> DbPool {
    let mut options = PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .database(&cfg.name);

    if !cfg.user.is_empty() {
        options = options.username(&cfg.user)
    }
    if !cfg.pass.is_empty() {
        options = options.password(&cfg.pass)
    }
    options.disable_statement_logging();

    tracing::info!("Database initialization");

    PgPoolOptions::new()
        .max_connections(cfg.max_pool)
        .connect_with(options)
        .await
        .expect("Failed to create pool")
}

pub fn db_time_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}
