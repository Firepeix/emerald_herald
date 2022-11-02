use std::path::PathBuf;
use color_eyre::{Result, eyre::eyre};
use reqwest::Response;
use rocket::http::Status;

use self::responder::ProxiedResponder;

pub mod responder;

pub enum Method {
    Get
}


pub async fn route_to(endpoint: &str, path: PathBuf, method: Method) -> Result<ProxiedResponder> {
    let url = to_url(endpoint, path)?;
    let response = match method {
        Method::Get => route_with_get(url).await
    };
    // TODO Adicionar wrap de retornar não importa o resultado
    Ok(response.unwrap())
}

pub async fn route_with_get(url: String) -> Result<ProxiedResponder> {
    let response = reqwest::get(url)
    .await?;

    proxy(response).await
}

pub async fn proxy(response: Response) -> Result<ProxiedResponder> {
    let status = &response.status();
    let body = response.text().await?;
    Ok(ProxiedResponder::new(body, Status::new(status.as_u16())))
}

fn to_url(endpoint: &str, path: PathBuf) -> Result<String> {
    let path_url = path.to_str().ok_or(eyre!("Não foi possivel encontrar caminho"))?.to_string();
    Ok(format!("{endpoint}/{path_url}"))
}