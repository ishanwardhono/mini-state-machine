use crate::{
    cores::error::service::Error,
    services::client::{model::model::ClientModel, repo::db::DbRepo},
    utils::validation,
};
use std::sync::Arc;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    req: &'a ClientModel,
    actor: &'a uuid::Uuid,
) -> Result<String, Error> {
    tracing::debug!("executing ...");
    validate(req)?;
    repo.update(req, actor).await
}

fn validate(req: &ClientModel) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if req.code.is_empty() {
        validation.add_str("Code is empty");
    }
    if req.url.is_empty() {
        validation.add_str("URL is empty");
    }

    validation.check()
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::error::service::Error,
        services::client::{
            logic::update::execute, model::model::ClientModel, repo::db::MockDbRepo,
        },
        utils::test::test_actor,
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn fail_validate_code_empty() -> Result<(), Error> {
        let mock_db_repo = MockDbRepo::new();
        let req = ClientModel {
            code: String::from(""),
            url: String::from(""),
            auth_token: Some(String::from("")),
        };
        let actor = uuid::Uuid::new_v4();

        let res = execute(Arc::new(mock_db_repo), &req, &actor).await;

        assert!(res.is_err());
        assert_eq!(
            Error::BadRequest("Code is empty, URL is empty".to_owned()),
            res.unwrap_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let mut mock_db_repo = MockDbRepo::new();
        let req = ClientModel {
            code: String::from("TEST"),
            url: String::from("TEST"),
            auth_token: Some(String::from("TEST")),
        };
        let actor = test_actor();

        mock_db_repo
            .expect_update()
            .with(eq(req.clone()), eq(actor.clone()))
            .once()
            .returning(move |_, _| Box::pin(async { Ok(String::from("TEST")) }));

        let res = execute(Arc::new(mock_db_repo), &req, &actor).await;

        let return_result = res?;
        assert_eq!(return_result, String::from("TEST"));
        Ok(())
    }
}
