mod login_controller;
mod utils;
use ::app_state::AppState;

use ::rocket::{Rocket, ignite};

#[get("/")]
pub fn index() -> String {
    "This is the Beamed blog API. This endpoint is probably not what you want.".to_string()
}

pub fn initialize_rocket() -> ::rocket::Rocket {
    return ignite()
        .mount("/api", routes![index])

}