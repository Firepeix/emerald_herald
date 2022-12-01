use std::{slice::Iter, path::PathBuf};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Application {
    name: String,
    url: String,
    unauthenticated_routes: Vec<PathBuf>
}

impl Application {
    pub fn new(name: String, url: String, unauthenticated_routes: Vec<PathBuf>) -> Self {
        Application { name, url, unauthenticated_routes }
    }

    pub fn domain(&self) -> String {
        self.name.to_lowercase()
    }

    pub fn endpoint(&self) -> &str {
        &self.url
    }

    pub fn is_unauthenticaded(&self, route: &PathBuf) -> bool {
        self.unauthenticated_routes.contains(route)
    }
}

#[derive(Clone, Deserialize)]
pub struct Applications(pub Vec<Application>);

impl Applications {
    pub fn iter(&self) -> Iter<'_, Application> {
        self.0.iter()
    }
}