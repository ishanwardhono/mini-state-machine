use std::sync::Arc;

use crate::{cores::error::service::Error, services::state::repo::db::DbRepo, utils::validation};

pub async fn execute(repo: Arc<dyn DbRepo>, codes: &Vec<String>) -> Result<Vec<String>, Error> {
    tracing::debug!("executing ...");
    validate(codes)?;
    repo.get_by_codes(codes).await
}

fn validate(codes: &Vec<String>) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if codes.len() <= 0 {
        validation.add_str("Code is empty");
    }
    if codes.contains(&String::new()) {
        validation.add_str("Invalid Code (Empty string)");
    }
    validation.check()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::state::repo::db::MockDbRepo;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn fail_codes_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let res = execute(Arc::new(mock_db_repo), &vec![]).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn fail_codes_invalid() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let res = execute(Arc::new(mock_db_repo), &vec!["".to_owned()]).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Invalid Code (Empty string)".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let req = vec!["TEST_STATE_1".to_owned(), "TEST_STATE_2".to_owned()];
        let mut mock_db_repo = MockDbRepo::new();

        mock_db_repo
            .expect_get_by_codes()
            .with(eq(req.clone()))
            .once()
            .returning(move |_| {
                Box::pin(async { Ok(vec!["TEST_STATE_1".to_owned(), "TEST_STATE_2".to_owned()]) })
            });

        let res = execute(Arc::new(mock_db_repo), &req).await;

        let return_result = res?;
        assert_eq!(req, return_result);
        Ok(())
    }
}
