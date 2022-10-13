use crate::{
    cores::error::service::Error,
    services::auth::{
        model::{entity::User, request::UserCreateRequest},
        repo::db::DbRepo,
    },
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, req: &UserCreateRequest) -> Result<User, Error> {
    tracing::debug!("executing...");
    repo.insert(req).await
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::{auth::role::Role, error::service::Error},
        services::auth::{
            business::insert::execute,
            model::{entity::User, request::UserCreateRequest},
            repo::db::MockDbRepo,
        },
        utils::test::{test_actor, test_time, test_uuid},
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let req = UserCreateRequest {
            username: String::from("test"),
            role: Role::Admin,
        };

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_insert()
            .with(eq(req.clone()))
            .once()
            .returning(move |req| {
                let cloned_req = req.clone();
                Box::pin(async {
                    Ok(User {
                        id: test_uuid(),
                        username: cloned_req.username,
                        role: Role::Admin,
                        create_time: test_time(),
                        create_by: test_actor(),
                        update_time: test_time(),
                        update_by: test_actor(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &req).await;

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
