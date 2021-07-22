use self::diesel::prelude::*;
use crate::data::diesel_pg::Db;
use crate::data::schema::users;
use rocket_sync_db_pools::diesel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, JsonSchema, Debug, AsChangeset, Queryable, Insertable, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct AppUser {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}
