use crate::{cores::error::service::Error, services::diagram::repo::db::DbRepo, utils::validation};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &str) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.delete(code).await
}

fn validate(code: &str) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if code.is_empty() {
        validation.add_str("Code is empty");
    }
    validation.check()
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::services::diagram::repo::db::MockDbRepo;

    #[tokio::test]
    async fn fail_validate_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let req = String::from("");
        let res = execute(Arc::new(mock_db_repo), &req).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty".to_owned()),
            res.unwrap_err()
        );

        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let req = "TEST";

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_delete()
            .with(eq(req.clone()))
            .once()
            .returning(move |_| Box::pin(async { Ok(()) }));

        execute(Arc::new(mock_db_repo), &req).await
    }
}
