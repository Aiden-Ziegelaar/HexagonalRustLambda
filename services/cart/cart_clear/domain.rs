use eventing::events::cart::cart_items_removed::EventCartItemsRemovedV1;
use eventing::EventingPort;
use models::models::cart::{CartItem, CartRepositoryPort};

pub async fn cart_clear_delete_core<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    user_id: String,
) -> Result<Vec<CartItem>, error::HexagonalError> {
    let cart_clear_result = cart_repository_port
        .cart_clear(&user_id.to_ascii_lowercase())
        .await;

    if cart_clear_result.is_ok() {
        let event_result = eventing_port
            .emit(&EventCartItemsRemovedV1::new(
                cart_clear_result.clone().unwrap(),
            ))
            .await;
        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    cart_clear_result
}

#[cfg(test)]
mod tests {
    use eventing::events::cart::cart_items_removed::EventCartItemsRemovedV1;
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_cart_clear_delete_core() {
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
            .expect_cart_clear()
            .returning(move |_| Ok(vec![result_cart_item.clone()]));

        let mut eventing_port = eventing::MockEventingPort::new();
        eventing_port
            .expect_emit::<EventCartItemsRemovedV1>()
            .returning(move |_| Ok(()));

        // Act
        let result =
            cart_clear_delete_core(&cart_repository_port, &eventing_port, cart_item.user_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cart_clear_delete_core_cart_repository_error() {
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
            .expect_cart_clear()
            .returning(move |_| {
                Err(error::HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "Error".to_string(),
                    trace: "".to_string(),
                })
            });

        let mut eventing_port = eventing::MockEventingPort::new();
        eventing_port
            .expect_emit::<EventCartItemsRemovedV1>()
            .returning(move |_| Ok(()));

        // Act
        let result =
            cart_clear_delete_core(&cart_repository_port, &eventing_port, cart_item.user_id).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cart_clear_delete_core_eventing_error() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();

        let cart_item = CartItem {
            product_id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let cart_item_result = cart_item.clone();

        cart_repository_port
            .expect_cart_clear()
            .returning(move |_| Ok(vec![cart_item_result.clone()]));

        let mut eventing_port = eventing::MockEventingPort::new();
        eventing_port
            .expect_emit::<EventCartItemsRemovedV1>()
            .returning(move |_| {
                Err(error::HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "Error".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result =
            cart_clear_delete_core(&cart_repository_port, &eventing_port, cart_item.user_id).await;

        // Assert
        assert!(result.is_err());
    }
}
