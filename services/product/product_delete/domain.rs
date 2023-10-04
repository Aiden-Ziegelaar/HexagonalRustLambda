use error::HexagonalError;
use eventing::{events::product::product_deleted::EventProductDeletedV1, EventingPort};
use models::models::product::{Product, ProductRepositoryPort};

pub async fn product_delete_core<T1: ProductRepositoryPort, T2: EventingPort>(
    product_repository_port: &T1,
    eventing_port: &T2,
    id: &String,
) -> Result<Product, HexagonalError> {
    let product = product_repository_port
        .product_delete_by_id(id)
        .await;

    if product.is_ok() {
        let event_result = eventing_port
            .emit(&EventProductDeletedV1::new(product.clone().unwrap()))
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
    async fn test_product_delete_core() {
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();  

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let result_product = product.clone();

        product_repository_port
            .expect_product_delete_by_id()
            .returning(move |_| Ok(result_product.clone()));

        eventing_port
            .expect_emit::<EventProductDeletedV1>()
            .times(1)
            .returning(|_| Ok(()));

        let result = product_delete_core(&product_repository_port, &eventing_port, &product.id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_product_delete_core_eventing_error() {
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();  

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let result_product = product.clone();

        product_repository_port
            .expect_product_delete_by_id()
            .returning(move |_| Ok(result_product.clone()));

        eventing_port
            .expect_emit::<EventProductDeletedV1>()
            .times(1)
            .returning(|_| Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "test".to_string(),
                trace: "".to_string(),
            }));

        let result = product_delete_core(&product_repository_port, &eventing_port, &product.id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_product_delete_core_product_error() {
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();
        let mut eventing_port = eventing::MockEventingPort::new();  

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };
        

        product_repository_port
            .expect_product_delete_by_id()
            .returning(move |_| Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "test".to_string(),
                trace: "".to_string(),
            }));

        eventing_port
            .expect_emit::<EventProductDeletedV1>()
            .times(0)
            .returning(|_| Ok(()));

        let result = product_delete_core(&product_repository_port, &eventing_port, &product.id).await;

        assert!(result.is_err());
    }
}
