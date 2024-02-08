use crate::domain::cart_clear_delete_core;
use eventing::{events::user::user_deleted::EventUserDeletedV1, EventingPort};
use models::models::cart::CartRepositoryPort;

pub async fn cart_clear_user_deleted_event_port<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    event: EventUserDeletedV1,
) -> Result<(), ()> {
    let username = event.user.username;
    match cart_clear_delete_core(cart_repository_port, eventing_port, username.to_string()).await {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Error: {}", err);
            Ok(())
        }
    }
}
