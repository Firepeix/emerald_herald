use std::{env, collections::HashMap};

use crate::applications::{Applications, Application};

use color_eyre::{Result};
use tracing::warn;

const APPLICATION_MAP_KEY: &str = "APPLICATIONS";

pub struct State {
    applications: Applications
}

impl State {
    pub fn apps(&self) -> &Applications {
        &self.applications
    }
}

pub(crate) fn install_state() -> Result<State> {
    Ok(create_state(decode_env(read_env())))
}

fn read_env() -> String {
    match env::var(APPLICATION_MAP_KEY) {
        Ok(serialized_apps) => serialized_apps,
        Err(error) => {
            match error {
                env::VarError::NotPresent => {
                    warn!("Não foi encontrado ambiente de APPLICATIONS");
                    String::from("[]")
                },
                env::VarError::NotUnicode(_) => {
                    warn!("APPLICATIONS não esta em padrão UNICODE");
                    String::from("[]")
                },
            }
        }
    }
}

fn decode_env(applications: String) -> Vec<HashMap<String, String>> {
    match serde_json::from_str(&applications) {
        Ok(decoded) => decoded,
        Err(error) => {
            warn!("Não foi possivel deserializar env APPLICATIONS error {}", error);
            vec![]
        }
    }
}

fn create_state(apps: Vec<HashMap<String, String>>) -> State {
    State {
        applications: Applications(
            apps.iter()
            .map(|map| Application::new(map["name"].clone(), map["url"].clone()))
            .collect()
        )
    }
}