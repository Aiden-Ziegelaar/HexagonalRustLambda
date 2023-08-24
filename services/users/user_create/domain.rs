use models::models::user::{user_create, User};

pub async fn user_create_core(who: User) -> Result<(), &'static str> {
    user_create(who).await.map_err(|_| "Unable to create user")
}
