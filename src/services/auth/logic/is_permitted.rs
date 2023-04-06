use crate::cores::auth::Role;

pub fn execute(valid_permission: Role, user_permission: Role) -> bool {
    if user_permission.level() <= valid_permission.level() {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{
        cores::{auth::Role, error::service::Error},
        services::auth::logic::is_permitted::execute,
    };

    #[tokio::test]
    async fn fail_not_permitted() -> Result<(), Error> {
        let valid_permission = Role::Admin;
        let user_permission = Role::BusinessClient;

        let is_permitted = execute(valid_permission, user_permission);

        assert!(!is_permitted);
        Ok(())
    }

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let valid_permission = Role::BusinessClient;
        let user_permission = Role::Admin;

        let is_permitted = execute(valid_permission, user_permission);

        assert!(is_permitted);
        Ok(())
    }
}
