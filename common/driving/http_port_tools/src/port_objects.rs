use std::future::Future;
use std::pin::Pin;

use lambda_http::RequestExt;

pub struct HttpPortResponse<T>(pub http::Response<T>);

pub struct HttpPortRequest {
    pub path_parameters: query_map::QueryMap,
    pub query_string_parameters: query_map::QueryMap,
    pub payload: Option<String>,
    pub headers: http::HeaderMap,
}

impl lambda_http::IntoResponse for HttpPortResponse<String> {
    fn into_response(
        self,
    ) -> Pin<Box<(dyn Future<Output = lambda_http::Response<lambda_http::Body>> + Send + 'static)>>
    {
        let (parts, body) = self.0.into_parts();
        let body = lambda_http::Body::from(body);
        Box::pin(async move { lambda_http::Response::from_parts(parts, body) })
    }
}

impl lambda_http::IntoResponse for HttpPortResponse<()> {
    fn into_response(
        self,
    ) -> Pin<Box<(dyn Future<Output = lambda_http::Response<lambda_http::Body>> + Send + 'static)>>
    {
        let (parts, body) = self.0.into_parts();
        let body = lambda_http::Body::from(body);
        Box::pin(async move { lambda_http::Response::from_parts(parts, body) })
    }
}

impl lambda_http::IntoResponse for HttpPortResponse<Vec<u8>> {
    fn into_response(
        self,
    ) -> Pin<Box<(dyn Future<Output = lambda_http::Response<lambda_http::Body>> + Send + 'static)>>
    {
        let (parts, body) = self.0.into_parts();
        let body = lambda_http::Body::from(body);
        Box::pin(async move { lambda_http::Response::from_parts(parts, body) })
    }
}

impl From<lambda_http::Request> for HttpPortRequest {
    fn from(request: lambda_http::Request) -> Self {
        let body = match request.body() {
            lambda_http::Body::Empty => String::new(),
            lambda_http::Body::Text(s) => s.to_string(),
            lambda_http::Body::Binary(_) => String::new(),
        };
        HttpPortRequest {
            path_parameters: request.path_parameters().clone(),
            query_string_parameters: request.query_string_parameters().clone(),
            payload: Some(body),
            headers: request.headers().clone(),
        }
    }
}
