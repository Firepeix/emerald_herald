use axum::http::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use tracing::warn;
use serde::{Serialize};

use crate::{date::DateTime};

use super::response::ProxyResponse;



#[derive(Serialize)]
struct UnauthorizedBody<'a> {
    message: &'a str,
    code: u8,
    timestamp: DateTime
}

#[derive(Clone)]
pub struct Guardian {
    url: String
}

impl Guardian {

    pub(crate) fn new(url: String) -> Self {
        Guardian { url }
    }

    pub(crate) async fn guard(self, token: Option<&str>) -> Option<ProxyResponse> {
        if token.is_none() {
            warn!(exception = "Faltando token de autenticação", "Autorização Negada");
            return Some(Guardian::unauthorized_response());
        }

        let response_result = reqwest::Client::new().get(self.url).bearer_auth(token.unwrap().replace("Bearer ", "")).send().await;
        match response_result {
            Ok(response) => { 
                if !response.status().is_success() {
                    warn!(status_code = response.status().as_u16(), "Autorização Negada");
                    return Some(Guardian::unauthorized_response())
                }
                
                None
            }
            Err(error) => {
                warn!(exception = format!("{:?}", error), "Não foi possivel comunicar com o Guardião");
                Some(Guardian::unauthorized_response())
            },
        }
    }

    fn unauthorized_response() -> ProxyResponse {
        let body = UnauthorizedBody{message: "Acesso não autorizado!", code: 5, timestamp: DateTime::now()};
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        ProxyResponse::new(serde_json::to_string(&body).expect("Fixed message"), StatusCode::UNAUTHORIZED, headers)
    }
}


