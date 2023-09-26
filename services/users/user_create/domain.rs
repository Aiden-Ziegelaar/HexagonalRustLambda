use eventing::events::user::user_created::EventUserCreatedV1;
use models::models::user::{user_create, User};

pub async fn user_create_core(who: User) -> Result<User, &'static str> {
    let user = user_create(who)
        .await;

    match user {
        Ok(result) => {
            EventUserCreatedV1::new(result.clone()).emit().await;
            return Ok(result);
        },
        Err(_) => return Err("Unable to create user")
    }
}
