use crate::domain::cart_clear_delete_core;

use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_port_tools::port_objects::HttpPortRequest;
use models::models::cart::CartRepositoryPort;
use serde_json::json;

pub async fn cart_create_post_http_port<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let email = match http_request.path_parameters.first("username") {
        Some(value) => value,
        None => {
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(
                    json! {
                        {
                            "error": "username is required"
                        }
                    }
                    .to_string(),
                );
            return Ok(resp.unwrap());
        }
    };
    match cart_clear_delete_core(cart_repository_port, eventing_port, email.to_string()).await {
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
