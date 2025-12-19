use rand::distr::Alphanumeric;
use rand::{rng, Rng};

use crate::database::load_holdings;
use crate::types::GoldHolding;

fn generate_unique_differentiator() -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        .take(4)
        .map(char::from)
        .collect()
}

pub fn check_for_collision(
    uid: &str,
    existing_ids: &Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if existing_ids.contains(&uid) {
        Err("UID already exists".into())
    } else {
        Ok(())
    }
}

pub fn construct_uid(code: &str, year: &str) -> Result<String, Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;
    let existing_ids: Vec<&str> = holdings.iter().map(|h| h.uid.as_str()).collect();
    // If uid collides, it loops again and re-creates it, until it arrives at a unique one.
    loop {
        let uid: String = format!("{}.{}.{}", code, year, generate_unique_differentiator());
        if check_for_collision(&uid, &existing_ids).is_ok() {
            return Ok(uid);
        }
    }
}
// TODO: the new uid may inherit the unique differentiator.
pub fn update_uid(
    new_holding: &GoldHolding,
    old_holding: &GoldHolding,
) -> Result<String, Box<dyn std::error::Error>> {
    let GoldHolding {
        coin_type: new_coin_type,
        coin_year: new_coin_year,
        ..
    } = new_holding;

    let GoldHolding {
        coin_type: old_coin_type,
        coin_year: old_coin_year,
        uid: old_uid,
        ..
    } = old_holding;

    if new_coin_type == old_coin_type && new_coin_year == old_coin_year {
        Ok(old_uid.clone())
    } else {
        let new_uid = construct_uid(new_coin_type, new_coin_year)?; // FIXME: new_coin_type needs to be matched  against the coin types

        Ok(new_uid)
    }
}
