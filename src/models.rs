use crate::schema::items;
use crate::schema::prices;
use chrono::NaiveDateTime;
use diesel::pg::data_types::{PgNumeric, PgTimestamp};

#[derive(Queryable, Clone)]
pub struct Item {
    pub id: i32,
    pub item_param: String,
    pub item_name: String,
    pub item_url: String,
    pub created_by: String,
    pub created_datetime: NaiveDateTime,
    pub modified_by: String,
    pub modified_datetime: NaiveDateTime,
    pub vendor: String,
}

#[derive(Insertable, Clone)]
#[table_name = "items"]
struct NewItem {
    pub vendor: String,
    pub item_param: String,
    pub item_name: String,
    pub item_url: String,
}

#[derive(Queryable, Clone)]
pub struct Price {
    pub id: i32,
    pub parent_item_id: i32,
    pub price: PgNumeric,
    pub current: bool,
    pub created_by: String,
    pub created_datetime: NaiveDateTime,
    pub modified_by: String,
    pub modified_datetime: NaiveDateTime,
}

#[derive(Insertable, Clone)]
#[table_name = "prices"]
struct NewPrice {
    parent_item_id: i32,
    price: PgNumeric,
    current: bool,
}
