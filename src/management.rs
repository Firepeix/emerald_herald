use crate::applications::{Applications, Application};

use color_eyre::{Result};

pub struct State {
    applications: Applications
}

impl State {
    pub fn apps(&self) -> &Applications {
        &self.applications
    }
}

pub(crate) fn install_state() -> Result<State> {
    let state = State {
        applications: Applications(vec![Application::new("ebisu".to_string(), "http://192.168.0.11:3001".to_string())])
    };

    Ok(state)
}