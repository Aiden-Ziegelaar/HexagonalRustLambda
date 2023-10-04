use error::HexagonalError;
use models::models::product::{Product, ProductRepositoryPort};

pub async fn product_get_core<T1: ProductRepositoryPort>(
    product_repository_port: &T1,
    id: &String,
) -> Result<Option<Product>, HexagonalError> {
    product_repository_port.product_get_by_id(id).await
}

#[cfg(test)]
mod tests {
    use models::default_time;

    use super::*;

    #[tokio::test]
    async fn test_product_get_core() {
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            product_name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        let result_product = product.clone();

        product_repository_port
            .expect_product_get_by_id()
            .returning(move |_| Ok(Some(result_product.clone())));

        let result = product_get_core(&product_repository_port, &product.id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_product_get_core_not_found() {
        let mut product_repository_port = models::models::product::MockProductRepositoryPort::new();

        let product = Product {
            id: uuid::Uuid::new_v4().to_string(),
            product_name: "test".to_string(),
            description: "test".to_string(),
            price_cents: 10000,
            created_at: default_time(),
            updated_at: default_time(),
        };

        product_repository_port
            .expect_product_get_by_id()
            .returning(move |_| Ok(None));

        let result = product_get_core(&product_repository_port, &product.id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
