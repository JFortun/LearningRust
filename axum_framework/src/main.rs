use std::net::SocketAddr;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    let address = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}