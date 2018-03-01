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

//auth crate to handle session handling
mod auth;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let mut app = rocket::ignite();
    app.mount("/", routes![index]);

    app.launch();    
}