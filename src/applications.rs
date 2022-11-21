use std::{path::PathBuf, collections::HashMap, sync::Arc, slice::Iter};

use axum::{
    routing::{get, MethodRouter},
    http::HeaderMap,
    response::IntoResponse,
    Router, extract::{Path, Query}, body::Bytes, Extension,
};
use tracing::info;

use crate::{gateway::{self, request::{ProxyRequest, ExtractMethod}}, management::{State}};

#[derive(Clone)]
pub struct Application {
    name: String,
    url: String
}

impl Application {
    pub fn new(name: String, url: String) -> Self {
        Application { name, url }
    }

    pub fn domain(&self) -> String {
        self.name.to_lowercase()
    }

    pub fn endpoint(&self) -> &str {
        &self.url
    }
}

pub struct Applications(pub Vec<Application>);

impl Applications {
    pub fn iter(&self) -> Iter<'_, Application> {
        self.0.iter()
    }
}