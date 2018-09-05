mod user_controller;
use ::app_state::AppState;
use ::rocket::{Rocket, ignite};
use models::user;
use rocket::request::{self, Request, Form, FromForm, FromRequest};
use rocket::http::{Cookie, Cookies, Status, SameSite};
use rocket::Outcome;
use models::user::{User, UserForm, ValidatedCredentials, LoginRequest};
use rocket::data::{self, Data, FromData};
use rocket::State;
use rocket::data::Outcome as DataOutcome;
use datalayer::user_data_layer;

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

impl FromData for ValidatedCredentials {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> DataOutcome<Self, ()> {
        let login_request_form = match Form::<LoginRequest>::from_data(request, data) {
            Outcome::Success(req) => req,
            _ => return Outcome::Failure((Status::BadRequest, ()))
        };
        let login_request = login_request_form.into_inner();
        let state = match get_state_from_request(request) {
            Ok(state) => state,
            Err(_) => return Outcome::Failure((Status::InternalServerError, ()))
        };
        let user = match user_data_layer::get_user_from_request(state, &login_request ) {
            Ok(user) => user,
            _ => return Outcome::Failure((Status::Unauthorized, ()))
        };
        Outcome::Success(ValidatedCredentials::from(&user))

    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ValidatedCredentials {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ValidatedCredentials, ()> {
        if let Some(username_cookie) = request.cookies().get(user::USERNAME_COOKIE) {
            let state = match get_state_from_request(request) {
                Ok(state) => state,
                Err(_) => return Outcome::Failure((Status::InternalServerError, ()))
            };            let (_, username) = username_cookie.name_value();
            let user = match user_data_layer::get_user_from_username(state, &username.to_string()) {
                Ok(user) => user,
                _ => return Outcome::Failure((Status::Unauthorized, ()))
            };
            return Outcome::Success(ValidatedCredentials::from(&user));
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

pub fn get_state_from_request<'a, 'r>(request: &'a Request<'r>) -> Result<&'a AppState, ()> {
    match request.guard::<State<AppState>>().succeeded() {
        Some(state) => Ok(state.inner()),
        None => Err(())
    }
}