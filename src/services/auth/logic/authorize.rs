use super::factory::Logic;
use crate::cores::{
    auth::Role,
    error::{service::Error, types::AuthError},
};
use crate::services::auth::model::entity::User;

pub async fn execute(
    factory: &impl Logic,
    token_opt: Option<String>,
    valid_permission: Role,
) -> Result<User, Error> {
    let token = token_opt.ok_or_else(|| {
        tracing::error!("{}", AuthError::TokenNotProvided);
        Error::unauth_from(AuthError::TokenNotProvided)
    })?;

    let user = factory.token_validation(&token).await?;

    if !factory.is_permitted(valid_permission, user.role) {
        tracing::error!(
            "{}",
            AuthError::NotPermitted(format!("{}({})", user.username, user.id.to_string()))
        );
        return Err(Error::unauth_from(AuthError::NotPermitted(user.username)));
    }

    Ok(user)
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::{
        cores::{auth::Role, error::service::Error},
        services::auth::{
            logic::{authorize::execute, factory::MockLogic},
            model::entity::User,
        },
        utils::test::{test_actor, test_time, test_uuid},
    };

    fn test_mock_factory() -> MockLogic {
        let mut mock_factory = MockLogic::new();
        mock_factory
            .expect_token_validation()
            .with(eq("test"))
            .once()
            .returning(move |token| {
                let token = token.to_owned();
                Box::pin(async {
                    Ok(User {
                        id: test_uuid(),
                        username: token,
                        role: Role::Admin,
                        business: None,
                        create_time: test_time(),
                        create_by: test_actor(),
                        update_time: test_time(),
                        update_by: test_actor(),
                    })
                })
            });

        mock_factory
    }

    #[tokio::test]
    async fn fail_not_provided() -> Result<(), Error> {
        let res = execute(&MockLogic::new(), None, Role::Admin).await;

        let err = res.unwrap_err();
        assert_eq!(
            err,
            Error::Unauthorized("Auth Token not provided".to_owned())
        );
        Ok(())
    }

    #[tokio::test]
    async fn fail_not_permitted() -> Result<(), Error> {
        let mut mock = test_mock_factory();

        mock.expect_is_permitted()
            .with(eq(Role::Admin), eq(Role::Admin))
            .once()
            .returning(move |_, _| false);

        let res = execute(&mock, Some("test".to_owned()), Role::Admin).await;

        let err = res.unwrap_err();
        assert_eq!(
            err,
            Error::Unauthorized("User test not permitted".to_owned())
        );
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let mut mock = test_mock_factory();

        mock.expect_is_permitted()
            .with(eq(Role::Admin), eq(Role::Admin))
            .once()
            .returning(move |_, _| true);

        let res = execute(&mock, Some("test".to_owned()), Role::Admin).await;

        let return_result = res?;
        assert_eq!(return_result.id, test_uuid());
        assert_eq!(return_result.username, "test".to_owned());
        assert_eq!(return_result.role, Role::Admin);
        assert_eq!(return_result.create_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        assert_eq!(return_result.update_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        Ok(())
    }
}
