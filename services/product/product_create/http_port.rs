use crate::domain::product_create_core;

use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_port_tools::http_payload_decoder;
use http_port_tools::port_objects::HttpPortRequest;
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use models::models::product::{Product, ProductRepositoryPort};
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
            "required": [
                "product_name",
                "price_cents",
                "description"
            ],
            "additionalProperties": false
        });
        JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .unwrap()
    };
}

pub async fn product_create_post_http_port<T1: ProductRepositoryPort, T2: EventingPort>(
    product_repository_port: &T1,
    eventing_port: &T2,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let payload = http_request.payload;
    let product = http_payload_decoder!(Product, PRODUCT_SCHEMA, payload);
    match product_create_core(product_repository_port, eventing_port, product).await {
        Ok(result) => {
            let resp = Response::builder()
                .status(StatusCode::CREATED)
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
