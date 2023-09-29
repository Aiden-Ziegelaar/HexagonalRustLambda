use error::HexagonalError;
use eventing::{events::user::user_updated::EventUserUpdatedV1, EventingPort};
use models::models::user::{MutableUser, User, UserRepositoryPort};

pub async fn user_update_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    who: MutableUser,
) -> Result<User, HexagonalError> {
    let user = user_repository_port.user_update_by_email(who).await;

    if user.is_ok() {
        let event_result = eventing_port.emit(&EventUserUpdatedV1::new(user.clone().unwrap())).await;
        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    user
}
