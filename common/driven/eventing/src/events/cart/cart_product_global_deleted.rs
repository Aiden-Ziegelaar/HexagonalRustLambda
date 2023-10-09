use serde::{Deserialize, Serialize};

use crate::events::event_emmiter::SerialisableEvent;

const EVENT_TYPE: &str = "cart_product_global_deleted";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventCartProductGlobalDeletedV1 {
    pub version: u32,
    pub event_type: String,
    pub product_id: String,
}

impl EventCartProductGlobalDeletedV1 {
    pub fn new(product_id: String) -> Self {
        Self {
            version: 1,
            event_type: EVENT_TYPE.to_string(),
            product_id,
        }
    }
}

impl SerialisableEvent for EventCartProductGlobalDeletedV1 {
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
