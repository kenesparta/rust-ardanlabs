use crate::login::{login, LoginRole};

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

pub fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "admin", LoginRole::Admin),
        User::new("bob", "pass", LoginRole::User),
    ]
}

fn test_vec() -> Vec<String> {
    let users: Vec<String> = get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect();
    users
}