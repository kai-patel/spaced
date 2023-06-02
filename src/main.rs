use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use std::{net::SocketAddr, sync::Arc};

use axum::{self, routing};

mod follow;
mod database;
mod http;
mod person;
mod models;
mod schema;

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
        .route("/user/:name", routing::get(http::http_get_user))
        .route(".well-known/webfinger", routing::get(http::webfinger))
        .layer(FederationMiddleware::new(config));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
