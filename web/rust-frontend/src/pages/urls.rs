use crate::components::new_url::NewUrl;
use crate::components::url_list::UrlList;
use crate::components::user_list::UserList;
use crate::models::user::User;
use crate::router::{AppAnchor, AppRoute};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::web_sys;

pub struct Urls {
    link: ComponentLink<Self>,
    props: UrlsProps,
    storage: StorageService,
}

#[derive(Properties, Clone)]
pub struct UrlsProps {
    pub user: User,
    pub token: String,
}

pub enum Msg {
    LogOut,
}

impl Component for Urls {
    type Message = Msg;
    type Properties = UrlsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            storage: StorageService::new(Area::Local).expect("storage was disabled by the user"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LogOut => {
                self.storage.remove("jwt");
                web_sys::window().unwrap().location().assign("/login");
                true
            }
        }
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
                <h1 class="title">{"Rusty shortener!"}</h1>
                <NewUrl user=self.props.user.clone() token=self.props.token.to_string()></NewUrl>
                <UrlList user=self.props.user.clone() token=self.props.token.to_string()></UrlList>
                <UserList user=self.props.user.clone() token=self.props.token.to_string()></UserList>
                <button class="login_button" onclick=self.link.callback(|_| Msg::LogOut)>
                    {"Log out"}
                </button>
                <AppAnchor route=AppRoute::AddUser>
                    <button class="login_button">
                        {"Add user"}
                    </button>
                </AppAnchor><br/>
            </div>
        }
    }
}
