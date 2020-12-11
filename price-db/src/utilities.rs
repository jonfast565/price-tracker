use bigdecimal::{BigDecimal};
use std::str::FromStr;

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