use crate::{
    cores::error::service::Error,
    services::state::{
        model::{entity::State, request::StateCreateRequest},
        repo::db::DbRepo,
    },
    utils::validation,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, req: &StateCreateRequest) -> Result<State, Error> {
    tracing::debug!("executing ...");
    validate(&req)?;
    repo.insert(req).await
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
        cores::{database::pg::db_time_now, error::service::Error},
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
        let res = execute(Arc::new(mock_db_repo), &req).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty".to_string()),
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

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_insert()
            .with(eq(req.clone()))
            .once()
            .returning(move |req| {
                let cloned_req = req.clone();
                Box::pin(async {
                    Ok(State {
                        id: 1,
                        code: cloned_req.code,
                        description: cloned_req.description,
                        webhooks: cloned_req.webhooks,
                        create_time: db_time_now(),
                        update_time: db_time_now(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &req).await;

        let return_result = res?.clone();
        assert_eq!(return_result.id, 1);
        assert_eq!(return_result.code, req.code);
        assert_eq!(return_result.description, req.description);
        assert_eq!(return_result.webhooks, req.webhooks);
        assert_ne!(return_result.create_time, chrono::NaiveDateTime::MIN);
        assert_ne!(return_result.update_time, chrono::NaiveDateTime::MIN);
        Ok(())
    }
}
