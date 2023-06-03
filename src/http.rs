use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
    protocol::context::WithContext,
    traits::Object,
    FEDERATION_CONTENT_TYPE,
};
use axum::{
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use url::Url;

use crate::database::{Database, DatabaseHandle, DatabaseTrait};

pub async fn http_get_user(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<DatabaseHandle<Database>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::debug!("called http_get_user");

    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());

    if accept == Some(FEDERATION_CONTENT_TYPE) {
        let Ok(db_user) = data.read_local_user(&name) else { return Err(StatusCode::NOT_FOUND) };
        let Ok(json_user) = db_user.into_json(&data).await else { return Err(StatusCode::INTERNAL_SERVER_ERROR) };

        Ok(FederationJson(WithContext::new_default(json_user)))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<DatabaseHandle<Database>>,
) -> Result<Json<Webfinger>, StatusCode> {
    tracing::debug!("called webfinger");
    let Ok(name) = extract_webfinger_name(&query.resource, &data) else { return Err(StatusCode::BAD_REQUEST) };
    let Ok(db_user) = data.read_local_user(&name) else { return Err(StatusCode::NOT_FOUND) };
    let Ok(federation_id) = Url::parse(&db_user.federation_id) else { return Err(StatusCode::NOT_FOUND) };

    Ok(Json(build_webfinger_response(
        query.resource,
        federation_id,
    )))
}

pub async fn http_ping() -> impl IntoResponse {
    tracing::debug!("called ping");
    "pong"
}
