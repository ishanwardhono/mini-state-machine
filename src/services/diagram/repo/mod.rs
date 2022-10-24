pub mod db;
mod db_query;

use self::db::{DbRepo, DbRepository};
use crate::cores::database::pg::DbPool;
use std::sync::Arc;

pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
    Arc::new(DbRepository { pool })
}
