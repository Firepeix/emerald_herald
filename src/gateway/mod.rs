use std::{path::PathBuf, collections::HashMap, sync::Arc};
use axum::extract::Query;
use color_eyre::{Result, eyre::eyre};
use reqwest::{Response, Method};

use self::{response::ProxyResponse, request::ProxyRequest, guard::Guardian};

pub mod request;
pub mod response;
pub(crate) mod guard;

pub async fn route_to(endpoint: &str, request: ProxyRequest, guardian: Guardian) -> Result<ProxyResponse> {
    let token = request.headers.get("Authorization").map(|h| h.to_str().expect("Authorization ser um UTF-8 valido"));
    if let Some(response) = guardian.guard(token).await {
        return Ok(response);
    }

    route(to_url(endpoint, request.path.clone())?, request).await
}

pub async fn route(url: String, request: ProxyRequest) -> Result<ProxyResponse> {
    let response = reqwest::Client::new()
        .request(request.method, url)
        .headers(request.headers)
        .body(request.body)
        .query(&map_query(request.query))
        .send()
        .await?;
     proxy(response).await
}

fn map_query(query: Query<HashMap<String, String>>) -> HashMap<String, String> {
    query.0
}

pub async fn proxy(response: Response) -> Result<ProxyResponse> {
    let status = &response.status();
    let headers = response.headers().clone();
    let body = &response.text().await?;
    Ok(ProxyResponse::new(body.clone(), status.into(), headers))
}

fn to_url(endpoint: &str, path: PathBuf) -> Result<String> {
    let path_url = path.to_str().ok_or(eyre!("Não foi possivel encontrar caminho"))?.to_string();
    Ok(format!("{endpoint}/{path_url}"))
}