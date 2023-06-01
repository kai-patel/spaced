use activitypub_federation::{
    fetch::object_id::ObjectId, kinds::actor::PersonType, protocol::public_key::PublicKey,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::DbUser;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: ObjectId<DbUser>,
    #[serde(rename = "type")]
    pub kind: PersonType,
    pub preferred_username: String,
    pub name: String,
    pub inbox: Url,
    pub outbox: Url,
    pub public_key: PublicKey,
    pub idx: i32
}
