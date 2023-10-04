use error::HexagonalError;
use eventing::{events::product::product_updated::EventProductUpdatedV1, EventingPort};
use models::models::product::{MutableProduct, Product, ProductRepositoryPort};

pub async fn product_update_core<T1: ProductRepositoryPort, T2: EventingPort>(
    product_repository_port: &T1,
    eventing_port: &T2,
    id: &String,
    product_updates: MutableProduct,
) -> Result<Product, HexagonalError> {
    if product_updates.price_cents.is_none() && product_updates.product_name.is_none() && product_updates.description.is_none() {
        return Err(HexagonalError {
            error: error::HexagonalErrorCode::BadInput,
            message: "No update parameters specified".to_string(),
            trace: "".to_string(),
        });
    }

    let product = product_repository_port.product_update_by_id(id, &product_updates).await;

    if product.is_ok() {
        let event_result = eventing_port
            .emit(&EventProductUpdatedV1::new(product.clone().unwrap()))
            .await;
        if event_result.is_err() {
            return Err(event_result.unwrap_err());
        }
    }

    product
}

#[cfg(test)]
mod tests {
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_product_update_core() {
        // Arrange
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            product_name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let mutable_product = MutableProduct {
            product_name: Some("test".to_string()),
            description: Some("test".to_string()),
            price_cents: Some(10000),
        };

        let return_product = product.clone();

        product_repository_port
            .expect_product_update_by_id()
            .returning(move |_, _| Ok(return_product.clone()));

        eventing_port
            .expect_emit::<EventProductUpdatedV1>()
            .times(1)
            .returning(|_| Ok(()));

        // Act
        let result = product_update_core(&product_repository_port, &eventing_port, &product.id, mutable_product).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_product_update_core_eventing_error() {
        // Arrange
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            product_name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let mutable_product = MutableProduct {
            product_name: Some("test".to_string()),
            description: Some("test".to_string()),
            price_cents: Some(10000),
        };

        let return_product = product.clone();

        product_repository_port
            .expect_product_update_by_id()
            .returning(move |_, _| Ok(return_product.clone()));

        eventing_port
            .expect_emit::<EventProductUpdatedV1>()
            .times(1)
            .returning(|_| Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "".to_string(),
                trace: "".to_string(),
            }));

        // Act
        let result = product_update_core(&product_repository_port, &eventing_port, &product.id, mutable_product).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_product_update_core_error_from_repository() {
        // Arrange
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();
        let eventing_port = eventing::MockEventingPort::new();

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            product_name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let mutable_product = MutableProduct {
            product_name: Some("test".to_string()),
            description: Some("test".to_string()),
            price_cents: Some(10000),
        };

        product_repository_port
            .expect_product_update_by_id()
            .returning(move |_, _| {
                Err(HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "".to_string(),
                    trace: "".to_string(),
                })
            });

        // Act
        let result = product_update_core(&product_repository_port, &eventing_port, &product.id, mutable_product).await;

        // Assert
        assert!(result.is_err());
    }
}
