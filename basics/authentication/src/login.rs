pub fn login(username: &str, pass: &str) -> bool {
    username.to_lowercase() == "admin" && pass == "admin"
}
