use error::HexagonalError;
use eventing::{events::user::username_updated::EventUsernameUpdatedV1, EventingPort};
use models::models::user::UserRepositoryPort;

pub async fn user_username_update_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    email: &String,
    username: &String,
) -> Result<(), HexagonalError> {
    let result = user_repository_port
        .user_update_username_by_email(email, username)
        .await;

    if result.is_ok() {
        let event_result = eventing_port
            .emit(&EventUsernameUpdatedV1::new(
                email.clone(),
                username.clone(),
            ))
            .await;

        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_username_update_core() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailIsNotValidated".to_string();
        let username = "thisUsernameIsNotValidated".to_string();

        user_repository_port
            .expect_user_update_username_by_email()
            .times(1)
            .returning(move |_, _| Ok(()));

        eventing_port
            .expect_emit::<EventUsernameUpdatedV1>()
            .times(1)
            .returning(move |_| Ok(()));

        // Act
        let result =
            user_username_update_core(&user_repository_port, &eventing_port, &email, &username)
                .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_username_update_core_error() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailIsNotValidated".to_string();
        let username = "thisUsernameIsNotValidated".to_string();

        user_repository_port
            .expect_user_update_username_by_email()
            .times(1)
            .returning(move |_, _| {
                Err(HexagonalError {
                    error: error::HexagonalErrorCode::NotFound,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result =
            user_username_update_core(&user_repository_port, &eventing_port, &email, &username)
                .await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_user_username_update_core_error_from_eventing() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailIsNotValidated".to_string();
        let username = "thisUsernameIsNotValidated".to_string();

        user_repository_port
            .expect_user_update_username_by_email()
            .times(1)
            .returning(move |_, _| Ok(()));

        eventing_port
            .expect_emit::<EventUsernameUpdatedV1>()
            .times(1)
            .returning(move |_| {
                Err(HexagonalError {
                    error: error::HexagonalErrorCode::NotFound,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result =
            user_username_update_core(&user_repository_port, &eventing_port, &email, &username)
                .await;

        // Assert
        assert!(result.is_err());
    }
}
