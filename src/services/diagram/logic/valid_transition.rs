use std::sync::Arc;

use crate::{cores::error::service::Error, services::diagram::repo::db::DbRepo, utils::validation};

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    code: &'a String,
    from: &'a String,
    to: &'a String,
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(code, from, to)?;
    let mut diagram = repo.get(&code).await?;

    //call remove bcs we take ownership
    let from_state = diagram.flows.remove(from).ok_or(Error::BadRequest(format!(
        "From State {} not found in diagram",
        from
    )))?;

    let transition = from_state
        .transitions
        .ok_or(Error::BadRequest(format!("State {} is final state", from)))?;

    if !transition.contains(to) {
        return Err(Error::BadRequest(format!(
            "Transition invalid from {} to {}",
            from, to
        )));
    }

    Ok(())
}

fn validate(code: &String, from: &String, to: &String) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if code.is_empty() {
        validation.add_str("Code is empty");
    }
    if from.is_empty() {
        validation.add_str("From State is empty");
    }
    if to.is_empty() {
        validation.add_str("To State is empty");
    }
    validation.check()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use mockall::predicate::eq;

    use super::*;
    use crate::services::diagram::{
        model::model::{Diagram, FlowModel},
        repo::db::MockDbRepo,
    };

    #[tokio::test]
    async fn fail_validate() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let res = execute(
            Arc::new(mock_db_repo),
            &String::from(""),
            &String::from(""),
            &String::from(""),
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty, From State is empty, To State is empty".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn fail_from_state_not_found() -> Result<(), Error> {
        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get()
            .with(eq(String::from("BUSINESS_CODE_TEST")))
            .once()
            .returning(move |_| {
                Box::pin(async {
                    Ok(Diagram {
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
                    })
                })
            });

        let res = execute(
            Arc::new(mock_db_repo),
            &String::from("BUSINESS_CODE_TEST"),
            &String::from("test"),
            &String::from("test"),
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("From State test not found in diagram".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn fail_from_state_no_transition() -> Result<(), Error> {
        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get()
            .with(eq(String::from("BUSINESS_CODE_TEST")))
            .once()
            .returning(move |_| {
                Box::pin(async {
                    Ok(Diagram {
                        code: String::from("BUSINESS_CODE_TEST"),
                        description: Some(String::from("")),
                        is_active: true,
                        flows: HashMap::from([
                            (
                                String::from("TEST_STATE"),
                                FlowModel {
                                    is_initial_state: true,
                                    transitions: None,
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
                    })
                })
            });

        let res = execute(
            Arc::new(mock_db_repo),
            &String::from("BUSINESS_CODE_TEST"),
            &String::from("TEST_STATE"),
            &String::from("test"),
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("State TEST_STATE is final state".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn fail_from_state_transition_not_found() -> Result<(), Error> {
        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get()
            .with(eq(String::from("BUSINESS_CODE_TEST")))
            .once()
            .returning(move |_| {
                Box::pin(async {
                    Ok(Diagram {
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
                    })
                })
            });

        let res = execute(
            Arc::new(mock_db_repo),
            &String::from("BUSINESS_CODE_TEST"),
            &String::from("TEST_STATE"),
            &String::from("TEST_STATE_2"),
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Transition invalid from TEST_STATE to TEST_STATE_2".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get()
            .with(eq(String::from("BUSINESS_CODE_TEST")))
            .once()
            .returning(move |_| {
                Box::pin(async {
                    Ok(Diagram {
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
                    })
                })
            });

        execute(
            Arc::new(mock_db_repo),
            &String::from("BUSINESS_CODE_TEST"),
            &String::from("TEST_STATE"),
            &String::from("TEST_STATE_1"),
        )
        .await
    }
}
