use std::{collections::HashSet, sync::Arc};

use uuid::Uuid;

use crate::{
    cores::error::service::Error,
    services::{
        diagram::{
            model::model::{Diagram, FlowModel},
            repo::db::DbRepo,
        },
        state::logic::factory as StateFactory,
    },
    utils::validation,
};

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    state_factory: Arc<dyn StateFactory::Logic>,
    diagram: &'a Diagram,
    actor: &'a Uuid,
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(&diagram)?;
    validate_state(state_factory, &diagram.flows).await?;
    repo.insert(diagram, actor).await
}

fn validate(diagram: &Diagram) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if diagram.code.is_empty() {
        validation.add_str("Business Code is empty");
    }
    if diagram.flows.len() <= 0 {
        validation.add_str("State flows is empty");
    }

    validation.check()
}

async fn validate_state(
    state_factory: Arc<dyn StateFactory::Logic>,
    flows: &Vec<FlowModel>,
) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    let mut states_set = HashSet::new();
    let mut initial_state_flag = false;
    let mut states = vec![];

    for flow in flows {
        if !states_set.insert(&flow.state) {
            validation.add(format!("Duplicate State {}", flow.state));
        }
        if flow.is_initial_state {
            initial_state_flag = true;
        }
        states.push(flow.state.to_string());
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
                if states_set.insert(next_state) {
                    validation.add(format!(
                        "next_state {} on State {} not registered in diagram",
                        next_state, flow.state
                    ));
                }
            });
    }
    validation.check()?;

    let db_states = state_factory.get_by_codes(&states).await?;
    states.retain(|s| !db_states.contains(&s));
    if states.len() > 0 {
        validation.add(format!("States {} not found in database", states.join(",")));
    }
    validation.check()
}
