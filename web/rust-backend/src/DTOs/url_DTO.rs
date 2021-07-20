use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UrlDTO {
    pub id: i32,
    pub long_url: String,
    pub short_url: String,
    pub uses: i32,
}
