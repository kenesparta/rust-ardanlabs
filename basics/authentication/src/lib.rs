pub mod greet;
pub mod login;
pub mod read;
mod user;

#[cfg(test)]
mod tests {
    use crate::greet::greet_user;
    use crate::login::{login, LoginAction, LoginRole};

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Herbert", greet_user("Herbert"))
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "admin"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "pass"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(
            login("AdMiN", "admin"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(login("admin", "pass"), Some(LoginAction::Denied));
        assert_eq!(login("admin", ""), Some(LoginAction::Denied));
        assert_eq!(login("ken", "pass"), None);
    }
}
