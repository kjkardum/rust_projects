use yew::prelude::*;

use crate::router::{AppRoute, Link};

pub struct UserList {
    link: ComponentLink<Self>,
    url_list: Vec<(String, i32)>,
}

pub enum Msg {}

impl Component for UserList {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let url_list: Vec<(String, i32)> =
            vec![("kjkardum".to_string(), 1), ("anoniman".to_string(), 2)];
        Self { link, url_list }
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
                <ul>
                    { self.url_list.clone().iter().map(|e| self.view_item(e)).collect::<Html>() }
                </ul>
            </>
        }
    }
}
impl UserList {
    fn view_item(&self, (username, Id): &(String, i32)) -> Html {
        html! {
            <li>
                {username}{" with ID: "}{Id}
            </li>
        }
    }
}
