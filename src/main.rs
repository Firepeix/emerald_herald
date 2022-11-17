use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    serve().await
}

async fn serve() {
    color_eyre::install().expect("Não foi possivel instalar color eyre!");
    emerald_herald::install().expect("Não foi possivel instalar configurações!");

    let app = Router::new().merge(emerald_herald::routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    tracing::info!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}