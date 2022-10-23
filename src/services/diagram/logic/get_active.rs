use super::factory::OperationLogic;
use crate::{cores::error::service::Error, services::diagram::model::model::Diagram};

pub async fn execute(factory: &impl OperationLogic, code: &str) -> Result<Diagram, Error> {
    tracing::debug!("executing ...");
    let diagram = factory.get(code).await?;
    if !diagram.is_active {
        return Err(Error::BadRequest(format!(
            "Business {} is not active",
            code
        )));
    }
    Ok(diagram)
}
