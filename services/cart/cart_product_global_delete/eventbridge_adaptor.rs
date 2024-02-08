mod domain;
mod event_port;

use crate::event_port::cart_product_deleted_event_port;
use eventing::events::product::product_deleted::EventProductDeletedV1;

use aws_lambda_events::cloudwatch_events::CloudWatchEvent;
use lambda_adaptor::common_lambda_adaptor;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use models::models::cart::CartRepositoryPort;

async fn eventbridge_lambda_driving_adaptor<T1: CartRepositoryPort>(
    cart_repository_port: &T1,
    event: LambdaEvent<CloudWatchEvent<EventProductDeletedV1>>,
) -> Result<(), Error> {
    let internal_event = event.payload.detail.unwrap();
    cart_product_deleted_event_port(cart_repository_port, internal_event)
        .await
        .unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Common snippit from all lambda functions
    common_lambda_adaptor!();
    {
        // Provision required repositories once in the main function
        let sdk_credential_meta_repository =
            sdk_credential_meta_repository::SdkCredentialsMetaRepository::new().await;
        let dynamo_db_repository = persistance_repository::DynamoDBSingleTableRepository::new(
            &sdk_credential_meta_repository,
        );
        let cart_repository =
            models::models::cart::CartRepositoryAdaptor::new(&dynamo_db_repository);

        run(service_fn(|event| {
            eventbridge_lambda_driving_adaptor(&cart_repository, event)
        }))
        .await
    }
}
