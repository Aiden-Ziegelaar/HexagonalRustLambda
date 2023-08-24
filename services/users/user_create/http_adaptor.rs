mod domain;
mod http_port;
use crate::http_port::user_create_post_http_port;

use http_apigw_adaptor::{http_lambda_driving_adaptor, HttpPortResponse};
use lambda_adaptor::lambda_driving_adaptor;
use lambda_http::{run, service_fn, Error, IntoResponse, RequestExt};
use query_map::QueryMap;

http_lambda_driving_adaptor!(user_create_post_http_port);
