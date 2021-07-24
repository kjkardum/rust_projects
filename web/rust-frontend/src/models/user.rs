use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub is_admin: bool,
    pub iat: u64,
    pub eat: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrUser {
    pub id: String,
    pub username: String,
    pub is_admin: String,
    pub iat: String,
    pub eat: String,
}
