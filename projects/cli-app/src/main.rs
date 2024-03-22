use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<LoginCommands>
}

#[derive(Subcommand)]
enum LoginCommands {
    /// List all users
    List,
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Password");
    println!("{:-<40}", "");
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(LoginCommands::List) => {
            print!("List Users Here");
        }
        None => {
            println!("Run with --help to see instructions.")
        }
    }
}
