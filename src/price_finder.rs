extern crate scraper;
use async_trait::async_trait;
use scraper::{Html, Selector};

#[async_trait]
pub trait PriceFinder {
    async fn read_price(&self, product_id: String) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct AmazonPriceFinder {}

#[async_trait]
impl PriceFinder for AmazonPriceFinder {
    async fn read_price(&self, product_id: String) -> Result<String, Box<dyn std::error::Error>> {
        self.read_amazon_price(&product_id).await
    }
}

impl AmazonPriceFinder {
    fn build_amazon_url(&self, product_id: &str) -> String {
        let amazon_product_url = "https://www.amazon.com/gp/product/";
        let mut product_id_string = String::new();
        product_id_string.push_str(amazon_product_url);
        product_id_string.push_str(product_id);
        super::log::info(format!("Amazon Url: {}", &product_id_string));
        product_id_string
    }

    async fn read_amazon_price(
        &self,
        product_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        super::log::info_static("Getting amazon page");
        let amazon_url = self.build_amazon_url(product_id);
        let document = super::utilities::simple_get_request(&amazon_url).await?;
        let body = Html::parse_document(&document);
        let price_block_selector = match Selector::parse("span#priceblock_ourprice") {
            Ok(result) => result,
            Err(_err) => {
                panic!("Could not parse span#priceblock_ourprice")
            }
        };
        let price_block_deal_selector = match Selector::parse("span#priceblock_dealprice") {
            Ok(result) => result,
            Err(_err) => {
                panic!("Could not parse span#priceblock_dealprice")
            }
        };

        super::log::info_static("Finding price");

        let our_price = body.select(&price_block_selector).next();
        let our_price_string = match our_price {
            Some(val) => val.inner_html(),
            None => String::new(),
        };

        let deal_price = body.select(&price_block_deal_selector).next();
        let deal_price_string = match deal_price {
            Some(val) => val.inner_html(),
            None => String::new(),
        };

        if our_price_string.is_empty() {
            return Ok(deal_price_string.to_owned());
        }

        Ok(our_price_string.to_owned())
    }
}
