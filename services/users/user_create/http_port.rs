use crate::domain::user_create_core;

use http::{Error, Response, StatusCode};
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use models::models::user::User;
use query_map::QueryMap;
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
                },
                "email": {
                    "type": "string"
                },
                "username": {
                    "type": "string"
                }
            },
            "required": [
                "first",
                "last",
                "email",
                "username"
            ]
        });
        JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .unwrap()
    };
}

pub async fn user_create_post_http_port(
    _path_params: &QueryMap,
    _query_params: &QueryMap,
    payload: &str,
) -> Result<Response<()>, Error> {
    let payload_json = serde_json::from_str::<serde_json::Value>(payload).unwrap();
    let result = USER_SCHEMA.validate(&payload_json);
    match result {
        Ok(_) => {}
        Err(e) => {
            e.enumerate().for_each(|x| {
                println!("Validation error: {}", x.1);
            });
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(());
            return Ok(resp.unwrap());
        }
    }
    let user = serde_json::from_str::<User>(payload).unwrap();
    match user_create_core(user).await {
        Ok(_) => {
            let resp = Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(());
            Ok(resp.unwrap())
        },
        Err(_) => {
            let resp = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(());
            Ok(resp.unwrap())
        },
    }
}
