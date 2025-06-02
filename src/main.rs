use clap::{Parser, Subcommand};
use movie::handler::{handle_list, handle_logout, handler_login};

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
    /// List all the movies
    List,
    /// Add a movie
    Add {
        /// The disc no. of the movie
        #[arg(short, long)]
        disc: usize,

        /// The year when the movie was released
        #[arg(short, long)]
        year: String,

        /// The title / file name of the movie
        #[arg(short, long)]
        title: String,

        /// optional remark of the movie
        #[arg(short, long)]
        remark: Option<String>
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => handler_login(username)?,
        Some(Commands::Logout) => handle_logout(),
        Some(Commands::List) => handle_list()?,
        Some(Commands::Add { 
            disc, 
            year, 
            title, 
            remark 
        }) => handle_add(disc, year, title, remark)?,
        _ => println!("No command provided or command not recognized"),
    }

    Ok(())
}

