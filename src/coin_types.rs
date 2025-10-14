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

pub fn display_coin_options() {
    println!("Available coin types:");
    for (index, coin_type) in COIN_TYPES.iter().enumerate() {
        println!("  {}. {}", index + 1, coin_type);
    }
    println!("  {}. Other (custom)", COIN_TYPES.len() + 1);
}

pub fn get_coin_type_by_index(index: usize) -> Option<&'static str> {
    if index > 0 && index <= COIN_TYPES.len() {
        Some(COIN_TYPES[index - 1])
    } else {
        None
    }
}