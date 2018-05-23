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
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate crypto;
extern crate time;
//auth mod to handle session handling
mod auth;
//db mod to handle db interaction 
mod db;

use dotenv::dotenv;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();
    let mut app = rocket::ignite().manage(db::init_pool());
    let mut mounted_app = app.mount("/", routes![index, auth::login]);
    mounted_app.launch();    
}
