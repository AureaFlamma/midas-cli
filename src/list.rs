use crate::gold_price::fetch_gold_price_gbp;
use crate::helpers::{
    get_coin_stats, get_colored_text, get_percentage_cell, get_price_cell, load_holdings, get_total_stats
};
use crate::types::TotalStats;
use comfy_table::{Cell, Color, Table};

// List all holdings in a table
pub async fn list_holdings(detail: bool) -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;
    let current_price_per_gram: f64 = fetch_gold_price_gbp().await?; // TODO: Implement caching to prevent more than 1 call a day. Also if call fails, display last fetched value.

    if holdings.is_empty() {
        println!("No holdings found. Use 'midas add' to add your first holding.");
        return Ok(());
    }
    let holdings_with_stats = holdings
        .iter()
        .map(|holding| {
            let stats = get_coin_stats(
                current_price_per_gram,
                holding.gold_content,
                holding.purchase_price,
            );
            (holding, stats)
        })
        .collect(); // TODO: this can be abstracted further

    let TotalStats {
        total_purchase_price,
        total_price_change,
        total_percentage_change,
    } = get_total_stats(&holdings_with_stats);

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

    for holding in &holdings {
        // TODO: destructure holding
        // FIXME: Mixing calculating logic and add-to-table logic.

        table.add_row(vec![
            Cell::new(&holding.uid),
            Cell::new(&holding.coin_type),
            Cell::new(&holding.coin_year.to_string()),
            Cell::new(&format!("{:.2}", holding.gold_content)),
            Cell::new(&holding.purchase_date),
            Cell::new(&format!("£{:.2}", holding.purchase_price)),
            Cell::new(&format!("£{:.2}", current_asset_price)),
            get_price_cell(price_change),
            get_percentage_cell(percentage_change),
        ]);
    }

    println!("\n{}", table);
    println!("\nTotal Holdings: {}", holdings.len());
    println!("Total Investment: £{:.2}", total_purchase_price);
    println!(
        "Total change: {}({})",
        get_colored_text(total_price_change, &format!("£{:.2}", total_price_change)),
        get_colored_text(
            total_percentage_change,
            &format!("{:.2}%", total_percentage_change)
        )
    );

    Ok(())
}
