use inquire::{Select, Text};

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
pub const COIN_TYPES: &[(&str, f64, &str)] = &[
    ("Sovereign", 7.32, "sov"), // grams of gold
    ("Britannia", 31.10, "brt"),
    ("Krugerrand", 33.93, "kur"),
    ("American Eagle", 33.93, "eag"),
    ("Canadian Maple Leaf", 31.10, "mpl"),
    ("Austrian Philharmonic", 31.10, "phi"),
    ("Chinese Panda", 30.00, "pan"),
    ("Australian Kangaroo", 31.10, "kan"),
]; // TODO: A struct may be more appropriate here.

pub fn select_coin_type() -> Result<(String, f64, String), Box<dyn std::error::Error>> {
    let mut options: Vec<String> = COIN_TYPES
        .iter()
        .map(|(name, grams, _)| format!("{} ({:.2}g gold)", name, grams))
        .collect();
    options.push("Other (custom)".to_string());

    let selection = Select::new("Select coin type:", options)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .with_page_size(15)
        .prompt()?;

    if selection == "Other (custom)" {
        let custom_name = Text::new("Enter custom coin type:").prompt()?;

        let custom_code = format!(
            "{:0<3}",
            custom_name
                .chars()
                .filter(|c| c.is_alphanumeric())
                .take(3)
                .collect::<String>()
                .to_lowercase()
                .to_lowercase()
        );

        // Get custom gold content
        let custom_grams = loop {
            let custom_grams_str = Text::new("Enter gold content (grams):")
                .with_help_message("Enter the amount of gold in grams (e.g., 31.10)")
                .prompt()?;

            match custom_grams_str.parse::<f64>() {
                Ok(custom_grams) if custom_grams > 0.0 => break custom_grams, // Return f64 directly
                Ok(_) => println!("Gold content must be greater than 0"),
                Err(_) => println!("Invalid number format. Please enter a valid number"),
            }
        };

        Ok((custom_name, custom_grams, custom_code)) // Both are guaranteed values
    } else {
        // Find the selected coin and return name + gold content
        for (name, grams, code) in COIN_TYPES {
            if selection.starts_with(name) {
                return Ok((name.to_string(), *grams, code.to_string())); // Return f64 directly
            }
        }
        // Fallback (shouldn't happen) - but we need to handle it
        Err("Unable to find selected coin".into())
    }
}

// Helper function to get gold content by name
pub fn get_gold_content(coin_name: &str) -> Option<f64> {
    COIN_TYPES
        .iter()
        .find(|(name, _, _)| *name == coin_name) // Need 3 elements, not 2
        .map(|(_, grams, _)| *grams) // Need 3 elements, not 2
}
