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

pub fn install() -> Result<State> {
    log::install()?;
    management::install_state()
}

pub fn routes(state: Arc<State>) -> Result<Router> {
    state.apps()
        .iter()
        .map(|app| routing::router(Arc::new(app.clone())))
        .reduce(|router: Router, router_b: Router| router.merge(router_b))
        .ok_or(eyre!("Não foi possivel merger as rotas"))
}

pub async fn route_to(endpoint: &str, request: ProxyRequest) {
    if let Err(report) = gateway::route_to(endpoint, request).await {
        error!("{}", report.to_string())
    }
}