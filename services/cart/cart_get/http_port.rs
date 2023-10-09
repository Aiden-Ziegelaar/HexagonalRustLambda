use crate::domain::cart_get_core;

use http::{Error, Response, StatusCode};
use http_apigw_adaptor::HttpPortRequest;
use models::models::cart::CartRepositoryPort;
use serde_json::json;

pub async fn cart_get_get_http_port<T1: CartRepositoryPort>(
    cart_repository_port: &T1,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let email = match http_request.query_string_parameters.first("id") {
        Some(value) => value,
        None => {
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(
                    json! {
                        {
                            "error": "id is required"
                        }
                    }
                    .to_string(),
                );
            return Ok(resp.unwrap());
        }
    };
    match cart_get_core(cart_repository_port, email.to_string()).await {
        Ok(result) => {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&result).unwrap());
            Ok(resp.unwrap())
        }
        Err(err) => {
            println!("Error: {}", err);
            Ok(err.compile_to_http_response())
        }
    }
}
