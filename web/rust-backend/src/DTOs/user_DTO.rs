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
use rocket::response::status;
use rocket::Response;
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
impl<'a, 'r> OpenApiFromRequest<'a> for UserDTO {
    fn request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut security_req = SecurityRequirement::new();
        // each security requirement needs a specific key in the openapi docs
        security_req.insert("JWT_security".into(), Vec::new());

        // The scheme for the security needs to be defined as well
        // https://swagger.io/docs/specification/authentication/basic-authentication/
        let security_scheme = SecurityScheme {
            description: Some("Requires JWToken to access".into()),
            // scheme identifier is the keyvalue under which this security_scheme will be filed in
            // the openapi.json file
            scheme_identifier: "FixedKeyApiKeyAuth".into(),
            // this will show where and under which name the value will be found in the HTTP header
            // in this case, the header key x-api-key will be searched
            // other alternatives are "query", "cookie" according to the openapi specs.
            // [link](https://swagger.io/specification/#security-scheme-object)
            // which also is where you can find examples of how to create a JWT scheme for example
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".into(),
                location: "header".into(),
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