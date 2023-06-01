use activitypub_federation::{
    axum::json::FederationJson,
    config::{Data, FederationConfig, FederationMiddleware},
    protocol::context::WithContext,
    traits::Object,
    FEDERATION_CONTENT_TYPE,
};
use std::{net::SocketAddr, sync::Arc};

use axum::{
    self,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing,
};

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
) -> Result<impl IntoResponse, impl IntoResponse> {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        let Ok(db_user) = data.read_local_user(&name).await else { return Err(StatusCode::BAD_REQUEST) };
        let Ok(json_user) = db_user.into_json(&data).await else { return Err(StatusCode::BAD_REQUEST) };

        Ok(FederationJson(WithContext::new_default(json_user)))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
