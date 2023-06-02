use activitypub_federation::{fetch::object_id::ObjectId, kinds::activity::FollowType, traits::ActivityHandler, config::Data};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{models::DbUser, database::DatabaseHandle};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    pub actor: ObjectId<DbUser>,
    pub object: ObjectId<DbUser>,
    #[serde(rename = "type")]
    pub kind: FollowType,
    pub id: Url,
}

#[async_trait]
impl ActivityHandler for Follow {
    type DataType = DatabaseHandle;
    type Error = anyhow::Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    #[allow(unused)]
    async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    #[allow(unused)]
    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let actor = self.actor.dereference(data).await?;
        let followed = self.object.dereference(data).await?;

        let lock = data.db_conn.lock().unwrap();

        todo!();
        // use crate::schema::users::dsl::*;

        // let results = diesel::update(users).
    }
}
