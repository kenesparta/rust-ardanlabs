use authentication::greet::greet_user;
use authentication::login::{login, login_map_file, LoginAction, LoginRole};
use authentication::read::read_line;

// TODO
// Security Enhancements: Although not directly shown in this snippet, consider adding mechanisms
// like delay on failed login attempts to mitigate brute force attacks.
fn main() {
    println!("{}", greet_user("Ken"));
    let mut tries = 0;
    loop {
        println!("Enter your username: ");
        let username = read_line();
        println!("Enter your passwd: ");
        let passwd = read_line();

        match login_map_file(&username, &passwd) {
            Some(LoginAction::Granted(LoginRole::Admin)) => {
                print!("Welcome Admin: {username}!\n");
                break;
            }
            Some(LoginAction::Granted(LoginRole::User)) => {
                print!("Welcome User: {username}!\n");
                break;
            }
            Some(LoginAction::Denied) => {
                print!("Incorrect username or password\n");
                tries += 1;
            }
            None => {
                print!("New user system\n");
                break;
            }
        }

        if tries >= 3 {
            print!("Too many failed logins\n");
            break;
        }
    }
}
