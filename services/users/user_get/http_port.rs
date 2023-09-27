use crate::domain::user_get_core;

use http::{Error, Response, StatusCode};
use query_map::QueryMap;
use serde_json::json;

pub async fn user_get_get_http_port(
    _path_params: &QueryMap,
    query_params: &QueryMap,
    _payload: &Option<String>,
) -> Result<Response<String>, Error> {
    let email = match query_params.first("email") {
        Some(value) => value,
        None => {
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(json!{
                    {
                        "error": "email is required"
                    }
                }.to_string());
            return Ok(resp.unwrap());
        }
    };
    match user_get_core(email.to_string()).await {
        Ok(user) => {
            match user {
                Some(result) => {
                    let resp = Response::builder()
                        .status(StatusCode::OK)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&result).unwrap());
                    Ok(resp.unwrap())
                },
                None => {
                    let resp = Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("content-type", "application/json")
                        .body("".to_string());
                    Ok(resp.unwrap())
                },
            }
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
