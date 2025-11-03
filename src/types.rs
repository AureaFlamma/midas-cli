use serde::{Deserialize, Serialize};

// Define what a gold holding looks like
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoldHolding {
    pub coin_type: String,
    pub uid: String,
    pub gold_content: f64,
    pub coin_year: String,
    pub purchase_date: String,
    pub purchase_price: f64,
}

pub struct GoldHoldingStats {
    pub current_price: f64,
    pub price_change: f64,
    pub percentage_change: f64,
}

#[derive(Debug)]
pub struct TotalStats {
    pub total_price_now: f64,
    pub total_price_change: f64,
    pub total_percentage_change: f64,
    pub number_of_assets: u16,
    pub total_weight: f64,
}

pub type HoldingsWithStats = Vec<(GoldHolding, GoldHoldingStats)>;
