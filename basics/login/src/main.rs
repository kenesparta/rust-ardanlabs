use authentication::greet::greet_user;
use authentication::login::login;
use authentication::read::read_line;

fn main() {
    println!("{}", greet_user("Ken"));
    let mut tries = 0;
    loop {
        println!("Enter your username: ");
        let username = read_line();
        println!("Enter your passwd: ");
        let passwd = read_line();
        if login(&username, &passwd) {
            print!("Welcome {username}!\n");
            break;
        }
        print!("Incorrect username or password\n");
        tries += 1;
        if tries >= 3 {
            print!("Too many failed logins\n");
            break;
        }
    }
}
