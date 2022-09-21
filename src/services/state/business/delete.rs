use std::sync::Arc;

use crate::{cores::error::Error, services::state::repo::db::DbRepo, utils::validation};

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<String, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.delete(code).await
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
    use crate::{
        cores::error::Error,
        services::state::{business::delete::execute, repo::db::MockDbRepo},
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn validation_error_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let req = String::from("");
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
        let req = String::from("TEST");

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_delete()
            .with(eq(req.clone()))
            .once()
            .returning(move |req| {
                let cloned_req = req.clone();
                Box::pin(async { Ok(cloned_req) })
            });

        let res = execute(Arc::new(mock_db_repo), &req).await;

        let return_result = res?.clone();
        assert_eq!(return_result, req);
        Ok(())
    }
}
