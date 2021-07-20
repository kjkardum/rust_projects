use yew::prelude::*;

use crate::router::{AppRoute, Link};

pub struct UrlList {
    link: ComponentLink<Self>,
    url_list: Vec<(String, String, i32)>,
}

pub enum Msg {}

impl Component for UrlList {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let url_list: Vec<(String, String, i32)> = vec![
            (
                "kjkardum.com".to_string(),
                "kjkardum.com/u/8fa7e9".to_string(),
                1,
            ),
            (
                "google.com".to_string(),
                "kjkardum.com/u/3s45v8".to_string(),
                2,
            ),
        ];
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
impl UrlList {
    fn view_item(&self, (fromUrl, toUrl, Id): &(String, String, i32)) -> Html {
        html! {
            <li>
                {fromUrl}{" -> "}{toUrl}
            </li>
        }
    }
}
