use std::{
    env,
    sync::{Arc, Mutex},
};

use activitypub_federation::{config::Data, traits::Object};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use url::Url;

use crate::models::DbUser;
use crate::person;

pub type DatabaseHandle = Arc<Database>;

pub struct Database {
    db_conn: Mutex<PgConnection>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            db_conn: Database::establish_connection(),
        }
    }

    fn establish_connection() -> Mutex<PgConnection> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
        Mutex::new(
            PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Could not connect to database")),
        )
    }

    #[allow(unused)]
    pub async fn read_local_user(&self, name: &str) -> Result<DbUser, anyhow::Error> {
        todo!();
    }
}

#[allow(unused)]
#[async_trait::async_trait]
impl Object for DbUser {
    type DataType = DatabaseHandle;

    type Kind = person::Person;

    type Error = anyhow::Error;

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        todo!();
    }

    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        todo!();
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        todo!();
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        todo!();
    }
}
