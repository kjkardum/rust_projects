use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//Data transfer object for adding new users to database
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewUserDTO {
    pub username: String,
}
