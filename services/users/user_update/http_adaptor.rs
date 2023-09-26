mod domain;
mod http_port;
use crate::http_port::user_create_post_http_port;

use http_apigw_adaptor::{http_lambda_driving_adaptor, HttpPortResponse};
http_lambda_driving_adaptor!(user_create_post_http_port);
