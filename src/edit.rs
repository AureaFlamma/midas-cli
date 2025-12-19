use crate::database::{load_holdings, update_holding};
use crate::types::GoldHolding;
use crate::uid::update_uid;

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
# Purchase date:
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

    let updated_holding = parse_edited_holding(&edited, &selected_holding)?; // <?> Why not the values directly?

    update_holding(&selected_holding.uid, &updated_holding)?;
    println!(
        "succesfully updated holding {}. It now has id of {}",
        selected_holding.uid, updated_holding.uid
    );

    Ok(())
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
    // TODO: validation function
    if parts.len() != 5 {
        return Err(format!("Expected 5 fields, found {}", parts.len()).into());
    }
    let mut updated_holding: GoldHolding = GoldHolding {
        uid: old_holding.uid.clone(),
        coin_type: parts[0].to_string(),
        coin_year: parts[1].to_string(),
        gold_content: parts[2].parse()?,
        purchase_date: parts[3].to_string(),
        purchase_price: parts[4].parse()?,
    };

    updated_holding.uid = update_uid(&updated_holding, old_holding)?;

    Ok(updated_holding)
}
