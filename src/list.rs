use crate::helpers::{
    check_if_empty, get_colored_text, get_holdings_stats, get_total_stats,
};
use crate::database::load_holdings;
use crate::table::{create_detail_table, create_summary_table};
use crate::types::TotalStats;

pub async fn list_holdings(detail: bool) -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;

    check_if_empty(
        &holdings,
        "No holdings found. Use 'midas add' to add your first holding.",
    );

    let holdings_with_stats = get_holdings_stats(holdings).await?;

    let TotalStats {
        total_price_now,
        total_price_change,
        total_percentage_change,
        number_of_assets,
        total_weight,
    } = get_total_stats(&holdings_with_stats);

    let table = if detail {
        create_detail_table(holdings_with_stats)
    } else {
        create_summary_table(holdings_with_stats)
    };

    println!("\n{}", table);
    println!("\nTotal Holdings: {} pieces", number_of_assets);
    println!("\nTotal weight: {:.2}g of gold", total_weight);
    println!("Total Investment: £{:.2}", total_price_now);
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
