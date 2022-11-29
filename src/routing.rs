use std::{path::PathBuf, collections::HashMap};

use axum::{
    routing::{get, MethodRouter},
    http::HeaderMap,
    response::IntoResponse,
    Router, extract::{Path, Query}, body::Bytes,
};
use tracing::info;

use crate::{gateway::{self, request::{ProxyRequest, ExtractMethod}, guard::Guardian}, applications::Application};

async fn redirect(
    ExtractMethod(method): ExtractMethod, 
    body: Bytes, 
    query: Query<HashMap<String, String>>, 
    Path(path): Path<String>, 
    headers: HeaderMap,
    state: Application,
    guardian: Guardian
) -> impl IntoResponse {
    let request = ProxyRequest {
        path: PathBuf::from(&path),
        method,
        headers,
        body,
        query
    };
    log(state.domain(), path, &request);
    gateway::route_to(state.endpoint(), request, guardian).await.unwrap()
}

fn log(application: String, path: String, request: &ProxyRequest) {
    let method = &request.method.to_string();
    info!(application, path, method, "New Request")
}

pub fn router(app: Application, guardian: Guardian) -> Router {
    let path = format!("/{}/*path", app.domain());
    Router::new().route(&path, default_routes(app, guardian))
}

fn default_routes(app: Application, guardian: Guardian) -> MethodRouter {
    let service =  {
        move |
        method, 
        path, 
        body, 
        query, 
        headers
        |  redirect(method, path, body, query, headers, app, guardian)
    };

     get(service.clone())
    .post(service.clone())
    .put(service.clone())
    .delete(service.clone())
    .head(service.clone())
    .trace(service.clone())
    .options(service.clone())
    .patch(service)
}