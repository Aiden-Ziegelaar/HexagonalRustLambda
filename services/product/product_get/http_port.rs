use crate::domain::product_get_core;

use error::HexagonalError;
use http::{Error, Response, StatusCode};
use http_apigw_adaptor::HttpPortRequest;
use models::models::product::ProductRepositoryPort;

pub async fn product_get_get_http_port<T1: ProductRepositoryPort>(
    product_repository_port: &T1,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let id = match http_request.path_parameters.first("id") {
        Some(value) => value,
        None => {
            let err = HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "id is required".to_string(),
                trace: "".to_string(),
            };
            return Ok(err.compile_to_http_response());
        }
    };
    match product_get_core(product_repository_port, &id.to_string()).await {
        Ok(product) => match product {
            Some(result) => {
                let resp = Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&result).unwrap());
                Ok(resp.unwrap())
            }
            None => {
                let resp = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("content-type", "application/json")
                    .body("".to_string());
                Ok(resp.unwrap())
            }
        },
        Err(err) => Ok(err.compile_to_http_response()),
    }
}
