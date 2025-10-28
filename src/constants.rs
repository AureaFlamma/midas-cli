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
]; // maybe a struct would be better - explicit labelling?
