use crate::login::LoginRole;

pub struct User {
    pub username: String,
    pub passwd: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, passwd: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            passwd: passwd.to_string(),
            role,
        }
    }
}

pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "admin", LoginRole::Admin),
        User::new("bob", "pass", LoginRole::User),
    ]
}
