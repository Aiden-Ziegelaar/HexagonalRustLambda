use crate::domain::user_username_update_core;

use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_apigw_adaptor::HttpPortRequest;
use http_port_tools::http_payload_decoder;
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use models::models::user::UserRepositoryPort;
use serde::{Deserialize, Serialize};
use serde_json::json;

lazy_static! {
    static ref USER_SCHEMA: JSONSchema = {
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {
                    "type": "string"
                },
                "username": {
                    "type": "string"
                }
            },
            "required": [
                "email",
                "username"
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
struct UserUsernameUpdate {
    email: String,
    username: String,
}

pub async fn user_username_update_put_http_port<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
    eventing_port: &T2,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let payload = http_request.payload;
    let user_updates = http_payload_decoder!(UserUsernameUpdate, USER_SCHEMA, payload);
    match user_username_update_core(
        user_repository_port,
        eventing_port,
        user_updates.email,
        user_updates.username,
    )
    .await
    {
        Ok(user) => {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&user).unwrap());
            Ok(resp.unwrap())
        }
        Err(_) => {
            let resp = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body("".to_string());
            Ok(resp.unwrap())
        }
    }
}
