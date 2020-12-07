extern crate reqwest;
use substring::Substring;

pub async fn simple_get_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    assert!(resp.status().is_success());
    let resp_text = resp.text().await?;
    super::log::info(format!("{}", resp_text.substring(0, 50)));
    Ok(resp_text)
}
