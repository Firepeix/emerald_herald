use std::{path::PathBuf};

use rocket::{Route, get, routes};

use crate::gateway::{self, responder::ProxiedResponder};

#[get("/ebisu/<path..>")]
async fn redirect(path: PathBuf) -> ProxiedResponder {
    gateway::route_to("http://192.168.0.11:3001", path, gateway::Method::Get).await.unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![
        redirect
    ]
}