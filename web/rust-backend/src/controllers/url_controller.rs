use crate::DTOs::{
    new_url_DTO::NewUrlDTO, response_DTO::ResponseDTO, url_DTO::UrlDTO, user_DTO::UserDTO,
};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};

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
fn get_urls(user: UserDTO) -> Json<Vec<UrlDTO>> {
    Json(vec![UrlDTO {
        id: 1,
        long_url: "kjkardum.com/projects#my-project".to_string(),
        short_url: "gj9f74".to_string(),
        uses: 5,
    }])
}

// Generate new short url from specified long url
//
// Returns UrlDTO with short url
#[openapi(tag = "Url")]
#[post("/", format = "json", data = "<new_url>")]
fn add_url(new_url: Json<NewUrlDTO>, user: UserDTO) -> Json<UrlDTO> {
    Json(UrlDTO {
        id: 1,
        long_url: "kjkardum.com/projects#my-project".to_string(),
        short_url: "gj9f74".to_string(),
        uses: 5,
    })
}

// Get urls from user using id
//
// Returns Vector with Urls
#[openapi(tag = "Url")]
#[get("/<user_id>")]
fn get_urls_by_user_id(user_id: i32, user: UserDTO) -> Json<Vec<UrlDTO>> {
    Json(vec![UrlDTO {
        id: 1,
        long_url: "kjkardum.com/projects#my-project".to_string(),
        short_url: "gj9f74".to_string(),
        uses: 5,
    }])
}

// Get urls from user using username
//
// Returns Vector with Urls
#[openapi(tag = "Url")]
#[get("/<username>", rank = 2)]
fn get_urls_by_username(username: &str, user: UserDTO) -> Json<Vec<UrlDTO>> {
    Json(vec![UrlDTO {
        id: 1,
        long_url: "kjkardum.com/projects#my-project".to_string(),
        short_url: "gj9f74".to_string(),
        uses: 5,
    }])
}

// Deletes url with specified id from database
//
// Returns string response with status and description
#[openapi(tag = "Url")]
#[delete("/<id>")]
fn delete_url(id: i32, user: UserDTO) -> Json<ResponseDTO> {
    Json(ResponseDTO {
        reply: "Successfully removed url from the database".to_string(),
        status: "Success",
    })
}
