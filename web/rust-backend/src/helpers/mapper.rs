use crate::data::diesel_pg::Db;
use crate::entities::{app_user::AppUser, url::Url};
use crate::services::token_service;
use crate::DTOs::{user_DTO::UserDTO, url_DTO::UrlDTO};
use futures::stream::{self, StreamExt};

//Mapper from db user entity to User data transfer object
//
//Resulting UserDTO can then be encoded in JWT and returned as response to client
pub fn to_user(input: &AppUser) -> UserDTO {
    UserDTO {
        id: input.id.unwrap(),
        username: input.username.clone(),
        is_admin: input.is_admin,
        iat: token_service::time_in_secs(),
        eat: token_service::time_in_secs() + token_service::WEEK,
    }
}

pub async fn to_url(input: &Url, connection: &Db) -> UrlDTO {
    UrlDTO {
        id: input.id.unwrap(),
        long_url: input.long_url.to_string(),
        short_url: input.short_url.to_string(),
        owner: AppUser::find_by_id(input.user_id.unwrap(), connection)
            .await
            .unwrap()
            .username.to_string(),
        uses: input.uses,
    }
}

pub async fn to_url_vec(list: Vec<Url>, connection: &Db) -> Vec<UrlDTO> {
    stream::iter(list)
        .filter_map(|input| async move { Some(to_url(&input, connection).await) })
        .collect().await
}
