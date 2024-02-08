pub mod events;

use async_trait::async_trait;
use aws_sdk_eventbridge::Client;

use error::HexagonalError;
use events::event_emmiter::SerialisableEvent;
use mockall::automock;
use sdk_credential_meta_repository::SdkCredentialsMetaRepository;

pub struct EventingRepository {
    pub client: Client,
    pub bus_name: String,
}

#[automock]
#[async_trait]
pub trait EventingPort {
    async fn emit<T: SerialisableEvent + Sync + 'static>(
        &self,
        event: &T,
    ) -> Result<(), HexagonalError>;
}

impl EventingRepository {
    pub fn new(
        sdk_credential_meta_repository: &SdkCredentialsMetaRepository,
    ) -> EventingRepository {
        EventingRepository {
            client: Client::new(&sdk_credential_meta_repository.sdk_config),
            bus_name: std::env::var("EVENT_BUS_NAME")
                .expect("EVENT_BUS_NAME environment variable not set"),
        }
    }
}

#[async_trait::async_trait]
impl EventingPort for EventingRepository {
    async fn emit<T: SerialisableEvent + Sync>(&self, event: &T) -> Result<(), HexagonalError> {
        let put_events_request = aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
            .set_event_bus_name(Some(self.bus_name.clone()))
            .set_detail_type(Some(event.get_event_type().clone()))
            .set_source(Some("RUSTHEXAGONALSTOREFRONT".to_string()))
            .set_detail(Some(event.serialise()))
            .build();

        self.client
            .put_events()
            .entries(put_events_request)
            .send()
            .await
            .map_err(|err| HexagonalError {
                message: format!("Unable to emit event: {}", event.get_event_type()),
                error: error::HexagonalErrorCode::AdaptorError,
                trace: err.to_string(),
            })
            .map(|value| (println!("{:?}", value)))
    }
}
