use crate::domain::cart_product_delete_core;

use eventing::events::product::product_deleted::EventProductDeletedV1;
use models::models::cart::CartRepositoryPort;

pub async fn cart_product_deleted_event_port<T1: CartRepositoryPort>(
    cart_repository_port: &T1,
    event: EventProductDeletedV1,
) -> Result<(), ()> {
    let product_id = event.product.id;
    match cart_product_delete_core(cart_repository_port, product_id.to_string()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            err.iter().for_each(|e| println!("Error: {}", e));
            Ok(())
        }
    }
}
