extern crate bcrypt;

use rocket;
use rocket::data::FromData;
use rocket::data::Outcome;
use rocket::Outcome::Success;
use rocket::outcome::IntoOutcome;
use rocket::{Data};
use rocket_contrib::Json;
use rocket_contrib::json::SerdeError;
use rocket::response::{Failure};
use rocket::request::{Request};
use rocket::http::Status;
use serde_derive::*;
use rocket::outcome::Outcome::Failure as OutcomeFailure;
use csrf::{AesGcmCsrfProtection, CsrfProtection};

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValidLogin {
    username: String
}

struct ValidatedCredentials {
    username: String,
    display_name: String,
    csrf_token: AesGcmCsrfProtection
}

impl FromData for ValidLogin {
    type Error = Failure;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, Failure> {
        let login_request = Json::<LoginRequest>::from_data(req, data);
        if let Success(parsed_request) = login_request {
            if validate_password(parsed_request) {

                Success(ValidLogin{username: String::new()})
            } else {
                OutcomeFailure((Status::Unauthorized, Failure::from(Status::Unauthorized)))
            }
        } else {
            OutcomeFailure((Status::Unauthorized, Failure::from(Status::Unauthorized)))
        }
    }
}

#[post("/login", format = "application/json", data = "<validated_user>")]
fn login(validated_user: ValidLogin) -> Json<ValidatedCredentials> {
    "Hello, world!"
}

fn validate_password(login_request: &LoginRequest) -> bool {
    
}
