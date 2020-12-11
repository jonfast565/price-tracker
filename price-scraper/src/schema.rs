table! {
    items (id) {
        id -> Int4,
        item_param -> Varchar,
        item_name -> Varchar,
        item_url -> Varchar,
        created_by -> Varchar,
        created_datetime -> Timestamp,
        last_modified_by -> Varchar,
        last_modified_datetime -> Timestamp,
        vendor -> Varchar,
    }
}

table! {
    prices (id) {
        id -> Int4,
        parent_item_id -> Int4,
        price -> Numeric,
        current -> Bool,
        created_by -> Varchar,
        created_datetime -> Timestamp,
        last_modified_by -> Varchar,
        last_modified_datetime -> Timestamp,
    }
}

joinable!(prices -> items (parent_item_id));

allow_tables_to_appear_in_same_query!(
    items,
    prices,
);
