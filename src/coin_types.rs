use inquire::Select;

/*   A. Why &str rather than String?
 * const context requires the values to be known at compile time. String literals live in the binary, so are known at CT. 
 * In contrast, String::from() does a bunch of things at runtime (allocate heap memory, write to it, return the String struct)
 * 
 *   B. Why &[] rather than []?
 * String slices do not have a set length. Below we are pushing to the slice, which we couldn't do with an array.
 * 
 *   C. Why explicit annotation?
 * Rust requires explicit type annotation for const and static items.
 */
pub const COIN_TYPES: &[(&str, f64)] = &[ 
    ("Sovereign", 7.32),                    // grams of gold
    ("Britannia", 31.10),
    ("Krugerrand", 33.93), 
    ("American Eagle", 33.93),
    ("Canadian Maple Leaf", 31.10),
    ("Austrian Philharmonic", 31.10),
    ("Chinese Panda", 30.00),
    ("Australian Kangaroo", 31.10),
];

pub fn select_coin_type() -> Result<(String, Option<f64>), Box<dyn std::error::Error>> {
    let mut options: Vec<String> = COIN_TYPES.iter()
        .map(|(name, grams)| format!("{} ({:.2}g gold)", name, grams))
        .collect();
    options.push("Other (custom)".to_string());
    
    let selection = Select::new("Select coin type:", options)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .with_page_size(15)
        .prompt()?;
    
    if selection == "Other (custom)" {
        let custom = inquire::Text::new("Enter custom coin type:")
            .prompt()?;
        Ok((custom, None))  // No predefined gold content for custom
    } else {
        // Find the selected coin and return name + gold content
        for (name, grams) in COIN_TYPES {
            if selection.starts_with(name) {
                return Ok((name.to_string(), Some(*grams)));
            }
        }
        // Fallback (shouldn't happen)
        Ok((selection, None))
    }
}

// Helper function to get gold content by name
pub fn get_gold_content(coin_name: &str) -> Option<f64> {
    COIN_TYPES.iter()
        .find(|(name, _)| *name == coin_name)
        .map(|(_, grams)| *grams)
}