use serde::{Deserialize, Serialize};

// Define what a gold holding looks like
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoldHolding {
    pub coin_type: String,
    pub gold_content: f64,
    pub coin_year: i32, // TODO: harmonise types.
    pub purchase_date: String,  // Store as string like "2024-01-15"
    pub purchase_price: f64,
}