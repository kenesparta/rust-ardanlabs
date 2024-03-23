use crate::login::LoginRole;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::path::Path;

pub fn hash_passwd(passwd: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(passwd);
    format!("{:X}", hasher.finalize())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub passwd: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, passwd: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            passwd: hash_passwd(passwd),
            role,
        }
    }
}

pub fn get_users_from_file() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        return users;
    }
    let users = get_users_map();
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
    users
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

pub fn save_users(users: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

fn test_vec() -> Vec<String> {
    let users: Vec<String> = get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect();
    users
}
