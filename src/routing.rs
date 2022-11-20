use std::{path::PathBuf, collections::HashMap, sync::Arc};

use axum::{
    routing::{get, MethodRouter},
    http::HeaderMap,
    response::IntoResponse,
    Router, extract::{Path, Query}, body::Bytes, Extension,
};
use tracing::info;

use crate::{gateway::{self, request::{ProxyRequest, ExtractMethod}}, management::{State}, applications::Application};

async fn redirect(
    ExtractMethod(method): ExtractMethod, 
    body: Bytes, 
    query: Query<HashMap<String, String>>, 
    Path(path): Path<String>, 
    headers: HeaderMap,
    state: Arc<Application>
) -> impl IntoResponse {
    let request = ProxyRequest {
        path: PathBuf::from(&path),
        method,
        headers,
        body,
        query
    };
    log(path, &request);
    gateway::route_to(state.endpoint(), request).await.unwrap()
}

fn log(path: String, request: &ProxyRequest) {
    let method = &request.method.to_string();
    info!(path, method, "New Request")
}

pub fn router(app: Arc<Application>) -> Router {
    let path = format!("/{}/*path", app.domain());
    Router::new().route(&path, default_routes(app))
}

fn default_routes(app: Arc<Application>) -> MethodRouter {
    let service =  {
        move |
        method, 
        path, 
        body, 
        query, 
        headers
        |  redirect(method, path, body, query, headers, app)
    };

     get(service.clone())
    .post(service.clone())
    .put(service.clone())
    .delete(service.clone())
    .head(service.clone())
    .trace(service.clone())
    .patch(service)
}