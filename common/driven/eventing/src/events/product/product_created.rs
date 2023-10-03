use serde::{Deserialize, Serialize};

use crate::events::event_emmiter::SerialisableEvent;

const EVENT_TYPE: &str = "product_created";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventProductCreatedV1 {
    pub version: u32,
    pub event_type: String,
    pub product: models::models::product::Product,
}

impl EventProductCreatedV1 {
    pub fn new(product: models::models::product::Product) -> Self {
        Self {
            version: 1,
            event_type: EVENT_TYPE.to_string(),
            product,
        }
    }
}

impl SerialisableEvent for EventProductCreatedV1 {
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
