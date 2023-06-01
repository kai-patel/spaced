use activitypub_federation::{config::Data, FEDERATION_CONTENT_TYPE, axum::json::FederationJson, protocol::context::WithContext, traits::Object};
use axum::{http::{HeaderMap, StatusCode}, extract::Path, response::IntoResponse};

use crate::database;

pub async fn http_get_user(
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
