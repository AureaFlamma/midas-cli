use std::path::PathBuf;

use rusqlite::{params, Connection, Result};

use crate::types::GoldHolding;

pub fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let mut path = PathBuf::from(home);
    path.push(".midas-cli");
    path.push("holdings.db");
    path
}

pub fn init_db() -> Result<Connection> {
    //TODO: rename
    let db_path = get_db_path();

    // Create directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("Failed to create directory: {}", e)),
            )
        })?;
    }

    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS holdings (
            uid TEXT PRIMARY KEY,
            coin_type TEXT NOT NULL,
            coin_year TEXT NOT NULL,
            gold_content REAL NOT NULL,
            purchase_date TEXT NOT NULL,
            purchase_price REAL NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

// Load holdings from JSON file
pub fn load_holdings() -> Result<Vec<GoldHolding>, Box<dyn std::error::Error>> {
    let conn = init_db()?;

    let mut stmt = conn.prepare(
        "SELECT uid, coin_type, coin_year, gold_content, purchase_date, purchase_price FROM holdings"
    )?;

    let holding_iter = stmt.query_map([], |row| {
        Ok(GoldHolding {
            uid: row.get(0)?,
            coin_type: row.get(1)?,
            coin_year: row.get(2)?,
            gold_content: row.get(3)?,
            purchase_date: row.get(4)?,
            purchase_price: row.get(5)?,
        })
    })?;

    let mut holdings = Vec::new();
    for holding in holding_iter {
        holdings.push(holding?);
    }

    Ok(holdings)
}

// Save holdings to JSON file
pub fn save_holding(holding: &GoldHolding) -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db()?;

    conn.execute(
        "INSERT INTO holdings (uid, coin_type, coin_year, gold_content, purchase_date, purchase_price)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            holding.uid,
            holding.coin_type,
            holding.coin_year,
            holding.gold_content,
            holding.purchase_date,
            holding.purchase_price
        ],
    )?;

    Ok(())
}

pub fn delete_holdings_from_db(ids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db()?;

    for id in ids {
        conn.execute("DELETE FROM holdings WHERE uid = ?1", params![id])?;
    }

    Ok(())
}

pub fn delete_all_holdings() -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db()?;
    let deleted = conn.execute("DELETE from holdings", [])?;
    if deleted == 0 {
        println!("No holdings to delete");
    } else {
        println!("All {} holdings deleted", deleted);
    }
    Ok(())
}

pub fn get_holding_count() -> Result<usize, Box<dyn std::error::Error>> {
    let conn = init_db()?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM holdings", [], |row| row.get(0))?;
    Ok(count as usize)
}

pub struct SortPreference {
    pub column: String,
    pub ascending: bool,
}

pub fn save_sort_preference_to_db(
    column: &str,
    ascending: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_preferences (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    let sort_value = format!("{}:{}", column, if ascending { "asc" } else { "desc" }); // We are saving as unified string, but later we're spillit it anyway. Why not different format?
    conn.execute(
        "INSERT OR REPLACE INTO user_preferences (key, value) VALUES ('sort_preference', ?1)",
        params![sort_value], //params! might be superfluous
    )?;

    Ok(())
}

pub fn load_sort_preference() -> Result<Option<SortPreference>, Box<dyn std::error::Error>> {
    let conn = init_db()?;

    let result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT value FROM user_preferences WHERE key = 'sort_preference'",
        [],
        |row| row.get(0), // TODO: Error handling for more than one
    );

    match result {
        Ok(value) => {
            let parts: Vec<&str> = value.split(':').collect();
            if parts.len() == 2 {
                // TODO: Error handling for more than 2
                Ok(Some(SortPreference {
                    column: parts[0].to_string(),
                    ascending: parts[1] == "asc", // rename to "order" or "direction"
                }))
            } else {
                Ok(None)
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn load_holdings_sorted(
    column: &str,
    asc: bool,
) -> Result<Vec<GoldHolding>, Box<dyn std::error::Error>> {
    let col = match column {
        "uid" | "coin_type" | "coin_year" | "gold_content" | "purchase_date" | "purchase_price" => {
            column
        } // TODO: Should be abstracted away for easy updating
        _ => {
            return Err(Box::new(std::io::Error::new(
                // Why the nesting?
                std::io::ErrorKind::InvalidInput,
                format!("Invalid sort column: {}", column),
            )));
        }
    };

    let order = if asc { "ASC" } else { "DESC" }; // Why r we storing it as one string above, but as two params here?
    let sql = format!(
        "SELECT uid, coin_type, coin_year, gold_content, purchase_date, purchase_price \
         FROM holdings ORDER BY {} {}",
        col, order
    );

    let conn = init_db()?;
    let mut stmt = conn.prepare(&sql)?;

    let holding_iter = stmt.query_map([], |row| {
        Ok(GoldHolding {
            // must this be wrapped in an okay?
            uid: row.get(0)?, // this can also use named params.
            coin_type: row.get(1)?,
            coin_year: row.get(2)?,
            gold_content: row.get(3)?,
            purchase_date: row.get(4)?,
            purchase_price: row.get(5)?,
        })
    })?;

    let mut holdings = Vec::new();
    for holding in holding_iter {
        holdings.push(holding?);
    }

    Ok(holdings)
}

pub fn load_holdings_with_preference() -> Result<Vec<GoldHolding>, Box<dyn std::error::Error>> {
    match load_sort_preference()? {
        Some(pref) => load_holdings_sorted(&pref.column, pref.ascending),
        None => load_holdings(), // fallback to unsorted if no preference saved
    }
}
