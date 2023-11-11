use error::HexagonalError;
use eventing::{events::user::user_updated::EventUserUpdatedV1, EventingPort};
use models::models::user::{MutableUser, User, UserRepositoryPort};

pub async fn user_update_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    username: &String,
    update: MutableUser,
) -> Result<User, HexagonalError> {
    if update.first.is_none() && update.last.is_none() {
        return Err(HexagonalError {
            error: error::HexagonalErrorCode::BadInput,
            message: "No update parameters specified".to_string(),
            trace: "".to_string(),
        });
    }

    let user = user_repository_port
        .user_update_by_username(username, update)
        .await;

    if user.is_ok() {
        let event_result = eventing_port
            .emit(&EventUserUpdatedV1::new(user.clone().unwrap()))
            .await;
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
    async fn test_user_update_core() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "testemail".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let mutable_user = MutableUser {
            first: Some("first".to_string()),
            last: Some("last".to_string()),
        };

        let return_user = user.clone();

        user_repository_port
            .expect_user_update_by_username()
            .times(1)
            .returning(move |_, _| Ok(return_user.clone()));

        eventing_port
            .expect_emit::<EventUserUpdatedV1>()
            .times(1)
            .returning(move |_| Ok(()));

        // Act
        let result = user_update_core(
            &user_repository_port,
            &eventing_port,
            &user.username,
            mutable_user.clone(),
        )
        .await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[tokio::test]
    async fn test_user_update_core_no_updates() {
        // Arrange
        let user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "testemail".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let mutable_user = MutableUser {
            first: None,
            last: None,
        };

        // Act
        let result = user_update_core(
            &user_repository_port,
            &eventing_port,
            &user.username,
            mutable_user.clone(),
        )
        .await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().error,
            error::HexagonalErrorCode::BadInput
        );
    }

    #[tokio::test]
    async fn test_user_update_core_error_from_dynamo() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "testemail".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let mutable_user = MutableUser {
            first: Some("first".to_string()),
            last: Some("last".to_string()),
        };

        user_repository_port
            .expect_user_update_by_username()
            .times(1)
            .returning(move |_, _| {
                Err(HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result = user_update_core(
            &user_repository_port,
            &eventing_port,
            &user.username,
            mutable_user.clone(),
        )
        .await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().error,
            error::HexagonalErrorCode::AdaptorError
        );
    }

    #[tokio::test]
    async fn test_user_update_core_error_from_eventing() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "testemail".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let mutable_user = MutableUser {
            first: Some("first".to_string()),
            last: Some("last".to_string()),
        };

        let return_user = user.clone();

        user_repository_port
            .expect_user_update_by_username()
            .times(1)
            .returning(move |_, _| Ok(return_user.clone()));

        eventing_port
            .expect_emit::<EventUserUpdatedV1>()
            .times(1)
            .returning(move |_| {
                Err(HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result = user_update_core(
            &user_repository_port,
            &eventing_port,
            &user.username,
            mutable_user.clone(),
        )
        .await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().error,
            error::HexagonalErrorCode::AdaptorError
        );
    }
}
