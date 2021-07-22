use crate::data::schema::urls;
use crate::entities::app_user::AppUser;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    JsonSchema,
    Debug,
    AsChangeset,
    Queryable,
    Insertable,
    Clone,
    Associations,
    Identifiable,
    PartialEq,
)]
#[serde(rename_all = "camelCase")]
#[belongs_to(AppUser, foreign_key = "user_id")]
#[table_name = "urls"]
pub struct Url {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub long_url: String,
    pub short_url: String,
    pub uses: i32,
}
