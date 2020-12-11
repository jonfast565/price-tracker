#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate logger;

mod models;
mod schema;
mod utilities;

use crate::models::{Item, NewPrice, Price};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn get_items() -> Vec<Item> {
    use crate::schema::items::dsl::*;

    let connection = establish_connection();
    let result_items = items
        .load::<Item>(&connection)
        .expect("Error loading items");

    logger::info(format!("Found {} items from database", result_items.len()));
    result_items.to_owned()
}

pub fn update_price(input_product_id: String, new_price: String) {
    use crate::schema::items::dsl::*;
    use crate::schema::prices::dsl::*;

    let connection = establish_connection();

    let first_product = items
        .filter(item_param.eq(input_product_id))
        .first::<Item>(&connection)
        .expect("Error loading item");

    let current_prices = prices.filter(parent_item_id.eq(first_product.id));

    let new_prices = prices
        .filter(parent_item_id.eq(first_product.id))
        .order_by(crate::schema::prices::last_modified_datetime.desc())
        .load::<Price>(&connection)
        .expect("Error loading prices");

    match new_prices.first() {
        Some(record) => {
            // Insert new price if it is different
            let new_price_bd = crate::utilities::bigdecimal_from_price(&new_price);
            if record.price != new_price_bd {
                logger::info(format!("Insert new price: {}", new_price.to_string()));
                diesel::update(current_prices)
                .set(current.eq(false))
                .execute(&connection)
                .expect("Error clearing current prices for this object");

                diesel::insert_into(prices)
                .values(NewPrice {
                    parent_item_id: record.parent_item_id,
                    price: new_price_bd,
                    current: true
                })
                .execute(&connection)
                .expect("Price not inserted");
            } else {
                logger::info_static("Prices already up to date");
            }
        }
        None => {
            // Insert the new price alone
            logger::info(format!("Insert new price: {}", new_price.to_string()));
            let new_price_bd = crate::utilities::bigdecimal_from_price(&new_price);
            
            diesel::update(current_prices)
            .set(current.eq(false))
            .execute(&connection)
            .expect("Error clearing current prices for this object");

            diesel::insert_into(prices)
            .values(NewPrice {
                parent_item_id: first_product.id,
                price: new_price_bd,
                current: true
            })
            .execute(&connection)
            .expect("Price not inserted");
        },
    };
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
