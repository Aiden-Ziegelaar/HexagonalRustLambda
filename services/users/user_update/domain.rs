use models::models::user::{user_update_by_email, MutableUser, User};

pub async fn user_update_core(who: MutableUser) -> Result<User, &'static str> {
    user_update_by_email(who)
        .await
        .map_err(|_| "Unable to update user")
}
