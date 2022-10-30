mod delete;
pub mod factory;
mod get_all;
mod get_by_code;
mod get_codes;
mod helper;
mod insert;
mod update;

use self::factory::{Factory, Logic};
use super::repo::db::DbRepo;
use crate::services::client::logic::factory as ClientFactory;
use std::sync::Arc;

pub fn new(repo: Arc<dyn DbRepo>, client_logic: Arc<dyn ClientFactory::Logic>) -> Arc<dyn Logic> {
    Arc::new(Factory { repo, client_logic })
}
