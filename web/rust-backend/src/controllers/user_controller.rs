use crate::data::diesel_pg::Db;
use crate::entities::app_user::AppUser;
use crate::DTOs::{new_user_DTO::NewUserDTO, user_DTO::UserDTO};
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};

pub fn get_endpoints() -> Vec<Route> {
    routes_with_openapi![
        add_user,
        get_users,
        get_user,
        delete_user,
        user_opt,
        user_opt_param]
}

// Add new user to DB with specified username
//
// Returns db entity for new user
#[openapi(tag = "User")]
#[post("/", format = "json", data = "<new_user>")]
async fn add_user(connection: Db, new_user: Json<NewUserDTO>, user: UserDTO) -> Result<Json<AppUser>, &'static str> {
    let db_user = AppUser {
        id: None,
        username: new_user.into_inner().username,
        password_hash: "None".to_string(),
        is_admin: false,
    };
    let new_user: AppUser = AppUser::create(db_user, &connection).await;
    Ok(Json(new_user))
}

// Get list of all users in application
//
// Returns vector of users "AppUser"
#[openapi(tag = "Users")]
#[get("/")]
async fn get_users(connection: Db, user: UserDTO) -> Result<Json<Vec<AppUser>>, &'static str> {
    if !user.is_admin { return Err("Insufficient privileges") }
    Ok(Json(AppUser::read(&connection).await))
}

// Get user with specified Id
//
// If found returns the user
#[openapi(tag = "User")]
#[get("/<id>")]
async fn get_user(connection: Db, id: i32, user: UserDTO) -> Result<Json<AppUser>, &'static str> {
    if !user.is_admin { return Err("Insufficient privileges") }
    if let Ok(user_by_id) = AppUser::find_by_id(id, &connection).await {
        return Ok(Json(user_by_id));
    }
    Err("Cannot find user!")
}

// Deletes user with specified id from database
//
// Returns string response with status and description
#[openapi(tag = "User")]
#[delete("/<id>")]
async fn delete_user(connection: Db, id: i32, user: UserDTO) -> Result<&'static str, &'static str> {
    if !user.is_admin { return Err("Insufficient privileges") }
    if id == user.id {
        return Err("Can not delete yourself!");
    }
    if AppUser::delete(id, &connection).await {
        return Ok("Successfully removed user from the database");
    }
    return Err("Can not delete user!");
}

#[openapi(skip)] #[options("/")] fn user_opt() -> &'static str { crate::cors::DEFAULT_OPTIONS }
#[openapi(skip)] #[options("/<_>")] fn user_opt_param() -> &'static str { crate::cors::DEFAULT_OPTIONS }