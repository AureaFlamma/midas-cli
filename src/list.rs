use crate::gold_price::fetch_gold_price_gbp;
use crate::helpers::load_holdings;
use comfy_table::Table;

// List all holdings in a table
pub async fn list_holdings() -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;

    if holdings.is_empty() {
        println!("No holdings found. Use 'midas add' to add your first holding.");
        return Ok(());
    }

    let current_price_per_gram: f64 = fetch_gold_price_gbp().await?;

    // Create table
    let mut table = Table::new();
    table.set_header(vec![
        "Asset ID",
        "Coin Type",
        "Coin year",
        "Gold weight (g)",
        "Purchase Date",
        "Purchase Price (£)",
        "Current Price (£)",
        "Price change (£)",
        "Price change (%)",
    ]);

    // Calculate total
    let mut total_purchase_price = 0.0;
    let mut total_price_now = 0.0;

    for holding in &holdings {
        // TODO: destructure holding
        // FIXME: Mixing calculating logic and add-to-table logic.
        let current_asset_price = current_price_per_gram * holding.gold_content;
        let price_change = current_asset_price - holding.purchase_price;
        let percentage_change = (price_change / holding.purchase_price) * 100.00;
        table.add_row(vec![
            &holding.uid,
            &holding.coin_type,
            &holding.coin_year.to_string(),
            &format!("{:.2}", holding.gold_content),
            &holding.purchase_date,
            &format!("£{:.2}", holding.purchase_price),
            &format!("£{:.2}", current_asset_price),
            &format!("£{:.2}", price_change),
            &format!("{:.2}%", percentage_change), // TODO: add handling of negative
        ]);
        total_purchase_price += holding.purchase_price;
        total_price_now += current_asset_price;
    }

    let total_price_change = total_price_now - total_purchase_price;
    let total_percentage_change = (total_price_change / total_purchase_price) * 100.00;
    println!("\n{}", table);
    println!("\nTotal Holdings: {}", holdings.len());
    println!("Total Investment: £{:.2}", total_purchase_price);
    println!(
        "Total change: £{:.2}({:.2}%)",
        total_price_change, total_percentage_change
    );

    Ok(())
}
