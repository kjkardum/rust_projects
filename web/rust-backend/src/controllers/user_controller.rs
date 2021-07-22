use crate::entities::app_user::AppUser;
use crate::DTOs::{new_user_DTO::NewUserDTO, response_DTO::ResponseDTO, user_DTO::UserDTO};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};

pub fn get_endpoints() -> Vec<Route> {
    routes_with_openapi![add_user, get_users, get_user, delete_user]
}

// Add new user to DB with specified username
//
// Returns db entity for new user
#[openapi(tag = "User")]
#[post("/", format = "json", data = "<new_user>")]
fn add_user(new_user: Json<NewUserDTO>, user: UserDTO) -> Json<AppUser> {
    Json(AppUser {
        id: 1,
        username: "kjkardum".to_string(),
        password_hash: "$2b$12$dpGY2pc/bA3RF60c1mm68OPyTecYTxlrp3fes6AfSTBC7Pn431o4K".to_string(),
        is_admin: true,
    })
}

// Get list of all users in application
//
// Returns vector of users "AppUser"
#[openapi(tag = "Users")]
#[get("/")]
fn get_users(user: UserDTO) -> Json<Vec<AppUser>> {
    Json(vec![AppUser {
        id: 1,
        username: "kjkardum".to_string(),
        password_hash: "$2b$12$dpGY2pc/bA3RF60c1mm68OPyTecYTxlrp3fes6AfSTBC7Pn431o4K".to_string(),
        is_admin: true,
    }])
}

// Get user with specified Id
//
// If found returns the user
#[openapi(tag = "User")]
#[get("/<id>")]
fn get_user(id: i32, user: UserDTO) -> Json<AppUser> {
    Json(AppUser {
        id: 1,
        username: "kjkardum".to_string(),
        password_hash: "$2b$12$dpGY2pc/bA3RF60c1mm68OPyTecYTxlrp3fes6AfSTBC7Pn431o4K".to_string(),
        is_admin: true,
    })
}

// Deletes user with specified id from database
//
// Returns string response with status and description
#[openapi(tag = "User")]
#[delete("/<id>")]
fn delete_user(id: i32, user: UserDTO) -> Json<ResponseDTO> {
    Json(ResponseDTO {
        reply: "Successfully removed user from the database".to_string(),
        status: "Success",
    })
}
