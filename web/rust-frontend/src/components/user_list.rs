use crate::models::user::{User, UserEntity};
use crate::BASE_URL;
use yew::format::Json;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
pub struct UserList {
    link: ComponentLink<Self>,
    props: UserProps,
    user_list: Vec<UserEntity>,
    fetch_task: Option<FetchTask>,
}
#[derive(Properties, Clone)]
pub struct UserProps {
    pub user: User,
    pub token: String,
}

pub enum Msg {
    Get,
    ReceiveResponse(Result<Vec<UserEntity>, anyhow::Error>),
    Remove(i32),
    RemoveResponse,
}

impl Component for UserList {
    type Message = Msg;
    type Properties = UserProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let user_list: Vec<UserEntity> = vec![];
        let fetch_task: Option<FetchTask>;
        if props.user.isAdmin {
            let request = Request::get(BASE_URL.to_owned() + "api/users/")
                .header("Content-Type", "application/json")
                .header("Authorization", "Bearer ".to_owned() + &props.token)
                .body(Nothing)
                .expect("Could not build that request.");

            let callback = link.callback(
                |response: Response<Json<Result<Vec<UserEntity>, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    Msg::ReceiveResponse(data)
                },
            );
            let task = FetchService::fetch(request, callback).expect("failed to start request");
            fetch_task = Some(task);
        } else {
            fetch_task = None;
        }
        Self {
            link,
            props,
            user_list,
            fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Get => {
                let request = Request::get(BASE_URL.to_owned() + "api/users/")
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer ".to_owned() + &self.props.token)
                    .body(Nothing)
                    .expect("Could not build that request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<UserEntity>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::ReceiveResponse(response) => {
                let data = match response {
                    Ok(result) => result,
                    Err(_) => vec![],
                };
                log::info!("data {:?}", data);
                self.fetch_task = None;
                self.user_list = data;
                true
            }
            Msg::Remove(id) => {
                let request = Request::delete(BASE_URL.to_owned() + "api/users/" + &id.to_string())
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer ".to_owned() + &self.props.token)
                    .body(Nothing)
                    .expect("Could not build that request.");

                let callback =
                    self.link
                        .callback(|_: Response<Json<Result<String, anyhow::Error>>>| {
                            Msg::RemoveResponse
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::RemoveResponse => {
                self.fetch_task = None;
                self.link.send_message(Msg::Get);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.props.user.isAdmin {
            html! {
                <>
                    <h4>{"Current users"}</h4>
                    <ul>
                        { self.user_list.clone().iter().map(|item| self.view_item(item)).collect::<Html>() }
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
    fn view_item(&self, item: &UserEntity) -> Html {
        let id = item.id;
        html! {
            <li class="li">
                {item.username.to_string()}{" with ID: "}{item.id} <button class="li-button" disabled={id==self.props.user.id} onclick=self.link.callback(move |_| Msg::Remove(id))>{"✕"}</button>
            </li>
        }
    }
}
