mod user_controller;
use ::app_state::AppState;

use ::rocket::{Rocket, ignite};

#[get("/")]
pub fn index() -> String {
    "This is the Beamed blog API. This endpoint is probably not what you want.".to_string()
}

pub fn initialize_rocket() -> ::rocket::Rocket {
    ignite()
        .mount("/api", routes![index,
                                 user_controller::login,
                                 user_controller::register
                                 ])
}