use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn set_db() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    );
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}