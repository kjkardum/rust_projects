use crate::data::diesel_pg::Db;
use crate::entities::url::Url;
use rand::{distributions::Alphanumeric, Rng};

pub async fn new(connection: &Db) -> String {
    loop {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>()
            .to_lowercase();
        if let Err(_) = Url::find_by_short(&s, connection).await {
            return s;
        }
    }
}