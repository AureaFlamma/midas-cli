use serde::{Deserialize, Serialize};

// Define what a gold holding looks like
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoldHolding {
    pub coin_type: String,
    pub uid: String,
    pub gold_content: f64,
    pub coin_year: i32,        // TODO: harmonise types.
    pub purchase_date: String, // Store as string like "2024-01-15"
    pub purchase_price: f64,
}

pub struct GoldHoldingStats {
    pub current_price: f64,
    pub price_change: f64,
    pub percentage_change: f64,
}

#[derive(Debug)]
pub struct TotalStats {
    pub total_purchase_price: f64,
    pub total_price_change: f64,
    pub total_percentage_change: f64,
}
