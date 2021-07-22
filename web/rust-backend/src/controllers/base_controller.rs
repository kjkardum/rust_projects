use crate::DTOs::{response_DTO::ResponseDTO, user_DTO::UserDTO};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{openapi, routes_with_openapi};

pub fn get_endpoints() -> Vec<Route> {
    routes_with_openapi![index]
}

//Base route
//
//For jwt testing purposes
#[openapi(tag = "Default")]
#[get("/")]
fn index(user: UserDTO) -> Json<ResponseDTO> {
    return Json(ResponseDTO {
        reply: "Hello World!".to_string(),
        status: "success",
    });
}
