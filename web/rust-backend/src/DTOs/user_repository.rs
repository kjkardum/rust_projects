use crate::entities::app_user::AppUser;
use self::diesel::prelude::*;
use crate::data::diesel_pg::Db;
use crate::data::schema::users;
use rocket_sync_db_pools::diesel;

impl AppUser {

    //Finds first occurence of db user entity with same username field and returns "Ok(AppUser)"
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

    //Finds first occurence of db user entity with same ID field and returns "Ok(AppUser)"
    pub async fn find_by_id(id: i32, connection: &Db) -> Result<AppUser, &'static str> {
        if let Ok(app_user) = connection.run(move |conn| {
            users::table
                .filter(users::id.eq(Some(id)))
                .first(conn)
        }).await {
            return Ok(app_user);
        }
        Err("Couldn't find user")
    }

    //Creates new user and returns it
    pub async fn create(user: AppUser, connection: &Db) -> AppUser {
        connection.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
        }).await;

        connection.run(move |conn| {
        users::table
            .order(users::id.desc())
            .first(conn)
        }).await.unwrap()
    }

    //Read all users from database
    pub async fn read(connection: &Db) -> Vec<AppUser> {
        connection.run(move |conn| {
            users::table
                .order(users::id)
                .load::<AppUser>(conn)
        }).await.unwrap()
    }

    //Update user in database and return if succeeded
    pub async fn update(id: i32, user: AppUser, connection: &Db) -> bool {
        connection.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(&user)
                .execute(conn)
        }).await.is_ok()
    }

    //Deletes user from database and return if succeeded
    pub async fn delete(id: i32, connection: &Db) -> bool {
        connection.run(move |conn| {
            diesel::delete(users::table.find(id))
                .execute(conn)
        }).await.is_ok()
    }
}
