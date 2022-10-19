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
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

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
        if flow.transitions.is_none() {
            continue;
        }
        flow.transitions
            .as_ref()
            .unwrap()
            .iter()
            .for_each(|transition| {
                if states_set.insert(transition) {
                    validation.add(format!(
                        "transition {} on State {} not registered in diagram",
                        transition, flow.state
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
