mod DTOs;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

#[get("/")]
fn index() -> Json<DTOs::testing_DTO::TestingDTO> {
    Json(DTOs::testing_DTO::TestingDTO {
        reply: "Main API Route :)".to_string(),
    })
}

#[post("/authenticate")]
fn authenticate_user() -> Json<DTOs::user_DTO::UserDTO> {
    Json(DTOs::user_DTO::UserDTO {
        id: 1,
        username: "kjkardum!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, authenticate_user])
}
