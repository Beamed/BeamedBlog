#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(decl_macro)]
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate csrf;
extern crate data_encoding;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate crypto;
extern crate time;
extern crate cookie;
extern crate bcrypt;
extern crate hyper;
extern crate http;
#[macro_use]
extern crate rocket;
use cookie::SameSite;
use std::error::Error;

//just stuff how we get important stuff (users, posts, etc.) into a datalayer for now.
pub mod datalayer;
pub mod models;
pub mod controllers;

pub mod app_state;


use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let mut server = controllers::initialize_rocket();
    info!("Bound to port 8080. Initializing..");

    server.launch();


}
