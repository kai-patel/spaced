use activitypub_federation::config::{Data, FederationConfig, FederationMiddleware};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{self, extract::Path, http::HeaderMap, response::IntoResponse, routing};

mod database;
mod person;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database = database::Database::new();
    let db_handler = Arc::new(database);

    let config = FederationConfig::builder()
        .domain("0.0.0.0")
        .app_data(db_handler)
        .debug(true)
        .build()?;

    let app = axum::Router::new()
        .route("/user/:name", routing::get(http_get_user))
        .layer(FederationMiddleware::new(config));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn http_get_user(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<database::DatabaseHandle>,
) -> impl IntoResponse {
}
