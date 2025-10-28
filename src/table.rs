use crate::helpers::{get_percentage_cell, get_price_cell};
use crate::types::HoldingsWithStats;
use comfy_table::{Cell, Table};

pub fn create_detail_table(holdings: HoldingsWithStats) -> Table {
    let mut detail_table = Table::new();
    detail_table.set_header(vec![
        "Asset ID",
        "Coin Type",
        "Coin year",
        "Gold weight (g)",
        "Purchase Date",
        "Purchase Price (£)",
        "Current Price (£)",
        "Price change (£)",
        "Price change (%)",
    ]); // TODO: Abstract to constants

    for (holding, stat) in &holdings {
        detail_table.add_row(vec![
            Cell::new(&holding.uid),
            Cell::new(&holding.coin_type),
            Cell::new(holding.coin_year.to_string()),
            Cell::new(format!("{:.2}", holding.gold_content)),
            Cell::new(&holding.purchase_date),
            Cell::new(format!("£{:.2}", holding.purchase_price)),
            Cell::new(format!("£{:.2}", stat.current_price)),
            get_price_cell(stat.price_change),
            get_percentage_cell(stat.percentage_change),
        ]);
    }

    detail_table
}

pub fn create_summary_table(holdings: HoldingsWithStats) -> Table {
    let mut summary_table = Table::new();
    summary_table.set_header(vec![
        "Asset ID",
        "Au content (g)",
        "Current Price (£)",
        "Price change",
    ]);

    for (holding, stat) in &holdings {
        summary_table.add_row(vec![
            &holding.uid,
            &format!("{:.2}", holding.gold_content),
            &format!("{:.2}", stat.current_price),
            &format!("£{:.2}({:.2}%)", stat.price_change, stat.percentage_change),
        ]);
    }

    summary_table
}
