use clap::{Parser, Subcommand};

mod types;
mod helpers;
mod add;
mod list;
mod coin_types;

use add::add_holding;
use list::list_holdings;

// CLI structure - defines the commands our app accepts
#[derive(Parser)]
#[command(name = "midas")]
#[command(about = "Track your gold holdings", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new gold holding
    Add,
    /// List all holdings in a table
    List,
}

fn main() {
    let cli = Cli::parse();

    // Execute the appropriate command
    match cli.command {
        Commands::Add => {
            if let Err(e) = add_holding() {
                eprintln!("Error adding holding: {}", e);
                std::process::exit(1);
            }
        }
        Commands::List => {
            if let Err(e) = list_holdings() {
                eprintln!("Error listing holdings: {}", e);
                std::process::exit(1);
            }
        }
    }
}