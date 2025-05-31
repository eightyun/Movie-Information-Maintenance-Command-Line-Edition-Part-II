use clap::{Parser, Subcommand};
use movie::handler::{handle_logout, handler_login};

#[derive(Parser)]
#[command(
    version,
    about = "Movie app",
    long_about = "Movie information app"
)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// User log into the system
    Login {
        /// The username of the user
        #[arg(short, long)]
        username: String
    },
    
    /// Log out
    Logout,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => handler_login(username)?,
        Some(Commands::Logout) => handle_logout(),
        _ => println!("No command provided or command not recognized"),
    }

    Ok(())
}

