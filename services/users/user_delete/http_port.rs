use crate::domain::user_delete_core;

use error::HexagonalError;
use eventing::EventingPort;
use http::{Error, Response, StatusCode};
use http_port_tools::port_objects::HttpPortRequest;
use models::models::user::UserRepositoryPort;

pub async fn user_delete_delete_http_port<T1: UserRepositoryPort, T2: EventingPort>(
    user_repository_port: &T1,
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
    match user_delete_core(user_repository_port, eventing_port, &username.to_string()).await {
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
