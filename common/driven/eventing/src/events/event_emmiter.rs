#[async_trait::async_trait]
pub trait EventEmmiter {
    async fn emit<T: SerialisableEvent>(event: T);
}

pub trait SerialisableEvent {
    fn get_event_type(&self) -> String;
    fn get_version(&self) -> u32;
    fn serialise(&self) -> String;
}

pub struct EventingRepository {}
