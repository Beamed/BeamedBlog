use datalayer::user_data_layer;
use serde_json;
use ::app_state::AppState;
use rocket::State;
use rocket::http::{Cookie, Cookies, Status, SameSite};
use rocket_contrib::Json;
use models::user;
use models::user::{User, UserForm, ValidatedCredentials, LoginRequest};
use std::fmt::Display;
use std::fmt;
use std::error::Error as StdError;
use rocket::request::{self, Request, Form, FromForm, FromRequest};
use rocket::Outcome;
use rocket::response::Failure;
use rocket::data::{self, Data, FromData};
//it's really frustrating that rocket has multiple named types
//that are completely different types, and expects you to interact with them.
use rocket::data::Outcome as DataOutcome;
use rocket;

#[derive(Debug)]
struct ErrorUnauthorized {

}

impl StdError for ErrorUnauthorized {

}

impl Display for ErrorUnauthorized {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "User provided invalid credentials to login, or authentication is unavailable.")
    }
}

#[post("/login", data = "<login>")]
pub fn login(app_state: State<AppState>, 
            mut cookies: Cookies, 
            login: ValidatedCredentials) -> Json<ValidatedCredentials> {
    debug!("Logging in {}", login.username);
    let mut username_cookie = Cookie::new(user::USERNAME_COOKIE, login.username.clone());
    initialize_cookie(&mut username_cookie);
    cookies.add(username_cookie);
    if let Some(display_name) = &login.display_name {
        let mut display_cookie = Cookie::new(user::DISPLAY_COOKIE, display_name.clone());
        initialize_cookie(&mut display_cookie);
        cookies.add(display_cookie);
    }
    if login.author {
        let mut author_cookie = Cookie::new(user::AUTHOR_COOKIE, login.author.to_string());
        initialize_cookie(&mut author_cookie);
        cookies.add(author_cookie);
    }
    Json(login)
}

#[post("/register", data = "<user_info_json>")]
pub fn register(app_state: State<AppState>,
                mut cookies: Cookies,
                user_info_json: Json<UserForm>) ->
                Result<Json<ValidatedCredentials>, Status> {
        let user_info : UserForm = user_info_json.into_inner();
        debug!("Received register request for username: {}", user_info.username);
        match user_data_layer::create_new_user(&app_state, &user_info) {
            Ok(()) => {
                info!("Successfully created user. Logging in...");
                Ok(login(app_state, cookies,
                         ValidatedCredentials{
                             username: user_info.username, display_name: user_info.display_name, author: user_info.author
                         }))
            },
            Err(e) => {
                error!("Error creating new user: {}", e);
                Err(Status::BadRequest)
            }
        }

    }

fn initialize_cookie(cookie: &mut Cookie) {
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Lax);
}