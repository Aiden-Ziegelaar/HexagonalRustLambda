use models::models::user::{user_get_by_email, User};

pub async fn user_get_core(email: String) -> Result<Option<User>, &'static str> {
    user_get_by_email(email)
        .await
        .map_err(|_| "Unable to fetch user")
}