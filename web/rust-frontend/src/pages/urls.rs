use yew::prelude::*;

use crate::components::new_url::NewUrl;
use crate::components::url_list::UrlList;
use crate::components::user_list::UserList;
use crate::router::{AppRoute, Link};

pub struct Urls {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Urls {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Rusty shortener!"}</h1>
                <NewUrl></NewUrl>
                <UrlList></UrlList>
                <UserList></UserList>
                <Link route=AppRoute::Login>{"Log out"}</Link><br/>
                <Link route=AppRoute::AddUser>{"Add user"}</Link>
            </div>
        }
    }
}
