use crate::{
    cores::{environment::Config, error::service::Error},
    services::auth::{model::entity::Claim, repo::db::DbRepo},
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::sync::Arc;

pub async fn execute(
    cfg: Arc<Config>,
    repo: Arc<dyn DbRepo>,
    username: &String,
) -> Result<String, Error> {
    tracing::debug!("executing...");

    let user = repo.get_by_username(username).await.unwrap();

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
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
