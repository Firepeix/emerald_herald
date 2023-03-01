use axum::{
    http::{StatusCode, HeaderMap, HeaderValue},
    response::{IntoResponse}, body::Bytes
};
use reqwest::header::{CONTENT_LENGTH};

pub struct ProxyResponse {
    body: Bytes,
    status: StatusCode,
    headers: HeaderMap
}

impl ProxyResponse {
    pub fn new(body: String, status: StatusCode, headers: HeaderMap) -> Self {
        Self::proxy(Bytes::from(body), status, headers)
    }

    fn headers(body: &Bytes, proxy_headers: HeaderMap) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.extend(proxy_headers);
        headers.insert(CONTENT_LENGTH, HeaderValue::from(body.len()));
        headers
    }

    pub fn proxy(body: Bytes, status: StatusCode, proxy_headers: HeaderMap) -> Self {
        let headers = Self::headers(&body, proxy_headers);
        ProxyResponse { 
            body, 
            status, 
            headers 
        }
    }
}

impl IntoResponse for ProxyResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = (self.status, self.body).into_response();
        response.headers_mut().extend(self.headers);
        response
    }
}