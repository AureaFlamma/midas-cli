use crate::constants::MINIMUM_COIN_YEAR;
use crate::database::{load_holdings, update_holding};
use crate::types::GoldHolding;
use crate::uid::update_uid;
use chrono::{Datelike, NaiveDate, Utc};

pub fn edit_holding_with_arg(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;

    let selected_holding = holdings
        .into_iter()
        .find(|holding| holding.uid == id)
        .ok_or_else(|| format!("Holding with id '{}' not found", id))?;

    let display_string = format!(
        // TODO: Abstract into string literal
        "
# Coin type:
{}
# Mint year:
{}
# Gold content:
{}
# Purchase date (YYYY-MM-DD):
{}
# Purchase price:
{}  
        ",
        selected_holding.coin_type,
        selected_holding.coin_year,
        selected_holding.gold_content,
        selected_holding.purchase_date,
        selected_holding.purchase_price
    );

    let edited = edit::edit(display_string)?;
    // <?> Why not the values directly?
    match parse_edited_holding(&edited, &selected_holding) {
        Ok(updated_holding) => {
            update_holding(&selected_holding.uid, &updated_holding)?;
            println!(
                "succesfully updated holding {}. It now has id of {}",
                selected_holding.uid, updated_holding.uid
            );

            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);

            Ok(())
        }
    }
}

fn parse_edited_holding(
    editable: &str,
    old_holding: &GoldHolding,
) -> Result<GoldHolding, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = editable
        .lines()
        .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
        .map(|part| part.trim())
        .collect();

    validate_edited_holding(&parts)?;

    let mut updated_holding: GoldHolding = GoldHolding {
        uid: old_holding.uid.clone(),
        coin_type: parts[0].to_string(), // TODO: A dropdown would be good here
        coin_year: parts[1].to_string(),
        gold_content: parts[2].parse()?,
        purchase_date: parts[3].to_string(),
        purchase_price: parts[4].parse()?,
    };

    updated_holding.uid = update_uid(&updated_holding, old_holding)?;

    Ok(updated_holding)
}

#[derive(Debug)]
enum ValidationError {
    InvalidMintYear(String),
    InvalidPurchaseDate(String),
}
// This makes the errors displayable.
// It gives it the Display trait, which makes them printable.
// In turn, the Display trait requires giving the entity a function with the signature of:
// fmt(&self, f: &mut Formatter<'_>) -> Result
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::InvalidMintYear(msg) => write!(f, "{}", msg),
            ValidationError::InvalidPurchaseDate(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

fn validate_edited_holding(
    parts: &[&str], // Length is known ahead of time. Oughtn't it be array then?
) -> Result<(), Box<dyn std::error::Error>> {
    let parts_array: [&str; 5] = parts
        .try_into()
        .map_err(|_| Box::<dyn std::error::Error>::from("Expected exactly 5 fields"))?;

    let [_, coin_year, _, purchase_date, _] = parts_array;

    // TODO: Perhaps this and the equivalent in add.rs could be abstracted into a common helper?
    let current_year: u32 = Utc::now().year().try_into().unwrap();
    match coin_year.parse::<u32>() {
        Ok(coin_year) if (coin_year >= MINIMUM_COIN_YEAR && coin_year <= current_year) => {}
        Ok(coin_year) if coin_year > current_year => {
            return Err(Box::new(ValidationError::InvalidMintYear(format!(
                "Invalid mint year. Mint year cannot be in the future. Max: {}",
                current_year
            ))))
        }
        _ => {
            return Err(Box::new(ValidationError::InvalidMintYear(
                "Invalid mint year. Please use YYYY (e.g. 2024)".into(),
            )))
        }
    }

    // TODO: Perhaps this and the equivalent in add.rs could be abstracted into a common helper?
    match NaiveDate::parse_from_str(purchase_date, "%Y-%m-%d") {
        Ok(_) => {}
        Err(_) => {
            return Err(Box::new(ValidationError::InvalidPurchaseDate(
                "Invalid purchase date format. Please use YYYY-MM-DD".into(),
            )))
        }
    }

    Ok(())
}

// TODO: Would be good to prompt user for re-input of the whole thing OR re-input of the invalid value
