use models::models::user::user_update_username_by_email;

pub async fn user_username_update_core(
    email: String,
    username: String,
) -> Result<(), &'static str> {
    user_update_username_by_email(email, username)
        .await
        .map_err(|_| "Unable to update username user")
}
