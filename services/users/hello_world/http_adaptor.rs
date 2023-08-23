mod http_port;
mod domain;
use crate::http_port::hello_world_get_http_port;

use query_map::QueryMap;
use lambda_http::{run, service_fn, Error, RequestExt, IntoResponse};
use http_apigw_adaptor::{ http_lambda_driving_adaptor, HttpPortResponse };
use lambda_adaptor::lambda_driving_adaptor;

http_lambda_driving_adaptor!(hello_world_get_http_port);
