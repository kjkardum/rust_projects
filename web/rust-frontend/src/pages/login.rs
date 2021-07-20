use yew::prelude::*;

use crate::router::{AppRoute, Link};

pub struct Login {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Login {
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
            <div>
                <h1>{"Rusty shortener!"}</h1>
                <input type="text" placeholder="username" name="username" class="login-form-input"/><br/>
                <input type="password" placeholder="password" name="password" class="login-form-input"/><br/>
                <Link route=AppRoute::Urls><button class="login_button">{"Login"}</button></Link><br/>
            </div>
        }
    }
}
