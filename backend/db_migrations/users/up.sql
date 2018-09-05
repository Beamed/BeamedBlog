CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    display_name VARCHAR,
    author BOOLEAN
);

