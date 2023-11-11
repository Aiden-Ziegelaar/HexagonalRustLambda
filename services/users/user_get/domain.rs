use error::HexagonalError;
use models::models::user::{User, UserRepositoryPort};

pub async fn user_get_core<T1: UserRepositoryPort>(
    user_repository_port: &T1,
    username: &String,
) -> Result<Option<User>, HexagonalError> {
    user_repository_port.user_get_by_username(username).await
}

#[cfg(test)]
mod tests {
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_user_get_core() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();

        let username = "mycoolusername".to_string();

        let user = User {
            email: "test@test.com".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let return_user = user.clone();

        user_repository_port
            .expect_user_get_by_username()
            .times(1)
            .returning(move |_| Ok(Some(return_user.clone())));

        // Act
        let result = user_get_core(&user_repository_port, &username).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), user);
    }

    #[tokio::test]
    async fn test_user_get_core_not_found() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();

        let username = "mycoolusername".to_string();

        user_repository_port
            .expect_user_get_by_username()
            .times(1)
            .returning(move |_| Ok(None));

        // Act
        let result = user_get_core(&user_repository_port, &username).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn test_user_get_core_error() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();

        let username = "mycoolusername".to_string();

        user_repository_port
            .expect_user_get_by_username()
            .times(1)
            .returning(move |_| {
                Err(HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result = user_get_core(&user_repository_port, &username).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().error,
            error::HexagonalErrorCode::AdaptorError
        );
    }
}
