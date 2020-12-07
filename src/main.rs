extern crate scraper;

#[macro_use]
extern crate diesel;

mod database;
mod log;
mod models;
mod price_finder;
mod schema;
mod utilities;

use futures::future::join_all;
use price_finder::PriceFinder;
use tracing::instrument;
use tracing_subscriber;

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    let amazon_price_finder = price_finder::AmazonPriceFinder {};
    let items = database::get_items();

    let mut v = Vec::new();
    for item in items {
        let price = amazon_price_finder.read_price(item.item_param);
        v.push(price);
    }
    let vres = join_all(v).await;

    for res in vres {
        match res {
            Ok(price) => log::info(format!("{}", price)),
            Err(_err) => log::error_static("Price could not be found"),
        }
    }

    Ok(())
}

fn setup_logging() {
    tracing_subscriber::fmt::init();
    log::display_header();
}
