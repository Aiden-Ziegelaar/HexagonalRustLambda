use crate::domain::hello_world_core;
use http::{Error, Response, StatusCode};
use query_map::QueryMap;

pub async fn hello_world_get_http_port(
    _path_params: &QueryMap,
    query_params: &QueryMap,
    _payload: &str,
) -> Result<Response<String>, Error> {
    let who = query_params.first("who");
    let result = hello_world_core(who).await;
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(result.unwrap());
    Ok(resp.unwrap())
}
