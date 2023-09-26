use crate::domain::user_delete_core;

use http::{Error, Response, StatusCode};
use jsonschema::{Draft, JSONSchema};
use lazy_static::lazy_static;
use query_map::QueryMap;
use serde_json::json;

lazy_static! {
    static ref USER_EMAIL_SCHEMA: JSONSchema = {
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {
                    "type": "string"
                }
            },
            "required": [
                "email"
            ],
            "additionalProperties": false
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
    payload: &Option<String>,
) -> Result<Response<String>, Error> {
    match payload {
        None => {
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body("".to_string());
            return Ok(resp.unwrap());
        }
        Some(_) => {}
    }
    let payload_str = payload.clone().unwrap();
    let payload_json = serde_json::from_str::<serde_json::Value>(&payload_str).unwrap();
    let result = USER_EMAIL_SCHEMA.validate(&payload_json);
    match result {
        Ok(_) => {}
        Err(e) => {
            e.enumerate().for_each(|x| {
                println!("Validation error: {}", x.1);
            });
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body("".to_string());
            return Ok(resp.unwrap());
        }
    }
    match user_delete_core(payload_json["email"].to_string()).await {
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
