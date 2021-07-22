use okapi::openapi3::{Object, Responses, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
    response::OpenApiResponder,
};
use crate::services::token_service;
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::Response;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//JWT encoded user data transfer object
//
//Extracted from Authorization header
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
    type Error = &'static str;
    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
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
            "Invalid token!"),
        )
    }
}

//Used for adding authorization to Swagger openapi.json
impl<'a, 'r> OpenApiFromRequest<'a> for UserDTO {
    fn request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut security_req = SecurityRequirement::new();
        security_req.insert("JWT_security".into(), Vec::new());
        let security_scheme = SecurityScheme {
            description: Some("Bearer JWT".into()),
            scheme_identifier: "JWT_Authorization".into(),
            data: SecuritySchemeData::Http {
                scheme: "bearer".into(),
                bearer_format: Some("JWT".into()),
            },
            extensions: Object::default(),
        };

        Ok(RequestHeaderInput::Security((
            security_scheme,
            security_req,
        )))
    }
}
impl<'a, 'r: 'a> OpenApiResponder<'a, 'r> for UserDTO {
    fn responses(_: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let responses = Responses::default();
        Ok(responses)
    }
}

impl<'a, 'r: 'a> rocket::response::Responder<'a, 'r> for UserDTO {
    fn respond_to(self, _: &rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        Ok(Response::new())
    }
}