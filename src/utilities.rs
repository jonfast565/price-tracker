extern crate reqwest;
use diesel::pg::data_types::PgNumeric;
use substring::Substring;
use std::str::FromStr;
use bigdecimal::{BigDecimal};

pub async fn simple_get_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    assert!(resp.status().is_success());
    let resp_text = resp.text().await?;
    super::log::info(format!("{}", resp_text.substring(0, 50)));
    Ok(resp_text)
}

pub fn remove_dollar_sign(price: &str) -> String {
    price.trim_start_matches('$').to_string()
}

pub fn bigdecimal_from_price(price: &str) -> BigDecimal {
    let price_sans_dollar = remove_dollar_sign(price);
    match BigDecimal::from_str(price_sans_dollar.as_str()) {
        Ok(result) => result,
        Err(_) => panic!("Price to big decimal failed")
    }
}