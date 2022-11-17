use axum::{Router};
use color_eyre::{Result};
use gateway::{request::ProxyRequest};
use tracing::error;

mod log;
mod ebisu;
mod gateway;

pub fn install() -> Result<()> {
    log::install()?;
    Ok(())
}

pub fn routes() -> Router {
    ebisu::router()
}

pub async fn route_to(endpoint: &str, request: ProxyRequest) {
    if let Err(report) = gateway::route_to(endpoint, request).await {
        error!("{}", report.to_string())
    }
}