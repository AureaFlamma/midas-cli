use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use comfy_table::Table;
use chrono::NaiveDate;

// Define what a gold holding looks like
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GoldHolding {
    coin_type: String,
    purchase_date: String,  // Store as string like "2024-01-15"
    purchase_price: f64,
}

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

// Get the path to our data file
fn get_data_file_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let mut path = PathBuf::from(home);
    path.push(".midas-cli");
    path.push("holdings.json");
    path
}

// Load holdings from JSON file
fn load_holdings() -> Result<Vec<GoldHolding>, Box<dyn std::error::Error>> {
    let path = get_data_file_path();
    
    // If file doesn't exist, return empty vector
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let contents = fs::read_to_string(path)?;
    let holdings: Vec<GoldHolding> = serde_json::from_str(&contents)?;
    Ok(holdings)
}

// Save holdings to JSON file
fn save_holdings(holdings: &Vec<GoldHolding>) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_data_file_path();
    
    // Create directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let json = serde_json::to_string_pretty(holdings)?;
    fs::write(path, json)?;
    Ok(())
}

// Prompt user for input
fn prompt(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", message);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Add a new holding interactively
fn add_holding() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Add New Gold Holding ===\n");
    
    // Get coin type
    let coin_type = prompt("Coin type (e.g., Sovereign, Britannia): ")?;
    
    // Get purchase date
    let purchase_date = loop {
        let date_str = prompt("Purchase date (YYYY-MM-DD): ")?;
        
        // Validate date format
        match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(_) => break date_str,
            Err(_) => println!("Invalid date format. Please use YYYY-MM-DD (e.g., 2024-01-15)"),
        }
    };
    
    // Get purchase price
    let purchase_price = loop {
        let price_str = prompt("Purchase price (£): ")?;
        
        match price_str.parse::<f64>() {
            Ok(price) if price > 0.0 => break price,
            _ => println!("Invalid price. Please enter a positive number"),
        }
    };
    
    // Create new holding
    let new_holding = GoldHolding {
        coin_type,
        purchase_date,
        purchase_price,
    };
    
    // Load existing holdings
    let mut holdings = load_holdings()?;
    
    // Add new holding
    holdings.push(new_holding);
    
    // Save back to file
    save_holdings(&holdings)?;
    
    println!("\n✓ Holding added successfully!");
    println!("Total holdings: {}", holdings.len());
    
    Ok(())
}

// List all holdings in a table
fn list_holdings() -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;
    
    if holdings.is_empty() {
        println!("No holdings found. Use 'midas add' to add your first holding.");
        return Ok(());
    }
    
    // Create table
    let mut table = Table::new();
    table.set_header(vec!["Coin Type", "Purchase Date", "Purchase Price (£)"]);
    
    // Calculate total
    let mut total = 0.0;
    
    for holding in &holdings {
        table.add_row(vec![
            &holding.coin_type,
            &holding.purchase_date,
            &format!("£{:.2}", holding.purchase_price),
        ]);
        total += holding.purchase_price;
    }
    
    println!("\n{}", table);
    println!("\nTotal Holdings: {}", holdings.len());
    println!("Total Investment: £{:.2}", total);
    
    Ok(())
}