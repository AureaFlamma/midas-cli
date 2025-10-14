use inquire::Select;

/*   A. Why &str rather than String?
 * cont context requires the values to be known at compile time. String literals live in the binary, so are known at CT. 
 * In contrast, String::from() does a bunch of things at runtime (allocate heap memory, write to it, return the String struct)
 * 
 *   B. Why &[] rather than []?
 * String slices do not have a set length. Below we are pushing to the slice, which we couldn't do with an array.
 * 
 *   C. Why explicit annotation?
 * Rust requires explicit type annotation for const and static items.
 */
pub const COIN_TYPES: &[&str] = &[ 
    "Sovereign",
    "Britannia",
    "Krugerrand", 
    "American Eagle",
    "Canadian Maple Leaf",
    "Austrian Philharmonic",
    "Chinese Panda",
    "Australian Kangaroo",
];

pub fn select_coin_type() -> Result<String, Box<dyn std::error::Error>> {
    let mut options: Vec<String> = COIN_TYPES.iter().map(|&s| s.to_string()).collect();
    options.push("Other (custom)".to_string());
    
    let selection = Select::new("Select coin type:", options)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .with_page_size(15)
        .prompt()?;
    
    if selection == "Other (custom)" {
        let custom = inquire::Text::new("Enter custom coin type:")
            .prompt()?;
        Ok(custom)
    } else {
        Ok(selection)
    }
}