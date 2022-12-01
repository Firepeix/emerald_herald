use std::{env, collections::HashMap};

use crate::{applications::{Applications, Application}, gateway::guard::Guardian};

use color_eyre::{Result};
use tracing::warn;

const APPLICATION_MAP_KEY: &str = "APPLICATIONS";
const GUARDIAN_URL_KEY: &str = "GUARDIAN_URL";

pub struct State {
    applications: Applications,
    guardian: Guardian
}

impl State {
    pub fn apps(&self) -> &Applications {
        &self.applications
    }

    pub fn guardian(&self) -> &Guardian {
        &self.guardian
    }
}

pub(crate) fn install_state() -> Result<State> {
    Ok(create_state(decode_env(read_env(APPLICATION_MAP_KEY)), read_env(GUARDIAN_URL_KEY)))
}

fn read_env(env_name: &str) -> String {
    match env::var(env_name) {
        Ok(serialized_apps) => serialized_apps,
        Err(error) => {
            match error {
                env::VarError::NotPresent => {
                    warn!(env_name, "Env Não foi encontrado ambiente");
                    String::from("[]")
                },
                env::VarError::NotUnicode(_) => {
                    warn!(env_name, "Env Não esta em padrão UNICODE");
                    String::from("[]")
                },
            }
        }
    }
}

fn decode_env(applications: String) -> Applications {
    match serde_json::from_str(&applications) {
        Ok(decoded) => decoded,
        Err(error) => {
            warn!(applications, "Não foi possivel deserializar env APPLICATIONS error = {}", error);
            Applications(vec![])
        }
    }
}

fn create_state(apps: Applications, guardian_url: String) -> State {
    State {
        applications: apps,
        guardian: Guardian::new(guardian_url)
    }
}