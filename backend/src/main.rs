#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(decl_macro)]
#![feature(custom_derive)]
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate cookie;
extern crate bcrypt;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;

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
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    let mut server = controllers::initialize_rocket();
    warn!("Bound to port 8080. Initializing..");

    server
        .manage(app_state::AppState::new())
        .launch();

    warn!("Shutting down..");
}
