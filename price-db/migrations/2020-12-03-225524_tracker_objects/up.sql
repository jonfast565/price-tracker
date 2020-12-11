-- Your SQL goes here
CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  item_param varchar NOT NULL,
  item_name varchar NOT NULL,
  item_url varchar NOT NULL,
  created_by varchar NOT NULL DEFAULT current_user,
  created_datetime timestamp NOT NULL DEFAULT current_date,
  last_modified_by varchar NOT NULL DEFAULT current_user,
  last_modified_datetime timestamp NOT NULL DEFAULT current_date
);

CREATE TABLE prices (
  id SERIAL PRIMARY KEY,
  parent_item_id INT NOT NULL REFERENCES items(id),
  price money NOT NULL,
  current boolean NOT NULL DEFAULT 'f',
  created_by varchar NOT NULL DEFAULT current_user,
  created_datetime timestamp NOT NULL DEFAULT current_date,
  last_modified_by varchar NOT NULL DEFAULT current_user,
  last_modified_datetime timestamp NOT NULL DEFAULT current_date
);