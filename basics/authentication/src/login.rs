use crate::user::{get_users, get_users_from_file, get_users_map, hash_passwd};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

pub fn login(username: &str, passwd: &str) -> Option<LoginAction> {
    let users = get_users();
    let username = username.to_lowercase();

    users
        .iter()
        .find(|u| u.username == username && u.passwd == passwd)
        .map(|u| LoginAction::Granted(u.role))
        .or_else(|| {
            if users.iter().any(|u| u.username == username) {
                return Some(LoginAction::Denied);
            }
            None
        })
}

pub fn login_map(username: &str, passwd: &str) -> Option<LoginAction> {
    let users = get_users_map();
    let username = username.to_lowercase();
    if let Some(user) = users.get(&username) {
        if user.passwd == passwd {
            return Some(LoginAction::Granted(user.role.clone()));
        }
        return Some(LoginAction::Denied);
    }
    None
}

pub fn login_map_file(username: &str, passwd: &str) -> Option<LoginAction> {
    let users = get_users_from_file();
    let username = username.to_lowercase();
    let passwd = hash_passwd(passwd);
    if let Some(user) = users.get(&username) {
        if user.passwd == passwd {
            return Some(LoginAction::Granted(user.role.clone()));
        }
        return Some(LoginAction::Denied);
    }
    None
}
