use crate::models::user::User;
use yew::prelude::*;

use crate::router::{AppAnchor, AppRoute};

pub struct UserList {
    link: ComponentLink<Self>,
    props: UserProps,
    url_list: Vec<(String, i32)>,
}
#[derive(Properties, Clone)]
pub struct UserProps {
    pub user: User,
    pub token: String,
}

pub enum Msg {}

impl Component for UserList {
    type Message = Msg;
    type Properties = UserProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let url_list: Vec<(String, i32)> =
            vec![("kjkardum".to_string(), 1), ("anoniman".to_string(), 2)];
        Self {
            link,
            props,
            url_list,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.props.user.is_admin {
            html! {
                <>
                    <h4>{"Current users"}</h4>
                    <ul>
                        { self.url_list.clone().iter().map(|e| self.view_item(e)).collect::<Html>() }
                    </ul>
                </>
            }
        } else {
            html! {
                <h4>{"Can't view other users - not admin"}</h4>
            }
        }
    }
}
impl UserList {
    fn view_item(&self, (username, Id): &(String, i32)) -> Html {
        html! {
            <li class="li">
                {username}{" with ID: "}{Id} <button class="li-button">{"✕"}</button>
            </li>
        }
    }
}
