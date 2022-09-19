use chrono::{NaiveDateTime, Utc};
use sqlx::pool::Pool;
use sqlx::postgres::{PgArguments, PgConnectOptions, PgPoolOptions};
use sqlx::query::Query;
use sqlx::{ConnectOptions, Postgres};

pub type DbPool = Pool<Postgres>;
pub type DbQueryArguments = Query<'static, Postgres, PgArguments>;

pub async fn init() -> DbPool {
    let mut options = PgConnectOptions::new()
        .host(&std::env::var("DB_HOST").expect("DB_HOST must be set"))
        .port(
            std::env::var("DB_PORT")
                .expect("DB_PORT must be set")
                .parse::<u16>()
                .unwrap(),
        )
        .database(&std::env::var("DB_NAME").expect("DB_NAME must be set"));

    if let Ok(user) = std::env::var("DB_USER") {
        options = options.username(&user)
    }
    if let Ok(password) = std::env::var("DB_PASS") {
        options = options.password(&password)
    }
    options.disable_statement_logging();

    tracing::info!("Database initialization");

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to create pool")
}

pub fn db_time_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}
