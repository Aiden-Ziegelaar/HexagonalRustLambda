use eventing::{events::user::user_created::EventUserCreatedV1, EventingPort};
use models::models::user::{User, UserRepositoryPort};
use regex::Regex;
use tokio::sync::OnceCell;

pub static EMAIL_REGEX: OnceCell<Regex> = OnceCell::const_new();

pub async fn create_email_regex() -> Regex {
    Regex::new("^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|\"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*\")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\\])$").unwrap()
}

pub async fn user_create_core<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    mut who: User,
) -> Result<User, error::HexagonalError> {
    let email_regex = EMAIL_REGEX.get_or_init(create_email_regex);

    // make sure email is lowercase
    who.email = who.email.to_ascii_lowercase();

    // validate email with regex
    if !email_regex.await.is_match(&who.email) {
        return Err(error::HexagonalError {
            error: error::HexagonalErrorCode::BadInput,
            message: "Invalid email".to_string(),
            trace: "".to_string(),
        });
    }

    let user = user_repository_port.user_create(who).await;

    if user.is_ok() {
        let event_result = eventing_port
            .emit(&EventUserCreatedV1::new(user.clone().unwrap()))
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
    async fn test_email_regex() {
        let email_regex = EMAIL_REGEX.get_or_init(create_email_regex).await;
        assert!(email_regex.is_match("test@email.com"));
        assert!(email_regex.is_match("test_test@email.com"));
        assert!(email_regex.is_match("test_test@subdomain.email.com"));
        assert!(!email_regex.is_match("notanemail"));
        assert!(!email_regex.is_match("CAPItalsemail@email.com"));
        assert!(!email_regex.is_match("CAPItalsemail@email.com"));
        assert!(!email_regex.is_match("test@Email.com"));
    }

    #[tokio::test]
    async fn test_user_create_core() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "testemail@email.com".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let returned_user = user.clone();

        eventing_port
            .expect_emit::<EventUserCreatedV1>()
            .times(1)
            .returning(move |_| Ok(()));
        user_repository_port
            .expect_user_create()
            .times(1)
            .returning(move |_| Ok(returned_user.clone()));

        // Act
        let result = user_create_core(&user_repository_port, &eventing_port, user.clone()).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[tokio::test]
    async fn test_user_create_core_invalid_email() {
        // Arrange
        let user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "notanemail".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        // Act
        let result = user_create_core(&user_repository_port, &eventing_port, user.clone()).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().error,
            error::HexagonalErrorCode::BadInput
        );
    }

    #[tokio::test]
    async fn test_user_create_error_from_dynamo() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "averygoodemail@email.com".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        user_repository_port
            .expect_user_create()
            .times(1)
            .returning(move |_| {
                Err(error::HexagonalError {
                    error: error::HexagonalErrorCode::Conflict,
                    message: "Error in Dynamo".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result = user_create_core(&user_repository_port, &eventing_port, user.clone()).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().error,
            error::HexagonalErrorCode::Conflict
        );
    }

    #[tokio::test]
    async fn test_user_create_error_from_eventing() {
        // Arrange
        let mut user_repository_port = models::models::user::MockUserRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let user = User {
            email: "averygoodemail@email.com".to_string(),
            first: "first".to_string(),
            last: "last".to_string(),
            username: "username".to_string(),
            created_at: default_time(),
            updated_at: default_time(),
        };

        let returned_user = user.clone();

        eventing_port
            .expect_emit::<EventUserCreatedV1>()
            .times(1)
            .returning(move |_| {
                Err(error::HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "".to_string(),
                    trace: "".to_string(),
                })
            });
        user_repository_port
            .expect_user_create()
            .times(1)
            .returning(move |_| Ok(returned_user.clone()));

        // Act
        let result = user_create_core(&user_repository_port, &eventing_port, user.clone()).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().error,
            error::HexagonalErrorCode::AdaptorError
        );
    }
}
