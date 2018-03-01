CREATE TABLE Sessions (
    id SERIAL PRIMARY KEY,
    user_id SERIAL,
    csrf_token VARCHAR,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);

