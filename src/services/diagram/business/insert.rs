use crate::cores::error::service::Error;

pub async fn execute() -> Result<(), Error> {
    tracing::debug!("executing ...");
    Ok(())
}
