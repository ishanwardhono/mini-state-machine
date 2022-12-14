use crate::{
    cores::error::service::Error,
    services::{
        client::ClientServiceLogic,
        state::{
            logic::helper,
            model::{entity::State, request::StateUpdateRequest},
            repo::db::DbRepo,
        },
    },
    utils::validation,
};
use std::sync::Arc;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    client_logic: Arc<ClientServiceLogic>,
    code: &'a str,
    state: &'a StateUpdateRequest,
    actor: &'a uuid::Uuid,
) -> Result<State, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    helper::validate_client_exists(client_logic, state.actions.clone()).await?;
    repo.update(code, state, actor).await
}

fn validate(req: &str) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if req == "" {
        validation.add_str("Code is empty");
    }
    validation.check()
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::error::service::Error,
        services::{
            client::logic::factory::MockLogic,
            state::{
                logic::update::execute,
                model::{entity::State, request::StateUpdateRequest},
                repo::db::MockDbRepo,
            },
        },
        utils::test::{test_actor, test_time, test_uuid},
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn fail_validate_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();
        let mock_client_logic = MockLogic::new();

        let req_code = String::from("");
        let req = StateUpdateRequest {
            description: None,
            actions: None,
        };
        let actor = uuid::Uuid::new_v4();

        let res = execute(
            Arc::new(mock_db_repo),
            Arc::new(mock_client_logic),
            &req_code,
            &req,
            &actor,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let mut mock_db_repo = MockDbRepo::new();
        let mock_client_logic = MockLogic::new();

        let req_code = "TEST";
        let req = StateUpdateRequest {
            description: None,
            actions: None,
        };
        let actor = test_actor();

        mock_db_repo
            .expect_update()
            .with(eq(req_code.clone()), eq(req.clone()), eq(actor.clone()))
            .once()
            .returning(move |code, req, _| {
                let cloned_code = code.to_string();
                let cloned_req = req.clone();
                Box::pin(async {
                    Ok(State {
                        id: test_uuid(),
                        code: cloned_code,
                        description: cloned_req.description,
                        actions: cloned_req.actions,
                        create_time: test_time(),
                        create_by: test_actor(),
                        update_time: test_time(),
                        update_by: test_actor(),
                    })
                })
            });

        let res = execute(
            Arc::new(mock_db_repo),
            Arc::new(mock_client_logic),
            &req_code,
            &req,
            &actor,
        )
        .await;

        let return_result = res?;
        assert_eq!(return_result.id, test_uuid());
        assert_eq!(return_result.code, req_code);
        assert_eq!(return_result.description, req.description);
        assert_eq!(return_result.actions, req.actions);
        assert_eq!(return_result.create_time, test_time());
        assert_eq!(return_result.create_by, actor);
        assert_eq!(return_result.update_time, test_time());
        assert_eq!(return_result.create_by, actor);
        Ok(())
    }
}
