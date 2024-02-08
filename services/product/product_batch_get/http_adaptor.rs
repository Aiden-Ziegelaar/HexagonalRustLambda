mod domain;
mod http_port;
use crate::http_port::product_get_batch_post_http_port;

use http_port_tools::port_objects::{HttpPortRequest, HttpPortResponse};

use lambda_adaptor::common_lambda_adaptor;
use lambda_http::{run, service_fn, Error, IntoResponse};
use models::models::product::ProductRepositoryPort;

async fn http_lambda_driving_adaptor<T1: ProductRepositoryPort>(
    product_repository_port: &T1,
    event: lambda_http::Request,
) -> Result<lambda_http::Response<lambda_http::Body>, Error> {
    let http_request = HttpPortRequest::from(event);
    let generic_http_response =
        product_get_batch_post_http_port(product_repository_port, http_request)
            .await
            .unwrap();
    let lambda_http_response = HttpPortResponse(generic_http_response)
        .into_response()
        .await;
    Ok(lambda_http_response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Common snippit from all lambda functions
    common_lambda_adaptor!();

    // Provision required repositories once in the main function
    let sdk_credential_meta_repository =
        sdk_credential_meta_repository::SdkCredentialsMetaRepository::new().await;
    let dynamo_db_repository =
        persistance_repository::DynamoDBSingleTableRepository::new(&sdk_credential_meta_repository);
    let product_repository =
        models::models::product::ProductRepositoryAdaptor::new(&dynamo_db_repository);

    run(service_fn(|event| {
        http_lambda_driving_adaptor(&product_repository, event)
    }))
    .await
}
