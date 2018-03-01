extern crate bcrypt;

use rocket;
#[macro_use()]
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
use data_encoding::BASE64;

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValidLogin {
    username: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ValidatedCredentials {
    username: String,
    display_name: String,
    csrf_token: String
}


impl FromData for ValidLogin {
    type Error = Failure;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, Failure> {
        let login_request = Json::<LoginRequest>::from_data(req, data);
        if let Success(parsed_request) = login_request {
            if validate_password(&parsed_request) {
                Success(ValidLogin{username: parsed_request.username.clone()})
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
    Json(ValidatedCredentials {
        username: validated_user.username,
        display_name: "Test".to_string(),
        csrf_token: "Test".to_string()
    })
}

fn validate_password(login_request: &LoginRequest) -> bool {
    true
}
