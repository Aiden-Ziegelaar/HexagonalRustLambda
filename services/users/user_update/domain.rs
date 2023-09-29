use error::HexagonalError;
use eventing::{events::user::user_updated::EventUserUpdatedV1, EventingPort};
use models::models::user::{MutableUser, User, UserRepositoryPort};

pub async fn user_update_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    who: MutableUser,
) -> Result<User, HexagonalError> {

    if who.first.is_none() && who.last.is_none() {
        return Err(HexagonalError {
            error: error::HexagonalErrorCode::BadInput,
            message: "No update parameters specified".to_string(),
            trace: "".to_string(),
        });
    }

    let user = user_repository_port.user_update_by_email(who).await;

    if user.is_ok() {
        let event_result = eventing_port.emit(&EventUserUpdatedV1::new(user.clone().unwrap())).await;
        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    user
}
