use crate::entities::{app_user::AppUser, url::Url};
use self::diesel::prelude::*;
use crate::data::diesel_pg::Db;
use crate::data::schema::{users, urls};
use rocket_sync_db_pools::diesel;

impl Url {
    //Finds all occurences of db url entities from user by username field
    pub async fn find_by_username(username: &str, connection: &Db) -> Result<Vec<Url>, &'static str> {
        let username_clone = username.to_string();
        let db_result: Result<AppUser, diesel::result::Error> = connection.run(move |conn| {
            users::table
                .filter(users::username.eq(username_clone))
                .first(conn)
        }).await;
        if let Ok(app_user) = db_result {
            if let Ok(db_urls) = connection.run(move |conn| {
                Url::belonging_to(&app_user)
                    .load::<Url>(conn)
            }).await{
                return Ok(db_urls);
            }
        }
        Err("Couldn't find user")
    }

    //Finds all occurences of db url entities from user by ID field
    pub async fn find_by_user_id(id: i32, connection: &Db) -> Result<Vec<Url>, &'static str> {
        let db_result: Result<AppUser, diesel::result::Error> = connection.run(move |conn| {
            users::table
                .filter(users::id.eq(Some(id)))
                .first(conn)
        }).await;
        if let Ok(app_user) = db_result {
            if let Ok(db_urls) = connection.run(move |conn| {
                Url::belonging_to(&app_user)
                    .load::<Url>(conn)
            }).await{
                return Ok(db_urls);
            }
        }
        Err("Couldn't find user")
    }

    //Finds first url occurence with specified id
    pub async fn find_by_short(short: &str, connection: &Db) -> Result<Url, &'static str> {
        let short_clone = short.to_string();
        if let Ok(app_url) = connection.run(move |conn| {
            urls::table
                .filter(urls::short_url.eq(short_clone))
                .first(conn)
        }).await {
            return Ok(app_url);
        }
        Err("Couldn't find url")
    }

    //Creates new url and returns it
    pub async fn create(url: Url, connection: &Db) -> Url {
        connection.run(move |conn| {
        diesel::insert_into(urls::table)
            .values(&url)
            .execute(conn)
        }).await;

        connection.run(move |conn| {
        urls::table
            .order(urls::id.desc())
            .first(conn)
        }).await.unwrap()
    }

    //Read all urls from database
    pub async fn read(connection: &Db) -> Vec<Url> {
        connection.run(move |conn| {
            urls::table
                .order(urls::id)
                .load::<Url>(conn)
        }).await.unwrap()
    }

    //Update url in database and return if succeeded
    pub async fn update(id: i32, url: Url, connection: &Db) -> bool {
        connection.run(move |conn| {
            diesel::update(urls::table.find(id))
                .set(&url)
                .execute(conn)
        }).await.is_ok()
    }

    //Deletes url from database and return if succeeded
    pub async fn delete(id: i32, connection: &Db) -> bool {
        connection.run(move |conn| {
            diesel::delete(urls::table.find(id))
                .execute(conn)
        }).await.is_ok()
    }
}
