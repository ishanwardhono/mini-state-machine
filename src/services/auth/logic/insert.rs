use crate::{
    cores::{auth::Role, error::service::Error},
    services::auth::{
        model::{entity::User, request::UserCreateRequest},
        repo::db::DbRepo,
    },
    utils::validation,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    req: &'a mut UserCreateRequest,
    actor: &'a Uuid,
) -> Result<User, Error> {
    tracing::debug!("executing...");
    validate(req)?;
    repo.insert(req, actor).await
}

fn validate(req: &mut UserCreateRequest) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if req.role == Role::BusinessClient && req.business.is_none() {
        validation.add_str("Business Name must be provided for BusinessClient");
    } else if req.role == Role::Admin {
        req.business = None;
    }
    validation.check()
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::{auth::Role, error::service::Error},
        services::auth::{
            logic::insert::execute,
            model::{entity::User, request::UserCreateRequest},
            repo::db::MockDbRepo,
        },
        utils::test::{test_actor, test_time, test_uuid},
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let mut req = UserCreateRequest {
            username: String::from("test"),
            role: Role::Admin,
            business: None,
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
                    Ok(User {
                        id: test_uuid(),
                        username: cloned_req.username,
                        role: Role::Admin,
                        business: None,
                        create_time: test_time(),
                        create_by: test_actor(),
                        update_time: test_time(),
                        update_by: test_actor(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &mut req, &actor).await;

        let return_result = res?;
        assert_eq!(return_result.id, test_uuid());
        assert_eq!(return_result.username, req.username);
        assert_eq!(return_result.role, req.role);
        assert_eq!(return_result.create_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        assert_eq!(return_result.update_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        Ok(())
    }
}
