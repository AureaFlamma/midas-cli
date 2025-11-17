use crate::constants::COIN_TYPES;
use crate::database::save_holding;
use crate::types::GoldHolding;
use crate::uid::construct_uid;
use chrono::{Utc, Datelike};

pub fn populate_table() -> Result<(), Box<dyn std::error::Error>> {
    for (coin_type, gold_content, code) in COIN_TYPES {
        let coin_year = 1900.to_string();
        let uid = construct_uid(code, &coin_year)?;
        let purchase_date = Utc::now().year().to_string();
        let purchase_price = gold_content * 60.00;
        
        let new_holding = GoldHolding {
            uid,
            coin_type: coin_type.to_string(), // FIXME: Sort out types here and in the primary save function
            coin_year,
            gold_content: *gold_content,
            purchase_date,
            purchase_price,
        };
        
        save_holding(&new_holding)?;
    };

    println!("Dev data succesfully generated");

    Ok(())
}