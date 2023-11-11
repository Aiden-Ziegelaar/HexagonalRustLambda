use serde::{Deserialize, Serialize};

use crate::events::event_emmiter::SerialisableEvent;

const EVENT_TYPE: &str = "user_email_updated";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventEmailUpdatedV1 {
    pub version: u32,
    pub event_type: String,
    pub new_email: String,
    pub username: String,
}

impl EventEmailUpdatedV1 {
    pub fn new(username: String, new_email: String) -> Self {
        Self {
            version: 1,
            event_type: EVENT_TYPE.to_string(),
            username,
            new_email,
        }
    }
}

impl SerialisableEvent for EventEmailUpdatedV1 {
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
