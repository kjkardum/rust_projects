use rust_backend::controllers;
use rust_backend::data::diesel_pg;
use rust_backend::swagger;

#[macro_use]
extern crate rocket;
extern crate bcrypt;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(diesel_pg::stage())
        .mount("/", controllers::base_controller::get_endpoints())
        .mount(
            "/api/account",
            controllers::account_controller::get_endpoints(),
        )
        .mount("/api/users", controllers::user_controller::get_endpoints())
        .mount("/api/urls", controllers::url_controller::get_endpoints())
        .mount("/swagger-ui", swagger::new())
}
