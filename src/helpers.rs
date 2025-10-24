use crate::types::{GoldHolding, GoldHoldingStats, TotalStats};
use colored::Colorize;
use comfy_table::{Cell, Color, Table};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

// Get the path to our data file
pub fn get_data_file_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let mut path = PathBuf::from(home);
    path.push(".midas-cli");
    path.push("holdings.json");
    path
}

// Load holdings from JSON file
pub fn load_holdings() -> Result<Vec<GoldHolding>, Box<dyn std::error::Error>> {
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
pub fn save_holdings(holdings: &Vec<GoldHolding>) -> Result<(), Box<dyn std::error::Error>> {
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
pub fn prompt(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_price_cell(price_change: f64) -> Cell {
    // TODO: minus sign should be before the £ sign.
    let text = format!("£{:.2}", price_change);
    let mut cell = Cell::new(text);

    if price_change > 0.0 {
        cell = cell.fg(Color::Green);
    } else if price_change < 0.0 {
        cell = cell.fg(Color::Red);
    }

    cell
}

// TODO: These 2 functions differ with position of 1 character only. Construct into one function
pub fn get_percentage_cell(price_change: f64) -> Cell {
    let text = format!("{:.2}%", price_change);
    let mut cell = Cell::new(text);

    if price_change > 0.0 {
        cell = cell.fg(Color::Green);
    } else if price_change < 0.0 {
        cell = cell.fg(Color::Red);
    }

    cell
}
// TODO: minus sign should be before the £ sign.
pub fn get_colored_text(value: f64, text: &str) -> String {
    if value >= 0.0 {
        text.green().to_string()
    } else {
        text.red().to_string()
    }
}

pub fn get_coin_stats(gold_price: f64, gold_content: f64, purchase_price: f64) -> GoldHoldingStats {
    let current_price = gold_price * gold_content;
    let price_change = current_price - purchase_price;
    let percentage_change = (price_change / purchase_price) * 100.00;

    GoldHoldingStats {
        current_price,
        price_change,
        percentage_change,
    }
}

pub fn get_total_stats(holdings_with_stats: &Vec<(&GoldHolding, GoldHoldingStats)>) -> TotalStats {
    let mut total_purchase_price = 0.0;
    let mut total_price_now = 0.0;
    for (holding, stats) in holdings_with_stats {
        total_purchase_price += holding.purchase_price;
        total_price_now += stats.current_price;
    } // TODO: change so that total_price_now multiplies total_weight by current price for lesser error.
      // TODO: return total_weight too
    let total_price_change = total_price_now - total_purchase_price;
    let total_percentage_change = (total_price_change / total_purchase_price) * 100.00;

    TotalStats {
        total_purchase_price,
        total_price_change,
        total_percentage_change,
    }
}
