use crate::{
    cores::error::service::Error,
    services::diagram::{model::model::Diagram, repo::db::DbRepo},
    utils::validation,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<Diagram, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.get(code).await
}

fn validate(code: &String) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if code.is_empty() {
        validation.add_str("Code is empty");
    }
    validation.check()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::services::diagram::repo::db::MockDbRepo;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn fail_validate_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();

        let res = execute(Arc::new(mock_db_repo), &String::from("")).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let req = String::from("TEST");
        let mut mock_db_repo = MockDbRepo::new();

        mock_db_repo
            .expect_get()
            .with(eq(req.clone()))
            .once()
            .returning(move |_| {
                Box::pin(async {
                    Ok(Diagram {
                        code: String::from("TEST"),
                        description: Some(String::from("test")),
                        is_active: true,
                        flows: HashMap::new(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &req).await;

        let return_result = res?;
        assert_eq!(return_result.code, req);
        assert_eq!(return_result.description, Some(String::from("test")));
        assert_eq!(return_result.is_active, true);
        Ok(())
    }
}
