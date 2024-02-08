use crate::domain::cart_update_item_core;

use error::HexagonalError;
use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_port_tools::http_payload_decoder;
use http_port_tools::port_objects::HttpPortRequest;
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use models::models::cart::{CartItem, CartRepositoryPort};
use serde::{Deserialize, Serialize};
use serde_json::json;

lazy_static! {
    static ref CART_ITEM_SCHEMA: JSONSchema = {
        let schema = json!({
            "type": "object",
            "properties": {
                "quantity": {
                    "type": "integer",
                    "minimum": 1
                }
            },
            "required": [
                "quantity"
            ],
            "additionalProperties": false
        });
        JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .unwrap()
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartUpdateItemBody {
    pub quantity: u32,
}

pub async fn cart_update_item_patch_http_port<T1: CartRepositoryPort, T2: EventingPort>(
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
    let payload = http_request.payload;
    let cart_update_item_body =
        http_payload_decoder!(CartUpdateItemBody, CART_ITEM_SCHEMA, payload);
    match cart_update_item_core(
        cart_repository_port,
        eventing_port,
        CartItem {
            product_id: product_id.to_string(),
            user_id: username.to_string(),
            quantity: cart_update_item_body.quantity,
            created_at: "".to_string(), // Will be excluded in adaptor
            updated_at: "".to_string(), // Will be overwritten in adaptor
        },
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
