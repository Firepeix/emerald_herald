use std::{path::PathBuf, collections::HashMap};

use axum::{
    routing::get,
    http::HeaderMap,
    response::IntoResponse,
    Router, extract::{Path, Query}, body::Bytes,
};
use tracing::info;

use crate::gateway::{self, request::{ProxyRequest, ExtractMethod}};

async fn redirect(ExtractMethod(method): ExtractMethod, body: Bytes, query: Query<HashMap<String, String>>, Path(path): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    let request = ProxyRequest {
        path: PathBuf::from(&path),
        method,
        headers,
        body,
        query
    };
    log(path, &request);
    gateway::route_to("http://192.168.0.11:3001", request).await.unwrap()
}

fn log(path: String, request: &ProxyRequest) {
    let method = &request.method.to_string();
    info!(path, method, "New Request")
}

pub fn router() -> Router {
    let route = get(redirect)
        .post(redirect)
        .put(redirect)
        .delete(redirect)
        .head(redirect)
        .trace(redirect)
        .patch(redirect);
    Router::new()
        .route("/ebisu/*path", route)
}