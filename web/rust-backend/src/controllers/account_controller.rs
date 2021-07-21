use crate::helpers::mapper;
use crate::services::token_service;
use crate::DTOs::{
    login_DTO::LoginDTO, response_DTO::ResponseDTO, user_DTO::UserDTO, user_repository,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};

pub fn get_endpoints() -> Vec<Route> {
    routes_with_openapi![authenticate_user]
}

//Authenticate user with username and password
//
//Returns JWT which expires in 7 days
#[openapi("Account")]
#[post("/authenticate", format = "json", data = "<credentials>")]
fn authenticate_user(credentials: Json<LoginDTO>) -> Json<ResponseDTO> {
    let login_data = credentials.into_inner();
    let mut error = "Bad request";
    if let Ok(app_user) = user_repository::find_by_username(&login_data.username) {
        if let Ok(bcrypt_result) = verify(&login_data.password, &app_user.password_hash) {
            //let hashed = hash(password, DEFAULT_COST)?;
            if bcrypt_result {
                let user = mapper::to_user(&app_user);
                let user_token = token_service::generate_token(&user);
                return Json(ResponseDTO {
                    reply: user_token,
                    status: "success",
                });
            } else {
                error = "Incorrect password!";
            }
        } else {
            error = "Cannot calculate password hash!";
        }
    } else {
        error = "User not found!";
    }
    return Json(ResponseDTO {
        reply: error.to_string(),
        status: "error",
    });
}
