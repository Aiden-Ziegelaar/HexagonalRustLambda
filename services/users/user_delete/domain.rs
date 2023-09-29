use error::HexagonalError;
use eventing::{events::user::user_deleted::EventUserDeletedV1, EventingPort};
use models::models::user::{User, UserRepositoryPort};

pub async fn user_delete_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    email: String,
) -> Result<User, HexagonalError> {
    let lowercase_email = email.to_lowercase();
    let user = user_repository_port.user_delete_by_email(lowercase_email).await;

    if user.is_ok() {
        let event_result = eventing_port.emit(&EventUserDeletedV1::new(user.clone().unwrap())).await;
        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    user
}
