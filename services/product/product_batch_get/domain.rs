use error::HexagonalError;
use models::models::product::{Product, ProductRepositoryPort};

pub async fn product_get_batch_core<T1: ProductRepositoryPort>(
    product_repository_port: &T1,
    ids: &Vec<String>,
) -> Result<Vec<Product>, HexagonalError> {
    product_repository_port.product_get_by_ids(ids).await
}

#[cfg(test)]
mod tests {
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_product_get_core() {
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();

        let product1 = Product {
            id: uuid::Uuid::new_v4().to_string(),
            product_name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let product2 = Product {
            id: uuid::Uuid::new_v4().to_string(),
            product_name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let product_ids = vec![product1.id.clone(), product2.id.clone()];

        product_repository_port
            .expect_product_get_by_ids()
            .returning(move |_| Ok(vec![product1.clone(), product2.clone()]));

        let result = product_get_batch_core(&product_repository_port, &product_ids).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_product_get_core_not_found() {
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();

        let product_ids = vec![
            uuid::Uuid::new_v4().to_string(),
            uuid::Uuid::new_v4().to_string(),
        ];

        product_repository_port
            .expect_product_get_by_ids()
            .returning(|_| Ok(vec![]));

        let result = product_get_batch_core(&product_repository_port, &product_ids).await;

        assert!(result.is_ok());
    }
}
