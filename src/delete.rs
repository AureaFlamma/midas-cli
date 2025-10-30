use crate::helpers::{check_if_empty, load_holdings, save_holdings};
use inquire::MultiSelect;

pub fn delete_holdings(ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;

    check_if_empty(&holdings, "No holdings found. Nothing to delete.");

    let holdings_after_deletion: Vec<_> = holdings
        .iter()
        .filter(|h| !ids.contains(&h.uid))
        .cloned()
        .collect();

    if holdings_after_deletion.len() == holdings.len() {
        println!("One or more of the specified IDs do not belong to any assets"); // TODO: Make a specfic message for each ID.

        Ok(())
    } else {
        save_holdings(&holdings_after_deletion)?; //TODO: success message
        Ok(())
    }
}

pub fn get_deletion_input() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;
    let options: Vec<String> = holdings.iter().cloned().map(|h| h.uid).collect();
    let selected_ids = MultiSelect::new("Select asset to delete: ", options)
        .with_help_message("Use arrow keys to navigate, Space to select and Enter to confirm")
        .with_page_size(20) // TODO: Abstract into constant
        .prompt()?;

    Ok(selected_ids)
}

pub fn delete_holdings_without_args() -> Result<(), Box<dyn std::error::Error>> {
    let selected_ids = get_deletion_input()?;

    delete_holdings(selected_ids)
}
