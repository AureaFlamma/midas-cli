use crate::constants::DB_COLUMNS;
use crate::database::save_sort_preference_to_db;
use inquire::Select;

pub fn set_sort_preference() -> Result<(), Box<dyn std::error::Error>> {
    let column = select_sort_column()?;
    let direction = select_sort_direction()?;

    save_sort_preference_to_db(&column, direction)?;

    println!(
        "✓ Sort preference saved: {} by {}",
        if direction { "Ascending" } else { "Descending" },
        column
    );

    Ok(())
}

pub fn select_sort_column() -> Result<String, Box<dyn std::error::Error>> {
    let selection = Select::new("select column to sort by", DB_COLUMNS.to_vec())
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .with_page_size(DB_COLUMNS.len())
        .prompt()?;

    Ok(String::from(selection)) // <?> Not sure about the constant conversion and re-conversion
}

pub fn select_sort_direction() -> Result<bool, Box<dyn std::error::Error>> {
    // TODO: Ascending/descending into constants,
    const SORT_DIRECTIONS: [(&str, bool); 2] = [("ascending", true), ("descending", false)];
    let options = SORT_DIRECTIONS.iter().map(|(name, _)| name).collect();

    let selection = Select::new("should we sort in ascending or descending order?", options)
        .with_page_size(2)
        .prompt()?; // <?> What would happen without this?

    for (direction_name, boolean) in SORT_DIRECTIONS {
        if &direction_name == selection {
            return Ok(boolean);
        }
    }

    Err("Invalid selection".into())
}

// TODO: Add capacity for typed arguments with menu arguments as fallback
// TODO: Lint on save
