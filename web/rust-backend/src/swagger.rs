use rocket::Route;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

fn get_docs() -> SwaggerUIConfig {
    use rocket_okapi::swagger_ui::UrlObject;

    SwaggerUIConfig {
        url: "/swagger/openapi.json".to_string(),
        urls: vec![
            UrlObject::new("Account", "/api/account/openapi.json"),
            UrlObject::new("Users", "/api/users/openapi.json"),
            UrlObject::new("Urls", "/api/urls/openapi.json"),
            UrlObject::new("Main", "/openapi.json"),
        ],
        ..Default::default()
    }
}

pub fn new() -> impl Into<Vec<Route>> {
    make_swagger_ui(&get_docs())
}
