use crate::user::get_users;

#[derive(PartialEq, Debug, Clone, Copy)]
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
