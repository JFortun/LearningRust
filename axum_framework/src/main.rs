use std::net::SocketAddr;
use axum::{routing::get, Router};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root)).layer(cors);

    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
