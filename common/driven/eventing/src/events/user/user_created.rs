use serde::{Deserialize, Serialize};

use crate::events::event_emmiter::SerialisableEvent;

const EVENT_TYPE: &str = "user_created";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventUserCreatedV1 {
    pub version: u32,
    pub event_type: String,
    pub user: models::models::user::User,
}

impl EventUserCreatedV1 {
    pub fn new(user: models::models::user::User) -> Self {
        Self {
            version: 1,
            event_type: EVENT_TYPE.to_string(),
            user,
        }
    }
}

impl SerialisableEvent for EventUserCreatedV1 {
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
