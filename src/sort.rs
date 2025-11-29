use crate::database::save_sort_preference_to_db;

pub fn set_sort_preference(column: String, ascending: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Validate column name
    let valid_columns = ["uid", "coin_type", "coin_year", "gold_content", "purchase_date", "purchase_price"];
    
    if !valid_columns.contains(&column.as_str()) {
        return Err(format!(
            "Invalid column '{}'. Valid columns: {}",
            column,
            valid_columns.join(", ")
        ).into());
    }
    
    save_sort_preference_to_db(&column, ascending)?;
    
    println!("✓ Sort preference saved: {} by {}", 
        if ascending { "Ascending" } else { "Descending" },
        column
    );
    
    Ok(())
}