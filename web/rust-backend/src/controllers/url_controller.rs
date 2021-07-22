use crate::DTOs::{
    new_url_DTO::NewUrlDTO, url_DTO::UrlDTO, user_DTO::UserDTO,
};
use crate::helpers::mapper;
use crate::entities::url::Url;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};
use crate::data::diesel_pg::Db;
use crate::services::url_shortener_service;

pub fn get_endpoints() -> Vec<Route> {
    routes_with_openapi![
        get_urls,
        add_url,
        delete_url,
        get_urls_by_user_id,
        get_urls_by_username
    ]
}

// Get list of all shortened urls in database
//
// Returns vector of urls "UrlDTO"
#[openapi(tag = "Urls")]
#[get("/")]
async fn get_urls(connection: Db, user: UserDTO) -> Result<Json<Vec<UrlDTO>>, &'static str> {
    if !user.is_admin { return Err("Insufficient privileges") }
    Ok(Json(mapper::to_url_vec(Url::read(&connection).await, &connection).await))
}

// Generate new short url from specified long url
//
// Returns UrlDTO with short url
#[openapi(tag = "Url")]
#[post("/", format = "json", data = "<new_url>")]
async fn add_url(connection: Db, new_url: Json<NewUrlDTO>, user: UserDTO) -> Result<Json<UrlDTO>, &'static str> {
    let db_url = Url {
        id: None,
        long_url: new_url.into_inner().long_url,
        short_url: url_shortener_service::new(&connection).await,
        uses: 0,
        user_id: Some(user.id),
    };
    let created_url: Url = Url::create(db_url, &connection).await;
    Ok(Json(mapper::to_url(&created_url, &connection).await))
}

// Get urls from user using id
//
// Returns Vector with Urls
#[openapi(tag = "Url")]
#[get("/<user_id>")]
async fn get_urls_by_user_id(connection: Db, user_id: i32, user: UserDTO) -> Result<Json<Vec<UrlDTO>>, &'static str> {
    if !user.is_admin && user.id!=user_id { return Err("Insufficient privileges, can only read own urls") }
    if let Ok(db_urls) = Url::find_by_user_id(user_id, &connection).await {
        return Ok(Json(mapper::to_url_vec(db_urls, &connection).await));
    }
    Err("Cannot find urls for specified id!")
}

// Get urls from user using username
//
// Returns Vector with Urls
#[openapi(tag = "Url")]
#[get("/<username>", rank = 2)]
async fn get_urls_by_username(connection: Db, username: &str, user: UserDTO) -> Result<Json<Vec<UrlDTO>>, &'static str> {
    if !user.is_admin && &(user.username)!=username { return Err("Insufficient privileges, can only read own urls") }
    if let Ok(db_urls) = Url::find_by_username(username, &connection).await {
        return Ok(Json(mapper::to_url_vec(db_urls, &connection).await));
    }
    Err("Cannot find urls for specified username!")
}

// Deletes url with specified id from database
//
// Returns string response with status and description
#[openapi(tag = "Url")]
#[delete("/<id>")]
async fn delete_url(connection: Db, id: i32, user: UserDTO) -> Result<&'static str, &'static str> {
    if !user.is_admin { return Err("Insufficient privileges") }
    if Url::delete(id, &connection).await {
        return Ok("Successfully removed id from the database");
    }
    return Err("Can not delete id!");
}
