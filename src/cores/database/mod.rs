use chrono::{NaiveDateTime, Utc};
use sqlx::pool::Pool;
use sqlx::postgres::{PgArguments, PgPoolOptions};
use sqlx::query::Query;
use sqlx::Postgres;

pub type DbPool = Pool<Postgres>;
pub type DbQueryArguments = Query<'static, Postgres, PgArguments>;

pub async fn set_db(database_url: String) -> DbPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub fn db_time_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}
