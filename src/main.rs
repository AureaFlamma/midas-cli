use clap::{Parser, Subcommand};

mod add;
mod coin_types;
mod constants;
mod database;
mod delete;
mod gold_price;
mod helpers;
mod list;
mod populate_table;
mod sort;
mod table;
mod types;
mod uid;

use add::add_holding;
use delete::{delete_holdings_with_args, delete_holdings_without_args};
use dotenv::dotenv;
use list::list_holdings;
use populate_table::populate_table;
use sort::set_sort_preference;

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
    Add,
    List {
        #[arg(short, long)]
        detail: bool,
    },
    Delete {
        ids: Option<Vec<String>>,
    },
    Populate,
    Sort,
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

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
        Commands::Delete { ids } => match ids {
            Some(ids) => {
                if let Err(e) = delete_holdings_with_args(ids) {
                    eprintln!("Error deleting holding: {}", e);
                    std::process::exit(1);
                }
            }
            None => {
                if let Err(e) = delete_holdings_without_args() {
                    eprintln!("Error deleting holding: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::Populate => {
            if let Err(e) = populate_table() {
                eprintln!("Error populating table: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Sort => {
            if let Err(e) = set_sort_preference().await {
                eprintln!("Error setting sort preference: {}", e);
                std::process::exit(1);
            }
        }
    }
}

// sort by current price, price change
