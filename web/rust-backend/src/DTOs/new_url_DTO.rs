use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//Data transfer object for adding new short urls to database
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewUrlDTO {
    pub long_url: String,
}
