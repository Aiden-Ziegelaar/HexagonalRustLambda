use std::future::Future;
use std::pin::Pin;

pub struct HttpPortResponse<T>(pub http::Response<T>);

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

#[macro_export]
macro_rules! http_lambda_driving_adaptor {
    ($x:ident) => {
        use lambda_adaptor::lambda_driving_adaptor;
        use lambda_http::{run, service_fn, Error, IntoResponse, RequestExt};
        use query_map::QueryMap;
        
        async fn http_lambda_driving_adaptor(
            event: lambda_http::Request,
        ) -> Result<lambda_http::Response<lambda_http::Body>, Error> {
            let blank_query_map = QueryMap::default();
            let path_parameters_ref = event.path_parameters_ref().unwrap_or(&blank_query_map);
            let query_string_parameters_ref = event
                .query_string_parameters_ref()
                .unwrap_or(&blank_query_map);
            let payload = match event.body() {
                lambda_http::Body::Empty => None,
                lambda_http::Body::Text(s) => Some(s.clone()),
                lambda_http::Body::Binary(b) => Some(String::from_utf8(b.clone()).unwrap()),
            };
            let generic_http_response =
                $x(path_parameters_ref, query_string_parameters_ref, &payload)
                    .await
                    .unwrap();
            let lambda_http_response = HttpPortResponse(generic_http_response)
                .into_response()
                .await;
            Ok(lambda_http_response)
        }

        lambda_driving_adaptor!(http_lambda_driving_adaptor);
    };
}
