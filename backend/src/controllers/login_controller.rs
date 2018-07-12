use datalayer::{self};
use serde_json;
use ::app_state::AppState;
use rocket::State;
use rocket::http::{Cookie, Cookies, Status, SameSite};
use rocket_contrib::Json;
use models::login_request::LoginRequest;
use models::user::User;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatedCredentials {
    pub username: String,
    pub display_name: Option<String>
}

impl<'a> From<&'a User> for ValidatedCredentials {
    fn from(user: &'a User) -> ValidatedCredentials {
        ValidatedCredentials {
            username: user.username.clone(),
            display_name: user.display_name.clone(),
        }
    }
}

impl FromData for ValidatedCredentials {
    type Error = ();

     fn from_data(request: &Request, data: Data) -> DataOutcome<Self, ()> {
         let login_request_form = match Form::<LoginRequest>::from_data(request, data) {
             Outcome::Success(req) => req,
             _ => return Outcome::Failure((Status::BadRequest, ()))
         };
         let login_request = login_request_form.into_inner();
         let state = match request.guard::<State<AppState>>().succeeded() {
             Some(state) => state,
             None => return Outcome::Failure((Status::InternalServerError, ()))
         };
         let user = match datalayer::get_user_from_request(&login_request, state.inner()) {
             Ok(user) => user,
             _ => return Outcome::Failure((Status::Unauthorized, ()))
         };
         Outcome::Success(ValidatedCredentials::from(&user))

     }
}

#[post("/login", data = "<login>")]
pub fn login(app_state: State<AppState>, 
            mut cookies: Cookies, 
            login: ValidatedCredentials) -> Json<ValidatedCredentials> {
    let mut username_cookie = Cookie::new("beamed-username", login.username.clone());
    initialize_cookie(&mut username_cookie);
    cookies.add(username_cookie);
    if let Some(display_name) = &login.display_name {
        let mut display_cookie = Cookie::new("beamed-displayname", display_name.clone());
        initialize_cookie(&mut display_cookie);
        cookies.add(display_cookie);
    }
    Json(login)
}

fn initialize_cookie(cookie: &mut Cookie) {
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Lax);
}