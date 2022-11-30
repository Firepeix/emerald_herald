use axum::{
    http::{StatusCode, HeaderMap},
    response::{IntoResponse}, body::Bytes
};

pub struct ProxyResponse {
    body: Bytes,
    status: StatusCode,
    headers: HeaderMap
}

impl ProxyResponse {
    pub fn new(body: String, status: StatusCode, headers: HeaderMap) -> Self {
        Self::proxy(Bytes::from(body), status, headers)
    }

    pub fn proxy(body: Bytes, status: StatusCode, headers: HeaderMap) -> Self {
        ProxyResponse { body, status, headers }
    }
}

impl IntoResponse for ProxyResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = (self.status, self.body).into_response();
        response.headers_mut().extend(self.headers);
        response
    }
}