extern crate scraper;
extern crate price_db;
extern crate logger;

mod price_finder;
mod utilities;

use futures::future::join_all;
use price_finder::PriceFinder;
use tracing::instrument;
use tracing_subscriber;

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    loop {
        process_loop().await?;
    }
    // Ok(())
}

async fn process_loop() -> Result<(), Box<dyn std::error::Error>> {
    let amazon_price_finder = price_finder::AmazonPriceFinder {};
    let items = price_db::get_items();

    let mut v = Vec::new();
    for item in items {
        let price = amazon_price_finder.read_price(item.item_param);
        v.push(price);
    }
    let vres = join_all(v).await;

    for res in vres {
        match res {
            Ok(result) => { 
                logger::info(format!("{}", result.price));
                price_db::update_price(result.product_id, result.price);
            },
            Err(_err) => logger::error_static("Price could not be found"),
        }
    }

    Ok(())
}

fn setup_logging() {
    tracing_subscriber::fmt::init();
    logger::display_header();
}
