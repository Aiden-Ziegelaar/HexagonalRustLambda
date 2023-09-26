use serde::{Deserialize, Serialize};

use crate::{ EVENTING, EventingRepository };

const EVENT_TYPE: &str = "user_created";

#[derive(Serialize, Deserialize)]
pub struct EventUserCreatedV1 {
    pub version: u32,
    pub event_type: String,
    pub user: models::models::user::User
}

impl EventUserCreatedV1 {
    pub fn new(user: models::models::user::User) -> Self {
        Self { 
            version: 1, 
            event_type: EVENT_TYPE.to_string(),
            user
        }
    }

    pub async fn emit(&self) {
        EVENTING.get_or_init(EventingRepository::new).await
            .put_event_on_bus(
                EVENT_TYPE.to_string(),
                serde_json::to_string(self).unwrap()
            )
            .await
            .unwrap();
    }
}