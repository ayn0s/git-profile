mod config;
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitp", version, about = "Git Profile Helper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "init")]
    Init { 
        #[arg(short, long)]
        name: Option<String>,
    },
    
    #[command(name = "clone")]
    Clone {
        url: String,
    },
    
    #[command(name = "profile")]
    Profile {
        #[command(subcommand)]
        subcommand: ProfileCommands,
    },
}

#[derive(Subcommand)]
pub enum ProfileCommands {
    #[command(name = "list")]
    List {
        #[arg(long)]
        verbose: bool,
    },

    #[command(name = "add")]
    Add {
        #[arg(short, long)]
        name: Option<String>,
        
        #[arg(short, long)]
        email: Option<String>,
        
        #[arg(long)]
        ssh: Option<String>,
    },

    #[command(name = "remove")]
    Remove {
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::init::handle(name),
        Commands::Profile { subcommand } => match subcommand {
            ProfileCommands::List { verbose } => commands::profile::list(verbose),
            ProfileCommands::Add { name, email, ssh } => commands::profile::add(name, email, ssh),
            ProfileCommands::Remove { name } => commands::profile::remove(name),
        },
        Commands::Clone { url } => commands::clone::handle(url)
    }
}

