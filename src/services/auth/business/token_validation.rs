use crate::{
    cores::error::Error,
    services::auth::{model::entity::Claim, repo::db::DbRepo},
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, token: &String) -> Result<i32, Error> {
    tracing::debug!("executing...");

    let token_data = decode::<Claim>(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap_or_default().as_bytes()),
        &Validation::new(Algorithm::HS512),
    );
    let claim = token_data.map_err(|e| Error::Unauthorized(e.to_string()))?;

    let user = repo.get_by_username(&claim.claims.sub).await?;

    Ok(user.id)
}
