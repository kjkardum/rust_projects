use yew::prelude::*;

use crate::router::{AppRoute, Link};

pub struct NewUrl {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for NewUrl {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <input type="text" placeholder="long url" name="url" class="login-form-input"/><br/>
                <Link route=AppRoute::Urls><button class="login_button">{"New URL"}</button></Link><br/>
            </>
        }
    }
}
