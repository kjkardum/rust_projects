use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//Simple response data transfer object for returning generic responses
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDTO {
    pub reply: String,
    pub status: &'static str,
}
