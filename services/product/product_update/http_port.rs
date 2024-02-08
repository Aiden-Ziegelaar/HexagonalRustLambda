use crate::domain::product_update_core;

use error::HexagonalError;
use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_port_tools::http_payload_decoder;
use http_port_tools::port_objects::HttpPortRequest;
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use models::models::product::{MutableProduct, ProductRepositoryPort};
use serde_json::json;

lazy_static! {
    static ref PRODUCT_SCHEMA: JSONSchema = {
        let schema = json!({
            "type": "object",
            "properties": {
                "product_name": {
                    "type": "string"
                },
                "price_cents": {
                    "type": "integer"
                },
                "description": {
                    "type": "string"
                }
            },
            "required": [],
            "additionalProperties": false
        });
        JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .unwrap()
    };
}

pub async fn product_update_put_http_port<T1: ProductRepositoryPort, T2: EventingPort>(
    product_repository_port: &T1,
    eventing_port: &T2,
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
    let payload = http_request.payload;
    let product_updates = http_payload_decoder!(MutableProduct, PRODUCT_SCHEMA, payload);
    match product_update_core(
        product_repository_port,
        eventing_port,
        &id.to_string(),
        product_updates,
    )
    .await
    {
        Ok(product) => {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&product).unwrap());
            Ok(resp.unwrap())
        }
        Err(err) => Ok(err.compile_to_http_response()),
    }
}
