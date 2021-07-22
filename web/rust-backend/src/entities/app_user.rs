use crate::data::schema::users;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(
    Serialize,
    Deserialize,
    JsonSchema,
    Debug,
    AsChangeset,
    Identifiable,
    PartialEq,
    Queryable,
    Insertable,
    Clone,
)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct AppUser {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}
