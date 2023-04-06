use crate::{
    cores::error::service::Error,
    services::client::{model::model::ClientModel, repo::db::DbRepo},
    utils::validation,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &str) -> Result<ClientModel, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.get_by_code(code).await
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
    use super::*;
    use crate::services::client::repo::db::MockDbRepo;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn fail_validate_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let res = execute(Arc::new(mock_db_repo), "").await;

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
            .expect_get_by_code()
            .with(eq(req.clone()))
            .once()
            .returning(move |_| {
                Box::pin(async {
                    Ok(ClientModel {
                        code: String::from("TEST"),
                        url: String::from("TEST"),
                        auth_token: Some(String::from("TEST")),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &req).await;

        let return_result = res?;
        assert_eq!(return_result.code, req);
        assert_eq!(return_result.url, String::from("TEST"));
        Ok(())
    }
}
