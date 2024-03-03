#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}


pub fn login(username: &str, passwd: &str) -> Option<LoginAction> {
    let uname = username.to_lowercase().as_str();

    if uname != "admin" && uname != "bob" {
        None
    }

    Some(
        match (uname, passwd) {
            ("admin", "admin") => LoginAction::Granted(LoginRole::Admin),
            ("bob", "pass") => LoginAction::Granted(LoginRole::User),
            _ => LoginAction::Denied,
        }
    )
}
