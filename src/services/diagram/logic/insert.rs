use crate::{
    cores::error::service::Error,
    services::{
        diagram::{
            model::model::{Diagram, FlowModel},
            repo::db::DbRepo,
        },
        state::StateServiceLogic,
    },
    utils::validation,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use uuid::Uuid;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    state_logic: Arc<StateServiceLogic>,
    diagram: &'a Diagram,
    actor: &'a Uuid,
) -> Result<String, Error> {
    tracing::debug!("executing ...");
    validate(&diagram)?;
    validate_state(state_logic, &diagram.flows).await?;
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
    state_logic: Arc<StateServiceLogic>,
    flows: &HashMap<String, FlowModel>,
) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    let mut states_set = HashSet::new();
    let mut initial_state_flag = false;
    let mut states = vec![];

    for (state, flow) in flows {
        if !states_set.insert(state) {
            validation.add(format!("Duplicate State {}", state));
        }
        if flow.is_initial_state {
            initial_state_flag = true;
        }
        states.push(state.to_string());
    }
    if !initial_state_flag {
        validation.add_str("Diagram has no initial state");
    }

    for (state, flow) in flows {
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
                        "transition {} on state {} not registered in diagram",
                        transition, state
                    ));
                }
            });
    }
    validation.check()?;

    let db_states = state_logic.get_codes(&states).await?;
    states.retain(|s| !db_states.contains(&s));
    if states.len() > 0 {
        validation.add(format!("States {} not found in database", states.join(",")));
    }
    validation.check()
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::{
        services::{diagram::repo::db::MockDbRepo, state::logic::factory::MockLogic},
        utils::test::test_uuid,
    };

    #[tokio::test]
    async fn fail_validation_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();
        let mock_state_logic = MockLogic::new();
        let diagram = Diagram {
            code: String::from(""),
            description: Some(String::from("")),
            is_active: true,
            flows: HashMap::from([(
                String::from("TEST_STATE"),
                FlowModel {
                    is_initial_state: true,
                    transitions: Some(vec!["TEST_STATE".to_owned()]),
                },
            )]),
        };

        let res = execute(
            Arc::new(mock_db_repo),
            Arc::new(mock_state_logic),
            &diagram,
            &test_uuid(),
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Business Code is empty".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn fail_validation_state() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();
        let mock_state_logic = MockLogic::new();
        let diagram = Diagram {
            code: String::from("BUSINESS_CODE_TEST"),
            description: Some(String::from("")),
            is_active: true,
            flows: HashMap::from([
                (
                    String::from("TEST_STATE"),
                    FlowModel {
                        is_initial_state: false,
                        transitions: Some(vec!["TEST_STATE_2".to_owned()]),
                    },
                ),
                (
                    String::from("TEST_STATE_1"),
                    FlowModel {
                        is_initial_state: false,
                        transitions: Some(vec!["TEST_STATE".to_owned()]),
                    },
                ),
            ]),
        };

        let res = execute(
            Arc::new(mock_db_repo),
            Arc::new(mock_state_logic),
            &diagram,
            &test_uuid(),
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Diagram has no initial state, transition TEST_STATE_2 on state TEST_STATE not registered in diagram".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn fail_validation_state_not_in_db() -> Result<(), Error> {
        let diagram = Diagram {
            code: String::from("BUSINESS_CODE_TEST"),
            description: Some(String::from("")),
            is_active: true,
            flows: HashMap::from([
                (
                    String::from("TEST_STATE"),
                    FlowModel {
                        is_initial_state: true,
                        transitions: Some(vec!["TEST_STATE_1".to_owned()]),
                    },
                ),
                (
                    String::from("TEST_STATE_1"),
                    FlowModel {
                        is_initial_state: false,
                        transitions: Some(vec!["TEST_STATE".to_owned()]),
                    },
                ),
            ]),
        };

        let mock_db_repo = MockDbRepo::new();
        let mut mock_state_logic = MockLogic::new();
        mock_state_logic
            .expect_get_codes()
            .withf(|transition| {
                let matcher = vec!["TEST_STATE".to_owned(), "TEST_STATE_1".to_owned()];
                transition.len() == matcher.len()
                    && matcher.iter().all(|state| transition.contains(state))
            })
            .once()
            .returning(move |_| Box::pin(async { Ok(vec!["TEST_STATE".to_owned()]) }));

        let res = execute(
            Arc::new(mock_db_repo),
            Arc::new(mock_state_logic),
            &diagram,
            &test_uuid(),
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("States TEST_STATE_1 not found in database".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let diagram = Diagram {
            code: String::from("BUSINESS_CODE_TEST"),
            description: Some(String::from("")),
            is_active: true,
            flows: HashMap::from([
                (
                    String::from("TEST_STATE"),
                    FlowModel {
                        is_initial_state: true,
                        transitions: Some(vec!["TEST_STATE_1".to_owned()]),
                    },
                ),
                (
                    String::from("TEST_STATE_1"),
                    FlowModel {
                        is_initial_state: false,
                        transitions: Some(vec!["TEST_STATE".to_owned()]),
                    },
                ),
            ]),
        };

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_insert()
            .with(eq(diagram.clone()), eq(test_uuid()))
            .once()
            .returning(move |_, _| Box::pin(async { Ok(String::from("BUSINESS_CODE_TEST")) }));

        let mut mock_state_logic = MockLogic::new();
        mock_state_logic
            .expect_get_codes()
            .withf(|transition| {
                let matcher = vec!["TEST_STATE".to_owned(), "TEST_STATE_1".to_owned()];
                transition.len() == matcher.len()
                    && matcher.iter().all(|state| transition.contains(state))
            })
            .once()
            .returning(move |_| {
                Box::pin(async { Ok(vec!["TEST_STATE".to_owned(), "TEST_STATE_1".to_owned()]) })
            });

        let res = execute(
            Arc::new(mock_db_repo),
            Arc::new(mock_state_logic),
            &diagram,
            &test_uuid(),
        )
        .await?;

        assert_eq!(res, String::from("BUSINESS_CODE_TEST"));
        Ok(())
    }
}
