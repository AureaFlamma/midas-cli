use crate::gold_price::fetch_gold_price_gbp;
use crate::types::{GoldHolding, GoldHoldingStats, HoldingsWithStats, TotalStats};
use colored::Colorize;
use comfy_table::{Cell, Color};
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

pub fn prompt(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_colored_change_cell(price_change: f64, string: String) -> Cell {
    let mut cell = Cell::new(string);

    if price_change > 0.0 {
        cell = cell.fg(Color::Green);
    } else if price_change < 0.0 {
        cell = cell.fg(Color::Red);
    }

    cell
}
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

pub fn get_total_stats(holdings_with_stats: &HoldingsWithStats) -> TotalStats {
    let mut total_purchase_price = 0.0;
    let mut total_price_now = 0.0;
    let mut total_weight = 0.0;
    for (holding, stats) in holdings_with_stats {
        total_purchase_price += holding.purchase_price;
        total_weight += holding.gold_content;
        total_price_now += stats.current_price;
    }
    let total_price_change = total_price_now - total_purchase_price;
    let total_percentage_change = (total_price_change / total_purchase_price) * 100.00;
    let number_of_assets = holdings_with_stats.len() as u16;
    TotalStats {
        total_price_now,
        total_price_change,
        total_percentage_change,
        number_of_assets,
        total_weight,
    }
}

pub async fn get_holdings_stats(
    holdings: Vec<GoldHolding>,
) -> Result<HoldingsWithStats, Box<dyn std::error::Error>> {
    let current_price_per_gram: f64 = fetch_gold_price_gbp().await?;

    let holdings_with_stats = holdings
        .into_iter()
        .map(|holding| {
            let stats = get_coin_stats(
                current_price_per_gram,
                holding.gold_content,
                holding.purchase_price,
            );
            (holding, stats)
        })
        .collect();

    Ok(holdings_with_stats)
}

pub fn check_if_empty(holdings: &Vec<GoldHolding>) {
    if holdings.is_empty() {
        println!("No holdings found. Use 'midas add' to add your first holding.");
        std::process::exit(0);
    }
}

pub fn format_currency(value: f64) -> String {
    format!("{:+.2}", value)
        .replace("+", "+£")
        .replace("-", "-£")
}

// TODO: Split across more than one helper file
