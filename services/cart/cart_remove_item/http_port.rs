use crate::domain::cart_remove_item_core;

use error::HexagonalError;
use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_port_tools::port_objects::HttpPortRequest;
use models::models::cart::CartRepositoryPort;

pub async fn cart_remove_item_delete_http_port<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let username = match http_request.path_parameters.first("username") {
        Some(username) => username,
        None => {
            return Ok(HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "username is required".to_string(),
                trace: "".to_string(),
            }
            .compile_to_http_response())
        }
    };
    let product_id = match http_request.path_parameters.first("product_id") {
        Some(value) => value,
        None => {
            let err = HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "product_id is required".to_string(),
                trace: "".to_string(),
            };
            return Ok(err.compile_to_http_response());
        }
    };
    match cart_remove_item_core(
        cart_repository_port,
        eventing_port,
        username.to_string(),
        product_id.to_string(),
    )
    .await
    {
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
