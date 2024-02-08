use error::HexagonalError;
use eventing::{events::user::username_updated::EventEmailUpdatedV1, EventingPort};
use lib_user_regexes::create_email_regex;
use models::models::user::UserRepositoryPort;
use regex::Regex;
use tokio::sync::OnceCell;

pub static EMAIL_REGEX: OnceCell<Regex> = OnceCell::const_new();

pub async fn user_email_update_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    username: &String,
    new_email: &String,
) -> Result<(), HexagonalError> {
    let email_regex = EMAIL_REGEX.get_or_init(create_email_regex);

    let lower_email = new_email.to_ascii_lowercase();

    if !email_regex.await.is_match(&lower_email) {
        return Err(HexagonalError {
            error: error::HexagonalErrorCode::BadInput,
            message: "Invalid email".to_string(),
            trace: "".to_string(),
        });
    }

    let result = user_repository_port
        .user_update_email_by_username(username, &lower_email)
        .await;

    if result.is_ok() {
        let event_result = eventing_port
            .emit(&EventEmailUpdatedV1::new(
                username.clone(),
                lower_email.clone(),
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

        let email = "thisEmailisValidated@test.com".to_string();
        let username = "thisUsernameIsNotValidated".to_string();

        user_repository_port
            .expect_user_update_email_by_username()
            .times(1)
            .returning(move |_, _| Ok(()));

        eventing_port
            .expect_emit::<EventEmailUpdatedV1>()
            .times(1)
            .returning(move |_| Ok(()));

        // Act
        let result =
            user_email_update_core(&user_repository_port, &eventing_port, &username, &email).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_username_update_core_error() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailisValidated@test.com".to_string();
        let username = "thisUsernameIsNotValidated".to_string();

        user_repository_port
            .expect_user_update_email_by_username()
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
            user_email_update_core(&user_repository_port, &eventing_port, &username, &email).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_user_username_update_core_error_from_eventing() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailisValidated@test.com".to_string();
        let username = "thisUsernameIsNotValidated".to_string();

        user_repository_port
            .expect_user_update_email_by_username()
            .times(1)
            .returning(move |_, _| Ok(()));

        eventing_port
            .expect_emit::<EventEmailUpdatedV1>()
            .times(1)
            .returning(move |_| {
                Err(HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result =
            user_email_update_core(&user_repository_port, &eventing_port, &username, &email).await;

        // Assert
        assert!(result.is_err());
    }
}
