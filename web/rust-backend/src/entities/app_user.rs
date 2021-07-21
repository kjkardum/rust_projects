use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppUser {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}
