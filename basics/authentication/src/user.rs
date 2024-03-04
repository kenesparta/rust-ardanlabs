use crate::login::LoginRole;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

pub fn get_users_map() -> HashMap<String, User> {
    let mut users_map = HashMap::new();
    users_map.insert(
        "admin".to_string(),
        User::new("admin", "admin", LoginRole::Admin),
    );
    users_map.insert("bob".to_string(), User::new("bob", "pass", LoginRole::User));
    users_map
}

fn test_vec() -> Vec<String> {
    let users: Vec<String> = get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect();
    users
}
