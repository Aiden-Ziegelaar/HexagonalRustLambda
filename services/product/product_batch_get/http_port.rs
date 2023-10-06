use crate::domain::product_get_batch_core;

use lazy_static::lazy_static;
use jsonschema::{Draft, JSONSchema};
use http::{Error, Response, StatusCode};
use http_apigw_adaptor::HttpPortRequest;
use models::models::product::ProductRepositoryPort;
use serde::{Serialize, Deserialize};
use serde_json::json;
use http_port_tools::http_payload_decoder;
use models::models::product::Product;

lazy_static! {
    static ref PRODUCT_BATCH_GET_SCHEMA: JSONSchema = {
        let schema = json!({
            "type": "object",
            "properties": {
                "product_ids": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
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

#[derive(Deserialize, Debug)]
struct ProductBatchGetRequest {
    product_ids: Vec<String>,
}

#[derive(Serialize, Debug)]
struct ProductBatchGetResponse {
    products: Vec<Product>,
}

pub async fn product_get_batch_post_http_port<T1: ProductRepositoryPort>(
    product_repository_port: &T1,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let payload = http_request.payload;
    let product_batch_get_request: ProductBatchGetRequest = http_payload_decoder!(ProductBatchGetRequest, PRODUCT_BATCH_GET_SCHEMA, payload);
    match product_get_batch_core(product_repository_port, &product_batch_get_request.product_ids).await {
        Ok(products) => {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ProductBatchGetResponse { products }).unwrap());
            Ok(resp.unwrap())
        },
        Err(err) => Ok(err.compile_to_http_response()),
    }
}
