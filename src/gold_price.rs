use serde::Deserialize;
use std::env;
use std::error::Error;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::constants::PRICE_CACHE_DURATION;

#[derive(Deserialize, Debug)]
pub struct GoldPriceResponse {
    pub price_gram_24k: f64, // Price per gram
}

struct CachedPrice {
    price: f64,
    timestamp: Instant,
}

static CACHE: Mutex<Option<CachedPrice>> = Mutex::new(None);
const CACHE_DURATION: Duration = Duration::from_secs(PRICE_CACHE_DURATION); // 6h in seconds

pub async fn fetch_gold_price_gbp() -> Result<f64, Box<dyn Error>> {
    {
        let cache = CACHE.lock().unwrap();
        if let Some(cached) = &*cache {
            if cached.timestamp.elapsed() < CACHE_DURATION {
                return Ok(cached.price);
            }
        }
    }
    let api_key =
        env::var("GOLDAPI_TOKEN").map_err(|_| "GOLDAPI_TOKEN environment variable not set")?;
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.goldapi.io/api/XAU/GBP")
        .header("x-access-token", api_key)
        .send()
        .await?;

    let price_data: GoldPriceResponse = response.json().await?;

    let gbp_per_gram = price_data.price_gram_24k;

    {
        let mut cache = CACHE.lock().unwrap();
        *cache = Some(CachedPrice {
            price: gbp_per_gram,
            timestamp: Instant::now(),
        });
    }

    Ok(gbp_per_gram)
}
