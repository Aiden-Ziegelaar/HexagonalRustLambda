use error::HexagonalError;
use eventing::{events::user::username_updated::EventUsernameUpdatedV1, EventingPort};
use models::models::user::UserRepositoryPort;

pub async fn user_username_update_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    email: String,
    username: String,
) -> Result<(), HexagonalError> {
    let result = user_repository_port
        .user_update_username_by_email(email.clone(), username.clone())
        .await;

    if result.is_ok() {
        let event_result = eventing_port.emit(&EventUsernameUpdatedV1::new(
            email.clone(),
            username.clone(),
        )).await;

        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    result
}
