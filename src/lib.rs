use std::sync::Arc;

use axum::{Router, routing::get};
use color_eyre::{Result};

use management::{State};


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
    .unwrap_or_default()
    .merge(emerald_routes()))
}

fn emerald_routes() -> Router {
    Router::new()
     .route("/health", get( || async { "up" }))
}