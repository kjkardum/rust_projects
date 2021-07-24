use crate::baseUrl;
use crate::models::{url_model::UrlModel, user::User};
use crate::pages::urls::Urls;
use serde_json::json;
use yew::format::Json;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::web_sys;

use crate::router::{AppAnchor, AppRoute};

pub struct NewUrl {
    link: ComponentLink<Self>,
    props: NewUrlProps,
    fetch_task: Option<FetchTask>,
    long_url: String,
}
#[derive(Properties, Clone)]
pub struct NewUrlProps {
    pub user: User,
    pub token: String,
}

pub enum Msg {
    AddUrl,
    ReceiveResponse(Result<UrlModel, anyhow::Error>),
    UpdateLongUrl(String),
}

impl Component for NewUrl {
    type Message = Msg;
    type Properties = NewUrlProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            fetch_task: None,
            long_url: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateLongUrl(long_url) => {
                self.long_url = long_url;
                false
            }
            Msg::AddUrl => {
                let request_body = json!({"longUrl": &(self.long_url)});
                let request = Request::post(baseUrl.to_owned() + "api/urls/")
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer ".to_owned() + &self.props.token)
                    .body(Json(&request_body))
                    .expect("Could not build that request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<UrlModel, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::ReceiveResponse(response) => {
                self.fetch_task = None;
                self.long_url = "".to_string();
                web_sys::window().unwrap().location().reload();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <input
                    type="text"
                    placeholder="long url"
                    name="url"
                    class="login-form-input"
                    value=self.long_url.to_string()
                    oninput=self.link.callback(|event: InputData| Msg::UpdateLongUrl(event.value))/><br/>
                <button
                    class="add_button"
                    onclick=self.link.callback(|_| Msg::AddUrl)>
                        <i class="fa fa-plus" aria-hidden="true"></i>
                </button>
            </>
        }
    }
}
