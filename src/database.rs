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
            coin_year INTEGER NOT NULL,
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

pub fn get_holding_count() -> Result<usize, Box<dyn std::error::Error>> {
    let conn = init_db()?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM holdings", [], |row| row.get(0))?;
    Ok(count as usize)
}
