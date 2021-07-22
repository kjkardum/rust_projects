use crate::entities::app_user::AppUser;
use self::diesel::prelude::*;
use crate::data::diesel_pg::Db;
use crate::data::schema::users;
use rocket_sync_db_pools::diesel;
//Finds first occurence of db user entity with same username field and returns "Ok(AppUser)"
//
//Retunrs string Err if not found
pub async fn find_by_username(username: &str, connection: &Db) -> Result<AppUser, &'static str> {
    let username_clone = username.to_string();
    if let Ok(app_user) = connection.run(move |conn| {
        users::table
            .filter(users::username.eq(username_clone))
            .first(conn)
    }).await {
        return Ok(app_user);
    }
    Err("Couldn't find user")
}
