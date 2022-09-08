use crate::{
    cores::error::Error,
    services::state::{model::request::StateRequest, repo::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, req: StateRequest) -> Result<bool, Error> {
    validate(&req)?;
    repo.insert(req).await
}

fn validate(req: &StateRequest) -> Result<(), Error> {
    if req.code == "" {
        return Err(Error::BadRequest("Code is empty".to_string()));
    }
    Ok(())
}
