use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct GoldPriceResponse {
    pub price_gram_24k: f64, // Price per gram
}

pub async fn fetch_gold_price_gbp() -> Result<f64, Box<dyn Error>> {
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

    Ok(gbp_per_gram)
}
