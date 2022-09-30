use crate::{
    cores::error::{service::Error, types::AuthError},
    services::auth::{
        model::entity::{Claim, User},
        repo::db::DbRepo,
    },
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, token: &String) -> Result<User, Error> {
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
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap_or_default().as_bytes()),
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
