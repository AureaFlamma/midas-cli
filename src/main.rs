use clap::{Parser, Subcommand};

mod add;
mod coin_types;
mod gold_price;
mod helpers;
mod list;
mod table;
mod types;
mod uid;

use add::add_holding;
use dotenv::dotenv;
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
    List {
        #[arg(short, long)]
        detail: bool,
    },
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    // Execute the appropriate command
    match cli.command {
        Commands::Add => {
            if let Err(e) = add_holding() {
                eprintln!("Error adding holding: {}", e);
                std::process::exit(1);
            }
        }
        Commands::List { detail } => {
            if let Err(e) = list_holdings(detail).await {
                eprintln!("Error listing holdings: {}", e);
                std::process::exit(1);
            }
        }
    }
}
