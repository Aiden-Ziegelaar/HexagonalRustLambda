#![feature(lazy_cell)]

use std::fmt;

pub mod events;

use aws_sdk_eventbridge::Client;

use sdk_credential_meta_repository::{SdkCredentialsMetaRepository, AWS_CREDENTIAL_REPOSITORY};
use tokio::sync::OnceCell;

pub static EVENTING: OnceCell<EventingRepository> =
    OnceCell::const_new();

#[derive(Debug, Clone)]
pub struct RepositoryError {
    pub message: String
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to perform eventing operation with error: {}", self.message)
    }
}

pub struct EventingRepository {
    pub client: Client,
    pub bus_name: String
}

impl EventingRepository {
    pub async fn new() -> EventingRepository {
        EventingRepository {
            client: Client::new(
                &AWS_CREDENTIAL_REPOSITORY
                    .get_or_init(SdkCredentialsMetaRepository::new)
                    .await
                    .sdk_config
                    .clone(),
            ),
            bus_name: std::env::var("EVENT_BUS_NAME")
                .expect("EVENT_BUS_NAME environment variable not set"),
        }
    }

    pub async fn put_event_on_bus(&self, event_type: String, event: String) -> Result<(), RepositoryError> {
        let put_events_request = aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
            .set_event_bus_name(Some(self.bus_name.clone()))
            .set_detail_type(Some(event_type.clone()))
            .set_detail(Some(event.clone()))
            .build();
        
        self.client
            .put_events()
            .entries(put_events_request)
            .send()
            .await
            .map_err(|_| RepositoryError {
                message: format!("Unable to put event of type {} on bus", event_type)
            })
            .map(|_| ())
    }
}
