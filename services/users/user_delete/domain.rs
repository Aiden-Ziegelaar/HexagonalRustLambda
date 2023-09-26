use models::models::user::{user_delete_by_email, User};
use eventing::events::user::user_deleted::EventUserDeletedV1;

pub async fn user_delete_core(email: String) -> Result<User, &'static str> {
    let user = user_delete_by_email(email)
        .await
        .map_err(|_| "Unable to update user");

    match user {
        Ok(result) => {
            EventUserDeletedV1::new(result.clone()).emit().await;
            return Ok(result);
        }
        Err(_) => return Err("Unable to delete user")
    }
}
