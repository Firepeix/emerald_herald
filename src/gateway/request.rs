//endpoint: &str, path: PathBuf, method: Method, headers: HeaderMap

use std::{path::PathBuf, collections::HashMap};

use axum::{http::HeaderMap, body::Bytes, extract::Query, http::StatusCode};

use super::Method;

pub struct ProxyRequest{
    pub path: PathBuf,
    pub method: Method,
    pub headers: HeaderMap,
    pub body: Bytes,
    pub query: Query<HashMap<String, String>>
}

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

pub struct ExtractMethod(pub Method);

#[async_trait]
impl<B> FromRequest<B> for ExtractMethod
where B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
       Ok(ExtractMethod(request.method().clone()))
    }
}