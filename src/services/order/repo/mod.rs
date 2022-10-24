pub mod db;
mod db_query;

use crate::cores::database::pg::DbPool;
use std::sync::Arc;

pub fn new(pool: Arc<DbPool>) -> Arc<dyn db::DbRepo> {
    Arc::new(db::DbRepository { pool })
}
