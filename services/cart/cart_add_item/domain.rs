use eventing::{events::cart::cart_item_added::EventCartItemAddedV1, EventingPort};
use models::models::cart::{CartItem, CartRepositoryPort};

pub async fn cart_add_item_core<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    mut cart_item: CartItem,
) -> Result<CartItem, error::HexagonalError> {
    cart_item.user_id = cart_item.user_id.to_ascii_lowercase();
    let cart_item_result = cart_repository_port.cart_add_item(&cart_item).await;

    if cart_item_result.is_ok() {
        let event_result = eventing_port
            .emit(&EventCartItemAddedV1::new(cart_item.clone()))
            .await;
        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    cart_item_result
}

#[cfg(test)]
mod tests {
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_cart_add_item_core() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let cart_item = CartItem {
            product_id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let result_cart_item = cart_item.clone();

        cart_repository_port
            .expect_cart_add_item()
            .returning(move |_| Ok(result_cart_item.clone()));

        eventing_port
            .expect_emit::<EventCartItemAddedV1>()
            .times(1)
            .returning(|_| Ok(()));

        // Act
        let result = cart_add_item_core(&cart_repository_port, &eventing_port, cart_item).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cart_add_item_core_eventing_error() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let cart_item = CartItem {
            product_id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let result_cart_item = cart_item.clone();

        cart_repository_port
            .expect_cart_add_item()
            .returning(move |_| Ok(result_cart_item.clone()));

        eventing_port
            .expect_emit::<EventCartItemAddedV1>()
            .times(1)
            .returning(|_| {
                Err(error::HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result = cart_add_item_core(&cart_repository_port, &eventing_port, cart_item).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cart_add_item_core_repository_error() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let cart_item = CartItem {
            product_id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        cart_repository_port
            .expect_cart_add_item()
            .returning(move |_| {
                Err(error::HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "test".to_string(),
                    trace: "".to_string(),
                })
            });

        eventing_port
            .expect_emit::<EventCartItemAddedV1>()
            .times(0)
            .returning(|_| Ok(()));

        // Act
        let result = cart_add_item_core(&cart_repository_port, &eventing_port, cart_item).await;

        // Assert
        assert!(result.is_err());
    }
}
