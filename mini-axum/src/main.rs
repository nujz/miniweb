use std::net::SocketAddr;

use axum::{handler::get, response::Html, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("ignored");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("Hello, World!")
}
