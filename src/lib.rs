use std::path::PathBuf;
use color_eyre::Result;
use gateway::Method;
use rocket::Route;
use tracing::error;

mod log;
mod ebisu;
mod gateway;

pub fn install() -> Result<()> {
    log::install()?;
    Ok(())
}

pub fn routes() -> Vec<Route> {
    ebisu::routes()
}

pub async fn route_to(endpoint: &str, path: PathBuf, method: Method) {
    if let Err(report) = gateway::route_to(endpoint, path, method).await {
        error!("{}", report.to_string())
    }
}