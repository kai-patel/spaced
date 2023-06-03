use std::{
    env,
    sync::{Arc, Mutex},
};

use activitypub_federation::{
    config::Data,
    protocol::verification::verify_domains_match,
    traits::{Actor, Object},
};
use chrono::Local;
use diesel::{
    Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use dotenvy::dotenv;
use url::Url;

use crate::person;
use crate::{models::DbUser, person::Person};

pub struct Database {
    pub db_conn: Mutex<PgConnection>,
}

pub trait DatabaseTrait<T, U> {
    fn read_local_user(&self, query: &str) -> Result<T, U>;
    fn read_from_id(&self, query: &str) -> Result<T, U>;
    fn from_json(&self, input: &DbUser) -> Result<usize, U>;
}

pub trait DbHandler<T> {
    fn db_conn(&self) -> &Mutex<T>;
}

// #[derive(Clone)]
pub struct DatabaseHandle<T>(Arc<T>);

impl<T> DatabaseHandle<T> {
    pub fn new(db: T) -> Self {
        DatabaseHandle(Arc::new(db))
    }
}

impl Clone for DatabaseHandle<Database> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
// pub type DatabaseHandle = Arc<Database>;

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
}

#[allow(unused)]
#[async_trait::async_trait]
impl Object for DbUser {
    type DataType = DatabaseHandle<Database>;

    type Kind = person::Person;

    type Error = anyhow::Error;

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let obj_as_str = object_id.as_str();
        let result = data.read_from_id(obj_as_str);

        match result {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Person {
            id: Url::parse(&self.id).unwrap().into(),
            kind: Default::default(),
            preferred_username: self.display_name.clone(),
            name: self.name.clone(),
            inbox: Url::parse(&self.inbox).unwrap(),
            outbox: Url::parse(&self.outbox).unwrap(),
            public_key: self.public_key(),
            idx: self.idx,
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        let result = verify_domains_match(json.id.inner(), expected_domain);
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let input = DbUser {
            name: json.name,
            display_name: json.preferred_username,
            password_hash: None,
            email: None,
            federation_id: "".to_string(),
            inbox: json.inbox.to_string(),
            outbox: json.outbox.to_string(),
            local: false,
            public_key: json.public_key.public_key_pem,
            private_key: None,
            last_refreshed_at: Local::now().naive_utc(),
            id: json.id.to_string(),
            idx: json.idx,
        };

        let result = data.from_json(&input);

        match result {
            Ok(1) => Ok(input),
            Ok(_) => Err(anyhow::Error::msg("Database upserted more than one record")),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}

impl Actor for DbUser {
    fn id(&self) -> Url {
        Url::parse(&self.id).unwrap()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }

    fn inbox(&self) -> Url {
        Url::parse(&self.inbox).unwrap()
    }
}

impl DbHandler<PgConnection> for DatabaseHandle<Database> {
    fn db_conn(&self) -> &Mutex<PgConnection> {
        &self.0.db_conn
    }
}

impl DatabaseTrait<DbUser, diesel::result::Error> for DatabaseHandle<Database> {
    fn read_local_user(&self, query: &str) -> Result<DbUser, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let binding = self.db_conn();
        let mut lock = binding.lock().unwrap();

        users
            .filter(name.eq(query))
            .filter(local.eq(true))
            .select(DbUser::as_select())
            .first(&mut *lock)
    }

    fn read_from_id(&self, query: &str) -> Result<DbUser, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let binding = self.db_conn();
        let mut lock = binding.lock().unwrap();

        users
            .filter(id.eq(query))
            .select(DbUser::as_select())
            .first(&mut *lock)
    }

    fn from_json(&self, input: &DbUser) -> Result<usize, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let binding = self.db_conn();
        let mut lock = binding.lock().unwrap();

        diesel::insert_into(users)
            .values(input)
            .on_conflict(id)
            .do_update()
            .set(input)
            .execute(&mut *lock)
    }
}
