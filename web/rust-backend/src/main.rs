use rust_backend::controllers;

#[macro_use]
extern crate rocket;
extern crate bcrypt;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", controllers::base_controller::get_endpoints())
        .mount(
            "/api/account",
            controllers::account_controller::get_endpoints(),
        )
        .mount("/api/users", controllers::user_controller::get_endpoints())
        .mount("/api/urls", controllers::url_controller::get_endpoints())
}
