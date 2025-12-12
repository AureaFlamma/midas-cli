use crate::{database::load_holdings, types::GoldHolding};

pub fn edit_holding_with_arg(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;

    let selected_holding = holdings
        .into_iter()
        .find(|holding| holding.uid == id)
        .ok_or_else(|| format!("Holding with id '{}' not found", id))?;

    let display_string = format!(
        "{} | {} | {} | {} | {}",
        selected_holding.coin_type,
        selected_holding.coin_year,
        selected_holding.gold_content,
        selected_holding.purchase_date,
        selected_holding.purchase_price
    );

    let edited = edit::edit(display_string)?;

    let updated_holding = parse_edited_holding(&edited, &selected_holding.uid)?; // <?> Why not the values directly?

    println!(
        "Updated holding {}: {} | {} | {} | {} | {}",
        updated_holding.uid,
        updated_holding.coin_type,
        updated_holding.coin_year,
        updated_holding.gold_content,
        updated_holding.purchase_date,
        updated_holding.purchase_price,
    );

    Ok(())
}

fn parse_edited_holding(
    editable: &str,
    uid: &str,
) -> Result<GoldHolding, Box<dyn std::error::Error>> {
    let data_line = editable
        .lines()
        .find(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
        .ok_or("no data found in edited content")?;

    let parts: Vec<&str> = data_line.split('|').map(|part| part.trim()).collect();

    if parts.len() != 5 {
        return Err(format!("Expected 5 fields, found {}", parts.len()).into());
    }

    Ok(GoldHolding {
        uid: uid.to_string(),
        coin_type: parts[0].to_string(),
        coin_year: parts[1].to_string(),
        gold_content: parts[2].parse()?,
        purchase_date: parts[3].to_string(),
        purchase_price: parts[4].parse()?,
    })
}

// ToDo: Re-create uid based on new coin name
