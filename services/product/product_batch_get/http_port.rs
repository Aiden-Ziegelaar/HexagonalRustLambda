use crate::domain::product_get_batch_core;

use error::HexagonalError;
use http::{Error, Response, StatusCode};
use http_port_tools::port_objects::HttpPortRequest;
use models::models::product::Product;
use models::models::product::ProductRepositoryPort;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct ProductBatchGetResponse {
    products: Vec<Product>,
}

pub async fn product_get_batch_post_http_port<T1: ProductRepositoryPort>(
    product_repository_port: &T1,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let ids = match http_request.query_string_parameters.all("id") {
        Some(value) => value,
        None => {
            let err = HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "at least one id is required".to_string(),
                trace: "".to_string(),
            };
            return Ok(err.compile_to_http_response());
        }
    };
    let string_ids = ids.iter().map(|id| id.to_string()).collect();
    match product_get_batch_core(product_repository_port, &string_ids).await {
        Ok(products) => {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ProductBatchGetResponse { products }).unwrap());
            Ok(resp.unwrap())
        }
        Err(err) => Ok(err.compile_to_http_response()),
    }
}
