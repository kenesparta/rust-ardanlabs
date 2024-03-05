use clap::{Command, Parser, Subcommand};

#[derive(Parser)]
#[command]
struct Args {
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Subcommand)]
enum Commands {
    /// List all users
    List,
}

fn main() {
    println!("Hello, world!");
}
