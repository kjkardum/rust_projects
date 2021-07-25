use crate::models::user::{User, UserEntity};
use crate::router::{AppAnchor, AppRoute};
use crate::BASE_URL;
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::web_sys;

pub struct AddUser {
    link: ComponentLink<Self>,
    props: AddUserProps,
    fetch_task: Option<FetchTask>,
    new_username: String,
}
#[derive(Properties, Clone)]
pub struct AddUserProps {
    pub user: User,
    pub token: String,
}

pub enum Msg {
    AddUsername,
    ReceiveResponse(Result<UserEntity, anyhow::Error>),
    UpdateUsername(String),
}

impl Component for AddUser {
    type Message = Msg;
    type Properties = AddUserProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            fetch_task: None,
            new_username: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(new_username) => {
                self.new_username = new_username;
                false
            }
            Msg::AddUsername => {
                let request_body = json!({"username": &(self.new_username)});
                let request = Request::post(BASE_URL.to_owned() + "api/users/")
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer ".to_owned() + &self.props.token)
                    .body(Json(&request_body))
                    .expect("Could not build that request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<UserEntity, anyhow::Error>>>| {
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
                self.new_username = "".to_string();
                if let Ok(_) = response {
                    web_sys::window().unwrap().location().assign("/").unwrap();
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Give someone access!"}</h1>
                <input
                    type="text"
                    placeholder="new username"
                    name="username"
                    class="login-form-input"
                    value=self.new_username.to_string()
                    oninput=self.link.callback(|event: InputData| Msg::UpdateUsername(event.value))/><br/>

                <button
                    class="login_button"
                    onclick=self.link.callback(|_| Msg::AddUsername)>{"Add user"}</button>
                <AppAnchor route=AppRoute::Urls>
                    <button class="login_button">{"Go back"}</button>
                </AppAnchor><br/><br/>
            </div>
        }
    }
}
