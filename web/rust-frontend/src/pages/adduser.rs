use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::web_sys;

use crate::router::{AppAnchor, AppRoute};

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

                <AppAnchor route=AppRoute::Urls>
                    <button class="login_button">{"Add user"}</button>
                </AppAnchor><br/>
            </div>
        }
    }
}
