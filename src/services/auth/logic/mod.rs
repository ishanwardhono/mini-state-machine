mod authorize;
pub mod factory;
mod get_by_username;
mod insert;
mod is_permitted;
mod login;
mod token_validation;

use self::factory::{Factory, Logic};
use super::repo::db::DbRepo;
use crate::cores::env::Config;
use std::sync::Arc;

pub fn new(cfg: Arc<Config>, repo: Arc<dyn DbRepo>) -> Arc<dyn Logic> {
    Arc::new(Factory { cfg, repo })
}
