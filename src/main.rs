use axum::{Router, extract::Extension};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    serve().await
}

async fn serve() {
    color_eyre::install().expect("Não foi possivel instalar color eyre!");
    let state = Arc::new(emerald_herald::install().expect("Não foi possivel instalar configurações!"));
    let routes = emerald_herald::routes(state.clone()).expect("Não foi possivel criar rotas!");
    let app = Router::new().merge(routes).layer(Extension(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    tracing::info!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
