use crate::{
    cores::error::service::Error, services::client::logic::factory as ClientFactory,
    utils::common::VecExt,
};
use std::{collections::HashSet, sync::Arc};

pub async fn validate_client_exists(
    client_logic: Arc<dyn ClientFactory::Logic>,
    clients: Option<Vec<String>>,
) -> Result<(), Error> {
    if clients.is_none() {
        return Ok(());
    }

    let clients = clients.unwrap();
    if clients.is_empty() {
        return Ok(());
    }

    let mut distinct_clients = HashSet::new();
    for client in &clients {
        if !distinct_clients.insert(client.clone()) {
            return Err(Error::BadRequest(format!(
                "Duplicate client {} on actions",
                client
            )));
        }
    }

    let db_clients = client_logic
        .get_codes(&distinct_clients.into_iter().collect())
        .await?;
    let invalid_clients = clients.uncontain(db_clients);
    if invalid_clients.len() > 0 {
        Err(Error::BadRequest(format!(
            "Action Client {} not found",
            invalid_clients.join(", ")
        )))
    } else {
        Ok(())
    }
}
