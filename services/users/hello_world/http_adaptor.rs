mod domain;
mod http_port;
use crate::http_port::hello_world_get_http_port;

use http_apigw_adaptor::{HttpPortRequest, HttpPortResponse};

use lambda_adaptor::common_lambda_adaptor;
use lambda_http::{run, service_fn, Error, IntoResponse};

async fn http_lambda_driving_adaptor(
    event: lambda_http::Request,
) -> Result<lambda_http::Response<lambda_http::Body>, Error> {
    let http_request = HttpPortRequest::from(event);
    let generic_http_response = hello_world_get_http_port(http_request).await.unwrap();
    let lambda_http_response = HttpPortResponse(generic_http_response)
        .into_response()
        .await;
    Ok(lambda_http_response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    common_lambda_adaptor!();

    run(service_fn(http_lambda_driving_adaptor)).await
}
