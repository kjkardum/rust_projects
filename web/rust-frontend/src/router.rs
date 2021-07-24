use crate::pages;
use yew::{html::IntoPropValue, prelude::*, web_sys::Url};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/adduser"]
    AddUser,
    #[to = "/"]
    Urls,
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
