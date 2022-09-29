use crate::{
    cores::error::service::Error,
    services::state::{model::entity::State, repo::db::DbRepo},
    utils::validation,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<State, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.get_by_code(code).await
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
    use super::*;
    use crate::{cores::database::pg::db_time_now, services::state::repo::db::MockDbRepo};
    use mockall::predicate::eq;

    #[tokio::test]
    async fn validation_error_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let res = execute(Arc::new(mock_db_repo), &String::from("")).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty".to_string()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let req = String::from("TEST");
        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get_by_code()
            .with(eq(req.clone()))
            .once()
            .returning(move |code| {
                let cloned_code = code.clone();
                Box::pin(async {
                    Ok(State {
                        id: 1,
                        code: cloned_code,
                        description: Some(String::from("test")),
                        webhooks: Some(vec![String::from("test_app")]),
                        create_time: db_time_now(),
                        update_time: db_time_now(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &req).await;

        let return_result = res?.clone();
        assert_eq!(return_result.id, 1);
        assert_eq!(return_result.code, req);
        assert_eq!(return_result.description, Some(String::from("test")));
        assert_eq!(return_result.webhooks.as_ref().unwrap().len(), 1);
        assert_eq!(
            return_result.webhooks.as_ref().unwrap()[0],
            String::from("test_app")
        );
        assert_ne!(return_result.create_time, chrono::NaiveDateTime::MIN);
        assert_ne!(return_result.update_time, chrono::NaiveDateTime::MIN);
        Ok(())
    }
}
