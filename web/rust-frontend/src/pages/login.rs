use crate::BASE_URL;
use serde::Deserialize;
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::Request;
use yew::services::fetch::{FetchService, FetchTask, Response};
use yew::services::storage::{Area, StorageService};
use yew::web_sys;

#[derive(Deserialize, Debug, Clone)]
struct LoginResult {
    reply: String,
    status: String,
}

pub struct Login {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    username: String,
    password: String,
    fetch_error: String,
    storage: StorageService,
}

pub enum Msg {
    Login,
    ReceiveResponse(Result<LoginResult, anyhow::Error>),
    UpdateUsername(String),
    UpdatePassword(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            username: "".to_string(),
            password: "".to_string(),
            fetch_error: "".to_string(),
            storage: StorageService::new(Area::Local).expect("storage was disabled by the user"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login => {
                let request_body =
                    json!({"username": &(self.username), "password": &(self.password)});
                let request = Request::post(BASE_URL.to_owned() + "api/account/authenticate")
                    .header("Content-Type", "application/json")
                    .body(Json(&request_body))
                    .expect("Could not build that request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<LoginResult, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                self.fetch_error = "loading...".to_string();
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                true
            }
            Msg::ReceiveResponse(response) => {
                let data = match response {
                    Ok(result) => result,
                    Err(_) => LoginResult {
                        reply: "unkown error".to_string(),
                        status: "error".to_string(),
                    },
                };
                if &data.status == "error" {
                    self.fetch_error = data.reply;
                    return true;
                } else {
                    self.storage.store("jwt", Ok(data.reply));
                    self.fetch_task = None;
                    web_sys::window().unwrap().location().assign("/").unwrap();
                    return true;
                }
            }
            Msg::UpdateUsername(username) => {
                self.username = username;
                false
            }
            Msg::UpdatePassword(password) => {
                self.password = password;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Rusty shortener!"}</h1>
                <input type="text" placeholder="username" name="username" class="login-form-input" oninput=self.link.callback(|event: InputData| Msg::UpdateUsername(event.value))/><br/>
                <input type="password" placeholder="password" name="password" class="login-form-input" oninput=self.link.callback(|event: InputData| Msg::UpdatePassword(event.value))/><br/>
                <button class="login_button" onclick=self.link.callback(|_| Msg::Login)>{"Login"}</button>
                <p style="text-color: red;">{if &(self.fetch_error) != "" {self.fetch_error.clone()} else {"If you are new user, the password you type will be set as your new password".to_string()}}</p>
            </div>
        }
    }
}
