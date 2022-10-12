use crate::{
    cores::error::service::Error,
    services::state::{
        model::{entity::State, request::StateCreateRequest},
        repo::db::DbRepo,
    },
    utils::validation,
};
use std::sync::Arc;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    req: &'a StateCreateRequest,
    actor: &'a uuid::Uuid,
) -> Result<State, Error> {
    tracing::debug!("executing ...");
    validate(&req)?;
    repo.insert(req, actor).await
}

fn validate(req: &StateCreateRequest) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if req.code == "" {
        validation.add("Code is empty");
    }

    validation.check()
}

#[cfg(test)]

mod tests {
    use crate::{
        cores::{
            error::service::Error,
            test::{test_actor, test_time, test_uuid},
        },
        services::state::{
            business::insert::execute,
            model::{entity::State, request::StateCreateRequest},
            repo::db::MockDbRepo,
        },
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn validation_error_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let req = StateCreateRequest {
            code: String::from(""),
            description: None,
            webhooks: None,
        };
        let actor = uuid::Uuid::new_v4();

        let res = execute(Arc::new(mock_db_repo), &req, &actor).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let req = StateCreateRequest {
            code: String::from("TEST"),
            description: None,
            webhooks: None,
        };
        let actor = test_actor();

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_insert()
            .with(eq(req.clone()), eq(actor.clone()))
            .once()
            .returning(move |req, _| {
                let cloned_req = req.clone();
                Box::pin(async {
                    Ok(State {
                        id: test_uuid(),
                        code: cloned_req.code,
                        description: cloned_req.description,
                        webhooks: cloned_req.webhooks,
                        create_time: test_time(),
                        create_by: test_actor(),
                        update_time: test_time(),
                        update_by: test_actor(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &req, &actor).await;

        let return_result = res?;
        assert_eq!(return_result.id, test_uuid());
        assert_eq!(return_result.code, req.code);
        assert_eq!(return_result.description, req.description);
        assert_eq!(return_result.webhooks, req.webhooks);
        assert_eq!(return_result.create_time, test_time());
        assert_eq!(return_result.create_by, actor);
        assert_eq!(return_result.update_time, test_time());
        assert_eq!(return_result.create_by, actor);
        Ok(())
    }
}
