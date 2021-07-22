use crate::entities::app_user::AppUser;
use crate::services::token_service;
use crate::DTOs::user_DTO::UserDTO;

//Mapper from db user entity to User data transfer object
//
//Resulting UserDTO can then be encoded in JWT and returned as response to client
pub fn to_user(input: &AppUser) -> UserDTO {
    UserDTO {
        id: input.id,
        username: input.username.clone(),
        is_admin: input.is_admin,
        iat: token_service::time_in_secs(),
        eat: token_service::time_in_secs() + token_service::WEEK,
    }
}
