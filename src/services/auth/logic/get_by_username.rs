use crate::{
    cores::error::service::Error,
    services::auth::{model::entity::User, repo::db::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, username: &String) -> Result<User, Error> {
    tracing::debug!("executing...");
    repo.get_by_username(username).await
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::{auth::role::Role, error::service::Error},
        services::auth::{
            logic::get_by_username::execute, model::entity::User, repo::db::MockDbRepo,
        },
        utils::test::{test_actor, test_time, test_uuid},
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let username = String::from("test");

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get_by_username()
            .with(eq(username.clone()))
            .once()
            .returning(move |username| {
                let username = username.clone();
                Box::pin(async {
                    Ok(User {
                        id: test_uuid(),
                        username,
                        role: Role::Admin,
                        create_time: test_time(),
                        create_by: test_actor(),
                        update_time: test_time(),
                        update_by: test_actor(),
                    })
                })
            });

        let res = execute(Arc::new(mock_db_repo), &username).await;

        let return_result = res?;
        assert_eq!(return_result.id, test_uuid());
        assert_eq!(return_result.username, username);
        assert_eq!(return_result.role, Role::Admin);
        assert_eq!(return_result.create_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        assert_eq!(return_result.update_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        Ok(())
    }
}
