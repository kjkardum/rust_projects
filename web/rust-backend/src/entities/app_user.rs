use self::diesel::prelude::*;
use crate::data::schema::users;
use crate::data::diesel_pg::Db;
use rocket_sync_db_pools::diesel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, JsonSchema, Debug, AsChangeset, Queryable, Insertable, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct AppUser {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

impl AppUser {
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

    pub async fn read(connection: &Db) -> Vec<AppUser> {
        connection.run(move |conn| {
            users::table
                .order(users::id)
                .load::<AppUser>(conn)
        }).await.unwrap()
    }

    pub async fn update(id: i32, user: AppUser, connection: &Db) -> bool {
        connection.run(move |conn| {
            diesel::update(users::table.find(id))
                .set(&user)
                .execute(conn)
        }).await.is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(users::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
