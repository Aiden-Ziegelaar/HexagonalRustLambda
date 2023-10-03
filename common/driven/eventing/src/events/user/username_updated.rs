use serde::{Deserialize, Serialize};

use crate::events::event_emmiter::SerialisableEvent;

const EVENT_TYPE: &str = "user_updated";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventUsernameUpdatedV1 {
    pub version: u32,
    pub event_type: String,
    pub email: String,
    pub new_username: String,
}

impl EventUsernameUpdatedV1 {
    pub fn new(email: String, new_username: String) -> Self {
        Self {
            version: 1,
            event_type: EVENT_TYPE.to_string(),
            email,
            new_username,
        }
    }
}

impl SerialisableEvent for EventUsernameUpdatedV1 {
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
