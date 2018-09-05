CREATE TABLE Posts {
    id SERIAL PRIMARY KEY,
    creator SERIAL FOREIGN KEY,
    created TIMESTAMP,
    title TEXT,
    body TEXT
};