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
extern crate log4rs;
extern crate crypto;
extern crate time;
extern crate actix_web;
use actix_web::{HttpRequest, Responder, App, server};

//auth mod to handle session handling
mod auth;
//db mod to handle db interaction 
mod db;
mod controllers;

use dotenv::dotenv;



fn main() {
    dotenv().ok();
    //let mut app = rocket::ignite().manage(db::init_pool());
    //let mut mounted_app = app.mount("/", routes![index, auth::login]);
    //mounted_app.launch();    
}
