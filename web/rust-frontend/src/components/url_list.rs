use crate::models::{url_model::UrlModel, user::User};
use crate::BASE_URL;
use yew::format::Json;
use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct UrlList {
    link: ComponentLink<Self>,
    props: UrlProps,
    url_list: Vec<UrlModel>,
    fetch_task: Option<FetchTask>,
}
#[derive(Properties, Clone)]
pub struct UrlProps {
    pub user: User,
    pub token: String,
}

pub enum Msg {
    Get,
    ReceiveResponse(Result<Vec<UrlModel>, anyhow::Error>),
    Remove(i32),
    RemoveResponse,
}

impl Component for UrlList {
    type Message = Msg;
    type Properties = UrlProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let admin_or_username: String;
        if !props.user.isAdmin {
            admin_or_username = props.user.username.to_string()
        } else {
            admin_or_username = "".to_string()
        };
        let request = Request::get(BASE_URL.to_owned() + "api/urls/" + &admin_or_username)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer ".to_owned() + &props.token)
            .body(Nothing)
            .expect("Could not build that request.");

        let callback = link.callback(
            |response: Response<Json<Result<Vec<UrlModel>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            link,
            props,
            url_list: vec![],
            fetch_task: Some(task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Get => {
                let admin_or_username: String;
                if !self.props.user.isAdmin {
                    admin_or_username = self.props.user.username.to_string()
                } else {
                    admin_or_username = "".to_string()
                };
                let request = Request::get(BASE_URL.to_owned() + "api/urls/" + &admin_or_username)
                    .header("Content-Type", "application/json")
                    .header("Authorization", "Bearer ".to_owned() + &self.props.token)
                    .body(Nothing)
                    .expect("Could not build that request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<UrlModel>, anyhow::Error>>>| {
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
                self.url_list = data;
                true
            }
            Msg::Remove(id) => {
                let request = Request::delete(BASE_URL.to_owned() + "api/urls/" + &id.to_string())
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
        html! {
            <>
                <h4>{"Existing urls"}</h4>
                <ul>
                    { self.url_list.clone().iter().map(|item| self.view_item(item) ).collect::<Html>() }
                </ul>
            </>
        }
    }
}

impl UrlList {
    fn view_item(&self, item: &UrlModel) -> Html {
        let id = item.id;
        html! {
            <li class="li">
                <a href={BASE_URL.to_owned() + "/u/" + &item.shortUrl}>{&item.shortUrl}</a>{" ⟶ "}<a href={item.longUrl.to_string()}>{&item.longUrl}</a>
                <button class="li-button" disabled=!self.props.user.isAdmin onclick=self.link.callback(move |_| Msg::Remove(id))>{"✕"}</button>
            </li>
        }
    }
}
