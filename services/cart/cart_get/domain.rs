use models::models::cart::{CartItem, CartRepositoryPort};

pub async fn cart_get_core<T1: CartRepositoryPort>(
    cart_repository_port: &T1,
    user_id: String,
) -> Result<Vec<CartItem>, error::HexagonalError> {
    let cart_result = cart_repository_port
        .cart_get_by_user_id(&user_id.to_ascii_lowercase())
        .await;

    cart_result
}

#[cfg(test)]
mod tests {
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_cart_get_core() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();

        let cart_item = CartItem {
            product_id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let result_cart_item = cart_item.clone();

        cart_repository_port
            .expect_cart_get_by_user_id()
            .returning(move |_| Ok(vec![result_cart_item.clone()]));

        // Act
        let result = cart_get_core(&cart_repository_port, cart_item.user_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cart_get_core_cart_repository_error() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();

        let cart_item = CartItem {
            product_id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        cart_repository_port
            .expect_cart_get_by_user_id()
            .returning(move |_| {
                Err(error::HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result = cart_get_core(&cart_repository_port, cart_item.user_id).await;

        // Assert
        assert!(result.is_err());
    }
}
