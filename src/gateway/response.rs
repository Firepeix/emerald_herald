use axum::{
    http::{StatusCode, HeaderMap},
    response::{IntoResponse}
};

pub struct ProxyResponse {
    body: String,
    status: StatusCode,
    headers: HeaderMap
}

impl ProxyResponse {
    pub fn new(body: String, status: StatusCode, headers: HeaderMap) -> Self {
        ProxyResponse { body, status, headers }
    }
}

impl IntoResponse for ProxyResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = (self.status, self.body).into_response();
        *response.headers_mut() = self.headers;
        response
    }
}