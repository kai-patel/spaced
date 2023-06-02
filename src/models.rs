use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub name: String,
    pub display_name: String,
    pub password_hash: Option<String>,
    pub email: Option<String>,
    pub federation_id: String,
    pub inbox: String,
    pub outbox: String,
    pub local: bool,
    pub public_key: String,
    pub private_key: Option<String>,
    pub last_refreshed_at: NaiveDateTime,
    pub id: String,
    pub idx: i32,
}
