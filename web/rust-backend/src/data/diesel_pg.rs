use okapi::openapi3::{Object, Responses, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket_sync_db_pools::diesel;
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
    response::OpenApiResponder,
};
use rocket::Response;

use self::diesel::prelude::*;

#[database("my_db")]
pub struct Db(diesel::PgConnection);

//Runs all migrations on startup
async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("src/data/migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel PG Stage", |rocket| async {
        rocket.attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}

impl<'a, 'r> OpenApiFromRequest<'a> for Db {
    fn request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
impl<'a, 'r: 'a> OpenApiResponder<'a, 'r> for Db {
    fn responses(_: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let responses = Responses::default();
        Ok(responses)
    }
}

impl<'a, 'r: 'a> rocket::response::Responder<'a, 'r> for Db {
    fn respond_to(self, _: &rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        Ok(Response::new())
    }
}