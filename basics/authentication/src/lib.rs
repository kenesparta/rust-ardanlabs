pub mod greet;
pub mod login;
pub mod read;

#[cfg(test)]
mod tests {
    use crate::greet::greet_user;
    use crate::login::login;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Herbert", greet_user("Herbert"))
    }

    #[test]
    fn test_login() {
        assert!(login("admin", "admin"));
        assert!(login("AdMiN", "admin"));
        assert!(!login("admin", "pass"));
        assert!(!login("admin", ""));
    }
}
