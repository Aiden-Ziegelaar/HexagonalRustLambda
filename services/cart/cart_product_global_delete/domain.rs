use models::models::cart::CartRepositoryPort;

pub async fn cart_product_delete_core<T1: CartRepositoryPort>(
    cart_repository_port: &T1,
    product_id: String,
) -> Result<(), Vec<error::HexagonalError>> {
    let product_delete_result = cart_repository_port
        .cart_global_remove_product(&product_id)
        .await;

    product_delete_result
}

#[cfg(test)]
mod tests {}
