use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use database::{Database, DatabaseHandle, DatabaseTrait};
use models::DbUser;
use std::net::SocketAddr;

use axum::{self, routing};

mod database;
mod follow;
mod http;
mod models;
mod person;
mod schema;

fn app<T, U, V>(config: FederationConfig<T>) -> axum::Router
where
    T: DatabaseTrait<U, V> + std::clone::Clone + std::marker::Sync + std::marker::Send + 'static,
{
    axum::Router::new()
        .route("/user/:name", routing::get(http::http_get_user))
        .route(".well-known/webfinger", routing::get(http::webfinger))
        .layer(FederationMiddleware::new(config))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database: Database = Database::new();
    let db_handler: DatabaseHandle<Database> = DatabaseHandle::new(database);

    let config = FederationConfig::builder()
        .domain("0.0.0.0")
        .app_data(db_handler)
        .debug(true)
        .build()?;

    let app = app::<DatabaseHandle<Database>, DbUser, diesel::result::Error>(config);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {}
