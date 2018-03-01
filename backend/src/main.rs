#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate csrf;
extern crate data_encoding;
#[macro_use]
extern crate diesel;
extern crate dotenv;
//auth crate to handle session handling
mod auth;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let mut app = rocket::ignite();
    let mut mounted_app = app.mount("/", routes![index]);

    mounted_app.launch();    
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}