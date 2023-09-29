use error::HexagonalError;
use models::models::user::{User, UserRepositoryPort};

pub async fn user_get_core<T1: UserRepositoryPort>(
    user_repository_port: &T1,
    email: String,
) -> Result<Option<User>, HexagonalError> {
    user_repository_port.user_get_by_email(email).await
}
