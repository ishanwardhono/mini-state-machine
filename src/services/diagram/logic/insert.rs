use std::{collections::HashSet, sync::Arc};

use uuid::Uuid;

use crate::{
    cores::error::service::Error,
    services::diagram::{
        model::{entity::Flow, model::Diagram},
        repo::db::DbRepo,
    },
    utils::validation,
};

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    diagram: &'a Diagram,
    actor: &'a Uuid,
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(&diagram)?;
    validate_state(&diagram.flows)?;
    repo.insert(diagram, actor).await
}

fn validate(req: &Diagram) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if req.business.code.is_empty() {
        validation.add_str("Business Code is empty");
    }

    validation.check()
}

fn validate_state(flows: &Vec<Flow>) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    let mut states = HashSet::new();
    let mut initial_state_flag = false;
    for flow in flows {
        if !states.insert(&flow.state) {
            validation.add(format!("Duplicate State {}", flow.state));
        }
        if flow.is_initial_state {
            initial_state_flag = true;
        }
    }
    if !initial_state_flag {
        validation.add_str("There is no initial state in Diagram");
    }

    for flow in flows {
        if flow.next_states.is_none() {
            continue;
        }
        flow.next_states
            .as_ref()
            .unwrap()
            .iter()
            .for_each(|next_state| {
                if states.insert(next_state) {
                    validation.add(format!(
                        "next_state {} on State {} not registered in diagram",
                        next_state, flow.state
                    ));
                }
            });
    }
    validation.check()?;

    Ok(())
}
