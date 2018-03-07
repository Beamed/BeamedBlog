extern crate bcrypt;

use rocket;
#[macro_use()]
use rocket::data::FromData;
use rocket::data::Outcome;
use rocket::Outcome::{Success, Forward};
use rocket::outcome::IntoOutcome;
use rocket::{Data};
use rocket_contrib::Json;
use rocket_contrib::json::SerdeError;
use rocket::response::{Failure};
use rocket::request::{Request, FromRequest};
use rocket::State;
use rocket::http::Status;
use serde_derive::*;
use rocket::outcome::Outcome::Failure as OutcomeFailure;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use data_encoding::BASE64;
use db::{DbConn, Pool};
use db::user::User;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidLogin {
    username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatedCredentials {
    username: String,
    display_name: String,
    csrf_token: String
}


impl FromData for ValidLogin {
    type Error = Failure;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, Failure> {
        let login_request = Json::<LoginRequest>::from_data(req, data);
        if let Success(parsed_request) = login_request {
            let conn = {
                let fetched_conn = DbConn::from_request(req);
                match fetched_conn {
                    Success(conn) => conn,
                    OutcomeFailure(e) => {
                        error!("Unable to fetch connection to DB.");
                        return OutcomeFailure((Status::InternalServerError, Failure::from(Status::InternalServerError)));
                    },
                    Forward(_) => {
                        error!("Unable to fetch connection to DB, received unexpected: Forward");
                        return OutcomeFailure((Status::InternalServerError, Failure::from(Status::InternalServerError)));
                    }
                }
            };
            if validate_password(&parsed_request, conn) {
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
pub fn login(conn: DbConn, validated_user: ValidLogin) -> Json<ValidatedCredentials> {
    Json(ValidatedCredentials {
        username: validated_user.username,
        display_name: "Test".to_string(),
        csrf_token: "Test".to_string()
    })
}

fn validate_password(login_request: &LoginRequest, conn: DbConn) -> bool {
    use db::schema::users::dsl::*;
    use diesel::prelude::*;
    use crypto::md5::Md5;
    use crypto::digest::Digest;

    let user_row = {
        let mut user_rows = match users.filter(username.eq(&login_request.username)).load::<User>(&*conn) {
            Ok(user) => user, 
            Err(e) => return false
        };
        if user_rows.len() != 1 {
            debug!("user_rows length unexpected value for username: {}, found: {}", login_request.username , user_rows.len());
            return false
        } else {
            user_rows.remove(0)
        }
    };
    let mut md5_hasher = Md5::new();
    let salt = match env::var("PASS_SALT") {
        Ok(salt) => salt,
        Err(e) => {
            error!("Could not find DB salt");
            return false;
        }
    };
    md5_hasher.input(salt.as_bytes());
    md5_hasher.input(login_request.password.as_bytes());
    md5_hasher.input(salt.as_bytes());
    let expected_pass = md5_hasher.result_str();
    user_row.password.eq(&expected_pass)
}
