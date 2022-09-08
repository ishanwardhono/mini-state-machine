use crate::{
    cores::error::Error,
    services::state::{model::request::StateCreateRequest, repo::db::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, req: StateCreateRequest) -> Result<bool, Error> {
    validate(&req)?;
    repo.insert(req).await
}

fn validate(req: &StateCreateRequest) -> Result<(), Error> {
    if req.code == "" {
        return Err(Error::BadRequest("Code is empty".to_string()));
    }
    Ok(())
}
