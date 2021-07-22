use self::diesel::prelude::*;
use crate::data::diesel_pg::Db;
use crate::data::schema::users;
use crate::entities::app_user::AppUser;
use crate::helpers::mapper;
use crate::services::token_service;
use crate::DTOs::{
    login_DTO::LoginDTO, response_DTO::ResponseDTO, user_DTO::UserDTO, user_repository,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};
use rocket_sync_db_pools::diesel;

pub fn get_endpoints() -> Vec<Route> {
    routes_with_openapi![authenticate_user]
}

//Authenticate user with username and password
//
//If new user doesn't yet have password, then requested password is set as new password
#[openapi(tag = "Account")]
#[post("/authenticate", format = "json", data = "<credentials>")]
async fn authenticate_user(connection: Db, credentials: Json<LoginDTO>) -> Json<ResponseDTO> {
    let login_data = credentials.into_inner();
    let mut error = "Bad request";
    if let Ok(app_user) = AppUser::find_by_username(&login_data.username, &connection).await {
        if &app_user.password_hash == "None" {
            if let Ok(hashed) = hash(login_data.password, DEFAULT_COST) {
                let update = AppUser {
                    id: app_user.id,
                    password_hash: hashed,
                    ..app_user
                };
                return Json(ResponseDTO {
                    reply: AppUser::update(app_user.id.unwrap(), update, &connection)
                        .await
                        .to_string(),
                    status: "success",
                });
            } else {
                error = "Cannot calculate password hash!"
            }
        } else if let Ok(bcrypt_result) = verify(&login_data.password, &app_user.password_hash) {
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
