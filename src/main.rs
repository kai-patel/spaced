use serde::{Deserialize, Serialize};
use url::Url;
use activitypub_federation::{fetch::object_id::ObjectId, traits::tests::DbUser, protocol::public_key::PublicKey, kinds::actor::PersonType, config::FederationConfig};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Person {
    id: ObjectId<DbUser>,
    #[serde(rename="type")]
    kind: PersonType,
    preferred_username: String,
    name: String,
    inbox: Url,
    outbox: Url,
    public_key: PublicKey
}

fn main() -> anyhow::Result<()> {
    let db_conn = todo!();

    let config = FederationConfig::builder()
        .domain("0.0.0.0")
        .app_data(db_conn)
        .build()?;

}
