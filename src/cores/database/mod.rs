use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

pub type DbPool = Pool<Postgres>;

pub async fn set_db(database_url: String) -> DbPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
