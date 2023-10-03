use eventing::{events::product::product_created::EventProductCreatedV1, EventingPort};
use models::models::product::{Product, ProductRepositoryPort};

pub async fn product_create_core<T1: ProductRepositoryPort, T2: EventingPort>(
    product_repository_port: &T1,
    eventing_port: &T2,
    product: Product,
) -> Result<Product, error::HexagonalError> {

    let product = product_repository_port.product_create(&product).await;

    if product.is_ok() {
        let event_result = eventing_port
            .emit(&EventProductCreatedV1::new(product.clone().unwrap()))
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
    async fn test_product_create_core() {
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
            .expect_product_create()
            .returning(move |_| Ok(result_product.clone()));

        eventing_port
            .expect_emit::<EventProductCreatedV1>()
            .times(1)
            .returning(|_| Ok(()));

        let result = product_create_core(&product_repository_port, &eventing_port, product).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_product_create_core_eventing_error() {
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
            .expect_product_create()
            .returning(move |_| Ok(result_product.clone()));

        eventing_port
            .expect_emit::<EventProductCreatedV1>()
            .times(1)
            .returning(|_| Err(error::HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "test".to_string(),
                trace: "test".to_string(),
            }));

        let result = product_create_core(&product_repository_port, &eventing_port, product).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_product_create_core_product_error() {
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
            .expect_product_create()
            .returning(move |_| Err(error::HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "test".to_string(),
                trace: "test".to_string(),
            }));

        eventing_port
            .expect_emit::<EventProductCreatedV1>()
            .times(0);

        let result = product_create_core(&product_repository_port, &eventing_port, product).await;

        assert!(result.is_err());
    }

}
