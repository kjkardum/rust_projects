use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlModel {
    pub id: i32,
    pub longUrl: String,
    pub shortUrl: String,
    pub owner: String,
    pub uses: i32,
}
