use rand::distr::Alphanumeric;
use rand::{rng, Rng};

use crate::database::load_holdings;

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
