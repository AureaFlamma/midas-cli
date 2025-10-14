use crate::helpers::{load_holdings, save_holdings, prompt};
use crate::types::GoldHolding;
use chrono::NaiveDate;

// Add a new holding interactively
pub fn add_holding() -> Result<(), Box<dyn std::error::Error>> {
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