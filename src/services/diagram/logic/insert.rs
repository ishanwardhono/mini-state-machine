use std::sync::Arc;

use uuid::Uuid;

use crate::{
    cores::error::service::Error,
    services::diagram::{model::model::Diagram, repo::db::DbRepo},
};

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    diagram: &'a Diagram,
    actor: &'a Uuid,
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    repo.insert(diagram, actor).await
}
