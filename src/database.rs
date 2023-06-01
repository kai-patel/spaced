use std::{
    env,
    sync::{Arc, Mutex},
};

use activitypub_federation::{config::Data, protocol::public_key::PublicKey, traits::Object};
use diesel::{
    Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use dotenvy::dotenv;
use url::Url;

use crate::person;
use crate::{models::DbUser, person::Person};

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

    pub async fn read_local_user(&self, query: &str) -> Result<DbUser, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let mut lock = self.db_conn.lock().unwrap();

        let result = users
            .filter(name.eq(query))
            .limit(1)
            .select(DbUser::as_select())
            .first(&mut *lock);

        result
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
        let mut lock = data.db_conn.lock().unwrap();
        let obj_as_str = object_id.as_str();
        use crate::schema::users::dsl::*;

        let result = users
            .filter(id.eq(obj_as_str))
            .select(DbUser::as_select())
            .first(&mut *lock);

        match result {
            Ok(r) => Ok(Some(r)),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Person {
            id: Url::parse(&self.id.to_string()).unwrap().into(),
            kind: Default::default(),
            preferred_username: self.display_name,
            name: self.name,
            inbox: Url::parse(&self.inbox).unwrap(),
            outbox: Url::parse(&self.outbox).unwrap(),
            public_key: todo!(), //self.public_key(),
            idx: self.idx,
        })
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
