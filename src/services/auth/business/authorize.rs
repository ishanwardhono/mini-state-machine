use super::factory::Business;
use crate::cores::{auth::role::Role, error::service::Error};

pub async fn execute(
    factory: &impl Business,
    token_opt: Option<String>,
    valid_permission: Role,
) -> Result<i32, Error> {
    let token = token_opt.ok_or_else(|| {
        tracing::error!("Auth Token not provided");
        Error::Unauthorized("Auth Token not provided".to_string())
    })?;

    let user = factory.token_validation(&token).await.map_err(|e| {
        tracing::error!("{}", e.to_message_display());
        e
    })?;

    if !factory.check_permission(valid_permission, user.role) {
        tracing::error!(
            "User {} ({}) has no permission for action",
            user.username,
            user.id
        );
        return Err(Error::Unauthorized(
            "User has no permission for action".to_string(),
        ));
    }

    Ok(user.id)
}
