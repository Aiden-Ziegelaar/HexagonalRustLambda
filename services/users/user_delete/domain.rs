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

#[cfg(test)]
mod tests {
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_user_delete_core() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailIsNotValidated".to_string();

        let user = User {
            email: email.clone(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let return_user = user.clone();

        user_repository_port.expect_user_delete_by_email().times(1).returning(move |_| Ok(return_user.clone()));

        eventing_port.expect_emit::<EventUserDeletedV1>().times(1).returning(move |_| Ok(()));

        // Act
        let result = user_delete_core(&user_repository_port, &eventing_port, email.clone()).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);        
    }

    #[tokio::test]
    async fn test_user_delete_not_found() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailIsNotValidated".to_string();

        user_repository_port.expect_user_delete_by_email().times(1).returning(move |_| Err(HexagonalError {
            error: error::HexagonalErrorCode::NotFound,
            message: "User not found".to_string(),
            trace: "".to_string(),
        }));

        // Act
        let result = user_delete_core(&user_repository_port, &eventing_port, email.clone()).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, error::HexagonalErrorCode::NotFound);
    }

    #[tokio::test]
    async fn test_user_delete_eventing_failure() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let email = "thisEmailIsNotValidated".to_string();

        let user = User {
            email: email.clone(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        user_repository_port.expect_user_delete_by_email().times(1).returning(move |_| Ok(user.clone()));

        eventing_port.expect_emit::<EventUserDeletedV1>().times(1).returning(move |_| Err(HexagonalError {
            error: error::HexagonalErrorCode::AdaptorError,
            message: "Error in Eventing".to_string(),
            trace: "".to_string(),
        }));

        // Act
        let result = user_delete_core(&user_repository_port, &eventing_port, email.clone()).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, error::HexagonalErrorCode::AdaptorError);
    }
}

