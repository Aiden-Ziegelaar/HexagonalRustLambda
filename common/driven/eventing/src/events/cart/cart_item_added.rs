use serde::{Deserialize, Serialize};

use crate::events::event_emmiter::SerialisableEvent;

const EVENT_TYPE: &str = "cart_item_added";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventCartItemAddedV1 {
    pub version: u32,
    pub event_type: String,
    pub cart_item: models::models::cart::CartItem,
}

impl EventCartItemAddedV1 {
    pub fn new(cart_item: models::models::cart::CartItem) -> Self {
        Self {
            version: 1,
            event_type: EVENT_TYPE.to_string(),
            cart_item,
        }
    }
}

impl SerialisableEvent for EventCartItemAddedV1 {
    fn get_event_type(&self) -> &String {
        &self.event_type
    }

    fn get_version(&self) -> u32 {
        self.version
    }

    fn serialise(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
