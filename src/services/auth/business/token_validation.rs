use crate::{
    cores::{
        env::ConfigJWT,
        error::{service::Error, types::AuthError},
    },
    services::auth::{
        model::entity::{Claim, User},
        repo::db::DbRepo,
    },
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::sync::Arc;

pub async fn execute(cfg: ConfigJWT, repo: Arc<dyn DbRepo>, token: &String) -> Result<User, Error> {
    tracing::debug!("authorizing...");

    let token_part: Vec<&str> = token.split(" ").collect();
    if token_part.len() != 2 {
        tracing::error!("{}", AuthError::InvalidFormat);
        return Err(Error::unauth_from(AuthError::InvalidFormat));
    }
    if token_part[0] != "Bearer" {
        tracing::error!("{}", AuthError::UnsupportedType);
        return Err(Error::unauth_from(AuthError::UnsupportedType));
    }

    let jwt_token = token_part[1];

    let token_data = decode::<Claim>(
        jwt_token,
        &DecodingKey::from_secret(cfg.secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    );
    let claim = token_data.map_err(|e| {
        tracing::error!("{}", e.to_string());
        Error::Unauthorized(e.to_string())
    })?;

    repo.get_by_username(&claim.claims.sub)
        .await
        .map_err(|e| match e {
            Error::NotFound(_) => {
                tracing::error!("{}", AuthError::InvalidUser(claim.claims.sub.clone()));
                Error::unauth_from(AuthError::InvalidUser(claim.claims.sub.clone()))
            }
            _ => e,
        })
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::{
            auth::role::Role,
            env::{Config, ConfigApp, ConfigJWT},
            error::service::Error,
        },
        services::auth::{
            business::token_validation::execute, model::entity::User, repo::db::MockDbRepo,
        },
        utils::test::{test_actor, test_time, test_uuid},
    };
    use mockall::predicate::eq;
    use std::sync::Arc;

    fn test_username() -> String {
        String::from("test")
    }

    fn test_config() -> Config {
        Config {
            app: ConfigApp {
                name: String::from("test app"),
                ..Config::app_default()
            },
            jwt: ConfigJWT {
                secret: String::from("test jwt secret"),
                audience: String::from("test jwt audience"),
                exp_dur: 0,
            },
            ..Config::default()
        }
    }

    fn test_mock_db_repo_username() -> MockDbRepo {
        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get_by_username()
            .with(eq(test_username()))
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
        mock_db_repo
    }

    #[tokio::test]
    async fn fail_invalid_format() -> Result<(), Error> {
        let token = String::from("");
        let expected_err_msg = String::from("Invalid Authorization Format");

        let res = execute(test_config().jwt, Arc::new(MockDbRepo::new()), &token).await;

        let err = res.unwrap_err();
        assert_eq!(err, Error::Unauthorized(expected_err_msg));
        Ok(())
    }

    #[tokio::test]
    async fn fail_unsupported_type() -> Result<(), Error> {
        let token = String::from("Basic token");
        let expected_err_msg = String::from("Unsupported Authorization Type");

        let res = execute(test_config().jwt, Arc::new(MockDbRepo::new()), &token).await;

        let err = res.unwrap_err();
        assert_eq!(err, Error::Unauthorized(expected_err_msg));
        Ok(())
    }

    #[tokio::test]
    async fn fail_auth() -> Result<(), Error> {
        let token = String::from("Bearer token");
        let expected_err_msg = String::from("InvalidToken");

        let res = execute(test_config().jwt, Arc::new(MockDbRepo::new()), &token).await;

        let err = res.unwrap_err();
        assert_eq!(err, Error::Unauthorized(expected_err_msg));
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let token = String::from("Bearer eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0IiwiZXhwIjozNTE2MjM5MDIyfQ.217sWHipmKzr7Wo5ofabgFednbguNBPJuxLy6yrZeYKIQqS0ISikiRz9Zzwv__62jADqmfA6ugygqE29wr8rjw");

        let res = execute(
            test_config().jwt,
            Arc::new(test_mock_db_repo_username()),
            &token,
        )
        .await;

        let return_result = res?;
        assert_eq!(return_result.id, test_uuid());
        assert_eq!(return_result.username, test_username());
        assert_eq!(return_result.role, Role::Admin);
        assert_eq!(return_result.create_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        assert_eq!(return_result.update_time, test_time());
        assert_eq!(return_result.create_by, test_actor());
        Ok(())
    }
}
