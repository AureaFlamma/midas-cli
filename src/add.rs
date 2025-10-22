use crate::helpers::{load_holdings, save_holdings, prompt};
use crate::types::GoldHolding;
use crate::coin_types::select_coin_type;
use crate::uid::construct_uid;
use chrono::{Datelike, NaiveDate, Utc};

// Add a new holding interactively
pub fn add_holding() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Add New Gold Holding ===\n");
    
    // Get coin type with interactive selection
    let (coin_type, gold_content, code) = select_coin_type()?;

    println!("Selected: {} ({:.2}g gold content)", coin_type, gold_content);
    let coin_year: i32 = loop {
        let year_str = prompt("Mint year: ")?;
        
        // Get current year
        let current_year = Utc::now().year();
        let minimum_year = 1650; // TODO: abstract into a constants file.
        // Validate year is a 4-digit number within valid range
        match year_str.parse::<i32>() {
            Ok(year) if year >= minimum_year && year <= current_year => break year,
            Ok(year) if year > current_year => {
                println!("Invalid year. Year cannot be in the future (max: {})", current_year);
            },
            _ => println!("Invalid year format. Please use YYYY (e.g., 2024)"),
        }
    };
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

    let uid = construct_uid(&code, &coin_year.to_string()); // TODO: Maybe coin year should be a string in general?

    // Create new holding
    let new_holding = GoldHolding {
        uid,
        coin_type,
        coin_year,
        gold_content,
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