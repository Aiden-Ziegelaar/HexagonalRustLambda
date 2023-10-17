use crate::domain::cart_remove_item_core;

use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_apigw_adaptor::HttpPortRequest;
use http_port_tools::http_payload_decoder;
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use models::models::cart::{CartRepositoryPort, CartItemDelete};
use serde_json::json;

lazy_static! {
    static ref CART_ITEM_SCHEMA: JSONSchema = {
        let schema = json!({
            "type": "object",
            "properties": {
                "product_id": {
                    "type": "string"
                },
                "user_id": {
                    "type": "string"
                }
            },
            "required": [
                "product_id",
                "user_id"
            ],
            "additionalProperties": false
        });
        JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .unwrap()
    };
}

pub async fn cart_remove_item_delete_http_port<T1: CartRepositoryPort, T2: EventingPort>(
    cart_repository_port: &T1,
    eventing_port: &T2,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let payload = http_request.payload;
    let cart = http_payload_decoder!(CartItemDelete, CART_ITEM_SCHEMA, payload);
    match cart_remove_item_core(cart_repository_port, eventing_port, cart).await {
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
