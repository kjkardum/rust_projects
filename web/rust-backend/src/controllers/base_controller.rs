use crate::DTOs::{response_DTO::ResponseDTO, user_DTO::UserDTO};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};
use crate::data::diesel_pg::Db;
use crate::entities::url::Url;

pub fn get_endpoints() -> Vec<Route> {
    routes_with_openapi![index, shortened]
}

//Base route
//
//For jwt testing purposes
#[openapi(tag = "Default")]
#[get("/")]
fn index(_user: UserDTO) -> Json<ResponseDTO> {
    return Json(ResponseDTO {
        reply: "Hello World!".to_string(),
        status: "success",
    });
}

//Redirect route
//
//Redirects from shortened to original url
#[openapi(skip)]
#[get("/u/<short>")]
async fn shortened(connection: Db, short: &str) -> Result<Redirect, &'static str> {
    if let Ok(db_urls) = Url::find_by_short(short, &connection).await {
        return Ok(Redirect::to(db_urls.long_url));
    }
    Err("Cannot find specified url!")
}