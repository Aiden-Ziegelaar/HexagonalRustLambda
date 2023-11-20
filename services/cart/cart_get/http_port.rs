use crate::domain::cart_get_core;

use error::HexagonalError;
use http::{Error, Response, StatusCode};
use http_port_tools::port_objects::HttpPortRequest;
use models::models::cart::CartRepositoryPort;

pub async fn cart_get_get_http_port<T1: CartRepositoryPort>(
    cart_repository_port: &T1,
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
    match cart_get_core(cart_repository_port, username.to_string()).await {
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
