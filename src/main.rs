use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(handler));
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello world!"
}
