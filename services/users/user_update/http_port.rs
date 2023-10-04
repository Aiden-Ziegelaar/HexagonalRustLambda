use crate::domain::user_update_core;

use error::HexagonalError;
use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_apigw_adaptor::HttpPortRequest;
use http_port_tools::http_payload_decoder;
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use models::models::user::{MutableUser, UserRepositoryPort};
use serde_json::json;

lazy_static! {
    static ref USER_SCHEMA: JSONSchema = {
        let schema = json!({
            "type": "object",
            "properties": {
                "first": {
                    "type": "string"
                },
                "last": {
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

pub async fn user_update_put_http_port<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let email = match http_request.query_string_parameters.first("email") {
        Some(value) => value,
        None => {
            let err = HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "email is required".to_string(),
                trace: "".to_string(),
            };
            return Ok(err.compile_to_http_response());
        }
    };
    let payload = http_request.payload;
    let user_updates = http_payload_decoder!(MutableUser, USER_SCHEMA, payload);
    match user_update_core(user_repository_port, eventing_port, &email.to_string(), user_updates).await {
        Ok(user) => {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&user).unwrap());
            Ok(resp.unwrap())
        }
        Err(err) => Ok(err.compile_to_http_response()),
    }
}
