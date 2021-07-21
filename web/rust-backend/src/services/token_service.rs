use crate::DTOs::user_DTO::UserDTO;
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub const WEEK: u64 = 60 * 60 * 24 * 7;

pub fn generate_token(user: &UserDTO) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("id", user.id.to_string());
    claims.insert("username", user.username.to_string());
    claims.insert("is_admin", user.is_admin.to_string());
    claims.insert("iat", user.iat.to_string());
    claims.insert("eat", user.eat.to_string());
    claims.sign_with_key(&key).unwrap()
}

pub fn verify_token(token: &str) -> Result<UserDTO, &'static str> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    if let Ok(claims) = token.verify_with_key(&key) {
        if !verify_expiry(&claims) {
            return Err("Expired token");
        }
        return Ok(UserDTO {
            id: claims["id"].parse::<i32>().unwrap(),
            username: claims["username"].to_string(),
            is_admin: claims["is_admin"].parse::<bool>().unwrap(),
            iat: claims["iat"].parse::<u64>().unwrap(),
            eat: claims["eat"].parse::<u64>().unwrap(),
        });
    }
    Err("Invalid token")
}

pub fn verify_expiry(claims: &BTreeMap<String, String>) -> bool {
    claims["eat"].parse::<u64>().unwrap() as i64 - time_in_secs() as i64 > 0
}

pub fn time_in_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
