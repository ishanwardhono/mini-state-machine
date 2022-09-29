use crate::cores::auth::role::Role;

pub fn execute(valid_permission: Role, user_permission: Role) -> bool {
    if user_permission.level() <= valid_permission.level() {
        return true;
    }
    false
}
