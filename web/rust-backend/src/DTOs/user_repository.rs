use crate::entities::app_user::AppUser;

pub fn find_by_username(username: &str) -> Result<AppUser, &'static str> {
    if username == "kjkardum" {
        return Ok(AppUser {
            id: 1,
            username: "kjkardum".to_string(),
            password_hash: "$2b$12$dpGY2pc/bA3RF60c1mm68OPyTecYTxlrp3fes6AfSTBC7Pn431o4K"
                .to_string(), //Hashed "Pa$$w0rd"
            is_admin: true,
        });
    }
    Err("Couldn't find user")
}
