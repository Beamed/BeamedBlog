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
use actix_web::{http, HttpRequest, Responder, App, server};
use actix_web::middleware::csrf as actix_csrf;

//auth mod to handle session handling
mod auth;
//db mod to handle db interaction 
mod db;
mod controllers;

use dotenv::dotenv;



fn main() {
    dotenv().ok();
    
    let mut server = server::new(||
        App::new().middleware(
            actix_csrf::CsrfFilter::new().allowed_origin("https://thebeamed.com/").allowed_origin("http://localhost:8080/")
        ).resource("/api/login", |r| {
            r.method(http::Method::POST).f(controllers::login_controller::handle_login);
        }).resource("/", |r| {
            r.method(http::Method::GET).f(controllers::index)
        })
    );
    server = server.bind("127.0.0.1:8080").expect("Could not bind to 127.0.0.1:8080");
    info!("Bound to port 8080. Initializing..");
    log4rs::init_file("conf/log4rs.yml", Default::default()).expect("Unable to initialize logging");
    server.run();
    //let mut app = rocket::ignite().manage(db::init_pool());
    //let mut mounted_app = app.mount("/", routes![index, auth::login]);
    //mounted_app.launch();    
}
