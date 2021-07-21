use crate::DTOs::{response_DTO::ResponseDTO, user_DTO::UserDTO};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;

pub fn get_endpoints() -> Vec<Route> {
    routes![index]
}

#[get("/")]
fn index(user: Result<UserDTO, status::Custom<&'static str>>) -> Json<ResponseDTO> {
    if let Err(e) = user {
        return Json(ResponseDTO {
            reply: "Not logged in!".to_string(),
            status: "error",
        });
    }
    Json(ResponseDTO {
        reply: "Main API Route :)".to_string(),
        status: "success",
    })
}
