use super::factory::Business;
use crate::cores::{
    auth::role::Role,
    error::{service::Error, types::AuthError},
};
use crate::services::auth::model::entity::User;

pub async fn execute(
    factory: &impl Business,
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
