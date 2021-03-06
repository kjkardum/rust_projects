use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//Data transfer object used for authenticating users at /authenticate
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}
