use eventing::{events::cart::cart_items_removed::EventCartItemsRemovedV1, EventingPort};
use models::models::cart::{CartItem, CartRepositoryPort};

pub async fn cart_remove_item_core<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    user_id: String,
    product_id: String,
) -> Result<CartItem, error::HexagonalError> {
    let cart_item_result = cart_repository_port
        .cart_remove_item(&user_id.to_ascii_lowercase(), &product_id)
        .await;

    if cart_item_result.is_ok() {
        let event_result = eventing_port
            .emit(&EventCartItemsRemovedV1::new(vec![cart_item_result
                .clone()
                .unwrap()]))
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
    async fn test_cart_remove_item_core() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();

        let product_id = uuid::Uuid::new_v4().to_string();
        let user_id = uuid::Uuid::new_v4().to_string();

        let result_cart_item = CartItem {
            product_id: product_id.clone(),
            user_id: user_id.clone(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        cart_repository_port
            .expect_cart_remove_item()
            .returning(move |_, _| Ok(result_cart_item.clone()));

        let mut eventing_port = eventing::MockEventingPort::new();
        eventing_port
            .expect_emit::<EventCartItemsRemovedV1>()
            .returning(move |_| Ok(()));

        // Act
        let result =
            cart_remove_item_core(&cart_repository_port, &eventing_port, user_id, product_id).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cart_remove_item_core_cart_repository_error() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();

        cart_repository_port
            .expect_cart_remove_item()
            .returning(move |_, _| {
                Err(error::HexagonalError {
                    message: "Error".to_string(),
                    error: error::HexagonalErrorCode::AdaptorError,
                    trace: "".to_string(),
                })
            });

        let mut eventing_port = eventing::MockEventingPort::new();
        eventing_port
            .expect_emit::<EventCartItemsRemovedV1>()
            .returning(move |_| Ok(()));

        // Act
        let result = cart_remove_item_core(
            &cart_repository_port,
            &eventing_port,
            uuid::Uuid::new_v4().to_string(),
            uuid::Uuid::new_v4().to_string(),
        )
        .await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cart_remove_item_core_eventing_error() {
        // Arrange
        let mut cart_repository_port = models::models::cart::MockCartRepositoryPort::new();

        let result_cart_item = CartItem {
            product_id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            quantity: 1,
            created_at: default_time(),
            updated_at: default_time(),
        };

        cart_repository_port
            .expect_cart_remove_item()
            .returning(move |_, _| Ok(result_cart_item.clone()));

        let mut eventing_port = eventing::MockEventingPort::new();
        eventing_port
            .expect_emit::<EventCartItemsRemovedV1>()
            .returning(move |_| {
                Err(error::HexagonalError {
                    message: "Error".to_string(),
                    error: error::HexagonalErrorCode::AdaptorError,
                    trace: "".to_string(),
                })
            });

        // Act
        let result = cart_remove_item_core(
            &cart_repository_port,
            &eventing_port,
            uuid::Uuid::new_v4().to_string(),
            uuid::Uuid::new_v4().to_string(),
        )
        .await;

        // Assert
        assert!(result.is_err());
    }
}
