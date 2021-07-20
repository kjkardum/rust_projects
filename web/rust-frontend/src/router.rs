use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

pub struct AppRouter {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/adduser"]
    AddUser,
    #[to = "/"]
    Urls,
}

pub type Link = RouterAnchor<AppRoute>;

impl Component for AppRouter {
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
        let render_func = Router::render(|route: AppRoute| match route {
            AppRoute::Urls => html! { <pages::urls::Urls/>},
            AppRoute::Login => html! { <pages::login::Login/>},
            AppRoute::AddUser => html! { <pages::adduser::AddUser/>},
            _ => html! {<pages::login::Login/>},
        });

        html! {
            <main>
                <img src="https://res.cloudinary.com/practicaldev/image/fetch/s--ZneJTg4E--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://qvault.io/wp-content/uploads/2020/05/rust-social.jpg" class="rust-icon"/>
                <div class="content">
                    <Router<AppRoute, ()> render=render_func/>
                </div>
            </main>
        }
    }
}
