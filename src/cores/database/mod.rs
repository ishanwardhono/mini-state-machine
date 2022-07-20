pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn set_db(database_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(
        database_url
    );
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}