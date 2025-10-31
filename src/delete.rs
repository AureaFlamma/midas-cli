use crate::constants::PAGE_LENGTH_DELETION_OPTIONS;
use crate::helpers::{check_if_empty, load_holdings, save_holdings};
use inquire::MultiSelect;

// TODO: Currently delete_holdings needlessly validates ids even when passed ids from get_deletion_input.
// Perhaps validation logic can be abstracted and called conditionally with prop
pub fn delete_holdings(ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;
    check_if_empty(&holdings, "No holdings found. Nothing to delete.");

    let holdings_ids: Vec<String> = holdings.iter().map(|h| h.uid.clone()).collect();

    let (valid_ids, invalid_ids): (Vec<String>, Vec<String>) =
        ids.into_iter().partition(|id| holdings_ids.contains(id));

    if valid_ids.is_empty() {
        println!("None of the specified IDs belong to any asset. No assets will be deleted");
        return Ok(());
    }

    let holdings_after_deletion: Vec<_> = holdings
        .into_iter()
        .filter(|h| !valid_ids.contains(&h.uid))
        .collect();

    if !invalid_ids.is_empty() {
        println!("Ids {} not found", invalid_ids.join(", "));
    }

    save_holdings(&holdings_after_deletion)?;
    println!("Deleted assets: {}", valid_ids.join(", "));
    Ok(())
}

pub fn get_deletion_input() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let holdings = load_holdings()?;
    let options: Vec<String> = holdings.iter().cloned().map(|h| h.uid).collect();
    let selected_ids = MultiSelect::new("Select asset to delete: ", options)
        .with_help_message("Use arrow keys to navigate, Space to select and Enter to confirm")
        .with_page_size(PAGE_LENGTH_DELETION_OPTIONS)
        .prompt()?;

    if selected_ids.is_empty() {
        Err("No ids provided for deletion. No assets will be deleted.".into())
    } else {
        Ok(selected_ids)
    }
}

pub fn delete_holdings_without_args() -> Result<(), Box<dyn std::error::Error>> {
    let selected_ids = get_deletion_input()?;

    delete_holdings(selected_ids)
}
