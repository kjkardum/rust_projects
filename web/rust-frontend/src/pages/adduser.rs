use yew::prelude::*;

use crate::router::{AppRoute, Link};

pub struct AddUser {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for AddUser {
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
                <h1>{"Give someone access!"}</h1>
                <input type="text" placeholder="new username" name="username" class="login-form-input"/><br/>
                <Link route=AppRoute::Urls><button class="login_button">{"Add user"}</button></Link><br/>
            </div>
        }
    }
}
