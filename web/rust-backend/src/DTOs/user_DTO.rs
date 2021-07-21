use crate::services::token_service;
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub is_admin: bool,
    pub iat: u64,
    pub eat: u64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserDTO {
    type Error = status::Custom<&'static str>;
    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<Self, status::Custom<&'static str>> {
        if let Some(authentication_header) = request.headers().get_one("Authorization") {
            let auth_header_string = authentication_header.to_string();
            if auth_header_string.starts_with("Bearer ") {
                let token = auth_header_string[7..auth_header_string.len()].trim();
                println!("{:?}", token);
                if let Ok(token_data) = token_service::verify_token(token) {
                    return Outcome::Success(token_data);
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(Status::Unauthorized, "You must log in")),
        )
    }
}
