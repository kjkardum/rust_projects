mod components;
mod models;
mod pages;
mod router;

use crate::models::user::{StrUser, User};
use jsonwebtokens as jwt;
use jwt::{raw, raw::TokenSlices, Algorithm, AlgorithmID, Verifier};
use router::{AppAnchor, AppRoute, AppRouter};
use serde_json::json;
use serde_json::value::Value;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::{route::Route, switch::Permissive};

struct Model {}

pub const baseUrl: &'static str = "http://localhost:8000/";

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <img src="https://res.cloudinary.com/practicaldev/image/fetch/s--ZneJTg4E--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://qvault.io/wp-content/uploads/2020/05/rust-social.jpg" class="rust-icon"/>
                <div class="content">
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                    />
                </div>
            </main>
        }
    }
}
impl Model {
    fn switch(switch: AppRoute) -> Html {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        match storage.restore("jwt") {
            Ok(key) => {
                let TokenSlices { claims, .. } = raw::split_token(&key).unwrap();
                let claims = raw::decode_json_token_slice(claims).unwrap();
                log::info!("{:?}", claims);
                let str_app_user: StrUser =
                    serde_json::from_value(claims).expect("Not deserializeable");
                let app_user = User {
                    id: str_app_user.id.parse::<i32>().unwrap(),
                    username: str_app_user.username.to_string(),
                    is_admin: str_app_user.is_admin.parse::<bool>().unwrap(),
                    iat: str_app_user.iat.parse::<u64>().unwrap(),
                    eat: str_app_user.eat.parse::<u64>().unwrap(),
                };
                match switch {
                    AppRoute::AddUser => {
                        html! { <pages::adduser::AddUser/> }
                    }
                    _ => {
                        html! { <pages::urls::Urls user=app_user token=key.to_string()/> }
                    }
                }
            }
            Err(_) => html! { <pages::login::Login /> },
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
