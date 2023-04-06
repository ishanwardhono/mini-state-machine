use crate::{
    cores::{env::Config, error::service::Error},
    services::auth::{model::entity::Claim, repo::db::DbRepo},
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::sync::Arc;

pub async fn execute(
    cfg: Arc<Config>,
    repo: Arc<dyn DbRepo>,
    username: &str,
) -> Result<String, Error> {
    tracing::debug!("executing...");

    let user = repo.get_by_username(username).await?;

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(cfg.jwt.exp_dur))
        .expect("valid timestamp")
        .timestamp();

    let current_timestamp = chrono::Utc::now().timestamp();

    let claims = Claim {
        sub: user.username,
        exp: expiration as usize,
        jti: Some(uuid::Uuid::new_v4().to_string()),
        nbf: Some(current_timestamp as usize),
        iat: Some(current_timestamp as usize),
        iss: Some(cfg.app.name.clone()),
        aud: Some(cfg.jwt.audience.clone()),
    };

    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(cfg.jwt.secret.as_bytes()),
    )
    .map_err(|e| Error::InternalError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::{
            auth::Role,
            env::{Config, ConfigApp, ConfigJWT},
            error::service::Error,
        },
        services::auth::{
            logic::login::execute,
            model::entity::{Claim, User},
            repo::db::MockDbRepo,
        },
        utils::test::{test_actor, test_time, test_uuid},
    };
    use chrono::{Duration, Utc};
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use mockall::predicate::eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let username = "test";

        let cfg = Config {
            app: ConfigApp {
                name: String::from("test app"),
                ..Config::app_default()
            },
            jwt: ConfigJWT {
                secret: String::from("test jwt secret"),
                audience: String::from("test jwt audience"),
                exp_dur: 7,
            },
            ..Config::default()
        };

        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo
            .expect_get_by_username()
            .with(eq(username.clone()))
            .once()
            .returning(move |username| {
                let username = username.to_owned();
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

        let init_time = Utc::now();
        let expected_valid_time = Duration::days(7);

        let res = execute(Arc::new(cfg.clone()), Arc::new(mock_db_repo), &username).await;
        let token = res?;
        let token_data = decode::<Claim>(
            &token,
            &DecodingKey::from_secret(cfg.jwt.secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );
        let claim = token_data.map_err(|e| {
            tracing::error!("{}", e.to_string());
            Error::Unauthorized(e.to_string())
        })?;

        assert_eq!(claim.claims.sub, username);
        assert_eq!(claim.claims.aud.unwrap(), cfg.jwt.audience);
        assert_eq!(claim.claims.iss.unwrap(), cfg.app.name);
        assert_eq!(
            true,
            (init_time
                .checked_add_signed(expected_valid_time)
                .unwrap()
                .timestamp() as usize)
                <= claim.claims.exp
        );
        assert_eq!(
            true,
            claim.claims.exp
                <= (init_time
                    .checked_add_signed(expected_valid_time)
                    .unwrap()
                    .timestamp() as usize)
        );

        Ok(())
    }
}
