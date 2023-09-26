mod domain;
mod http_port;
use crate::http_port::hello_world_get_http_port;

use http_apigw_adaptor::{http_lambda_driving_adaptor, HttpPortResponse};

http_lambda_driving_adaptor!(hello_world_get_http_port);
