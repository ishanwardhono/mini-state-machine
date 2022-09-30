use crate::{
    cores::error::service::Error,
    services::state::{
        model::{entity::State, request::StateUpdateRequest},
        repo::db::DbRepo,
    },
    utils::validation,
};
use std::sync::Arc;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    code: &'a String,
    state: &'a StateUpdateRequest,
) -> Result<State, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.update(code, state).await
}

fn validate(req: &String) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if req == "" {
        validation.add("Code is empty");
    }

    validation.check()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate::eq;

    use crate::{
        cores::{database::pg::db_time_now, error::service::Error},
        services::state::{
            business::update::execute,
            model::{entity::State, request::StateUpdateRequest},
            repo::db::MockDbRepo,
        },
    };

    #[tokio::test]
    async fn validation_error_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();
        let req_code = String::from("");
        let req = StateUpdateRequest {
            description: None,
            webhooks: None,
        };

        let res = execute(Arc::new(mock_db_repo), &req_code, &req).await;

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
        let req_code = String::from("TEST");
        let req = StateUpdateRequest {
            description: None,
            webhooks: None,
        };

        mock_db_repo
            .expect_update()
            .with(eq(req_code.clone()), eq(req.clone()))
            .once()
            .returning(move |code, req| {
                let cloned_code = code.clone();
                let cloned_req = req.clone();
                Box::pin(async {
                    Ok(State {
                        id: 1,
                        code: cloned_code,
                        description: cloned_req.description,
                        webhooks: cloned_req.webhooks,
                        create_time: db_time_now(),
                        update_time: db_time_now(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &req_code, &req).await;

        let return_result = res?.clone();
        assert_eq!(return_result.id, 1);
        assert_eq!(return_result.code, req_code);
        assert_eq!(return_result.description, req.description);
        assert_eq!(return_result.webhooks, req.webhooks);
        assert_ne!(return_result.create_time, chrono::NaiveDateTime::MIN);
        assert_ne!(return_result.update_time, chrono::NaiveDateTime::MIN);
        Ok(())
    }
}
