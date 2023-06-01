use activitypub_federation::{fetch::object_id::ObjectId, protocol::public_key::PublicKey, kinds::actor::PersonType};
use serde::{Serialize, Deserialize};
use url::Url;

use crate::database;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    id: ObjectId<database::DbUser>,
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    name: String,
    inbox: Url,
    outbox: Url,
    public_key: PublicKey,
}
