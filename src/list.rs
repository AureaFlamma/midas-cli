use crate::helpers::load_holdings;
use comfy_table::Table;

// List all holdings in a table
pub fn list_holdings() -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;
    
    if holdings.is_empty() {
        println!("No holdings found. Use 'midas add' to add your first holding.");
        return Ok(());
    }
    
    // Create table
    let mut table = Table::new();
    table.set_header(vec!["Asset ID", "Coin Type", "Coin year", "Gold weight (g)", "Purchase Date", "Purchase Price (£)"]);
    
    // Calculate total
    let mut total = 0.0;
    
    for holding in &holdings {
        table.add_row(vec![
            &holding.uid,
            &holding.coin_type,
            &holding.coin_year.to_string(),
            &format!("{:.2}", holding.gold_content),
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