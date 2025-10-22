use rand::{rng, Rng};
use rand::distr::Alphanumeric;

pub fn construct_uid(code: &str, year: &str) -> String {
    fn generate_uid() -> String {
        rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            .take(4)
            .map(char::from)
            .collect()
    }
    
    let uid = generate_uid();
    format!("{}.{}.{}", code, year, uid)
}
// TODO: Think about final and component string validation. If needed, change return type 
// to Result