use authentication::login::LoginRole;
use authentication::user::{
    get_users, get_users_from_file, get_users_map, hash_passwd, save_users, User,
};
use clap::builder::Str;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<LoginCommands>,
}

#[derive(Subcommand)]
enum LoginCommands {
    /// List all users
    List,

    /// Add a user
    Add {
        /// The users's login name
        username: String,

        /// Password plain text
        password: String,

        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },

    /// Delete a user
    Delete {
        /// The users's login name
        username: String,
    },

    /// Change user's password
    ChangePassword {
        /// The users's login name
        username: String,

        /// Password plain text
        new_password: String,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Password");
    println!("{:-<40}", "");

    get_users_from_file()
        .iter()
        .for_each(|(_, u)| println!("{:<20}{:20?}", u.username, u.role));
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users_from_file();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_users(username: String) {
    let mut users_map = get_users_from_file();
    if users_map.contains_key(&username) {
        users_map.remove(&username);
        save_users(users_map);
        return;
    }
    println!("{username} user does not exist");
}

fn change_password(username: String, new_password: String) {
    let mut users_map = get_users_from_file();
    if let Some(user) = users_map.get_mut(&username) {
        user.passwd = hash_passwd(&new_password);
        save_users(users_map);
        return;
    }
    println!("{username} user does not exist");
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(LoginCommands::List) => {
            list_users();
        }

        None => {
            println!("Run with --help to see instructions.")
        }

        Some(LoginCommands::Add {
            username,
            password,
            admin,
        }) => add_user(username, password, admin.unwrap_or(false)),

        Some(LoginCommands::Delete { username }) => delete_users(username),

        Some(LoginCommands::ChangePassword {
            username,
            new_password,
        }) => change_password(username, new_password),
    }
}
