use std::sync::Arc;

use axum::{Router};
use color_eyre::{Result, eyre::eyre};
use gateway::{request::ProxyRequest};
use management::{State};
use tracing::error;

mod log;
mod routing;
mod gateway;
mod management;
mod applications;
mod date;

pub fn install() -> Result<State> {
    log::install()?;
    management::install_state()
}

pub fn routes(state: Arc<State>) -> Result<Router> {
    Ok(state.apps()
    .iter()
    .map(|app| routing::router(app.clone(), state.guardian().clone()))
    .reduce(|router: Router, router_b: Router| router.merge(router_b))
    .unwrap_or_default())
}

//pub async fn route_to(endpoint: &str, request: ProxyRequest) {
//    if let Err(report) = gateway::route_to(endpoint, request).await {
//        error!("{}", report.to_string())
//    }
//}