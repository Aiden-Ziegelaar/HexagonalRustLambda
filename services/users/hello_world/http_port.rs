use crate::domain::hello_world_core;
use http::{Error, Response, StatusCode};
use http_port_tools::port_objects::HttpPortRequest;

pub async fn hello_world_get_http_port(
    request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let who = request.query_string_parameters.first("who");
    let result = hello_world_core(who).await;
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(result.unwrap());
    Ok(resp.unwrap())
}
