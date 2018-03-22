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
use rocket::http::{Status, Cookies, Cookie};
use std::time::{Duration};
use time;
use serde_derive::*;
use rocket::outcome::Outcome::Failure as OutcomeFailure;
use csrf::{AesGcmCsrfProtection, CsrfProtection};
use db::{DbConn, Pool};
use db::user::User;
use std::env;
use std::error::Error;
use data_encoding::{BASE64, DecodeError};
use self::bcrypt::{DEFAULT_COST, hash, verify};

mod autherror;
use self::autherror::AuthError;

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidLogin {
    user: User
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatedCredentials {
    username: String,
    display_name: Option<String>
}



impl FromData for ValidLogin {
    type Error = Failure;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, Failure> {
        use db::schema::users::dsl::*;
        use diesel::prelude::*;
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
            let user_row = {
                let mut user_rows = match users.filter(username.eq(&parsed_request.username)).load::<User>(&*conn) {
                    Ok(user) => user, 
                    Err(e) => {
                        error!("Unable to fetch user rows from DB.");
                        return OutcomeFailure((Status::InternalServerError, Failure::from(Status::InternalServerError)));
                    }
                };
                if user_rows.len() != 1 {
                    debug!("user_rows length unexpected value for username: {}, found: {}", parsed_request.username , user_rows.len());
                    return OutcomeFailure((Status::Unauthorized, Failure::from(Status::Unauthorized)));
                } else {
                    user_rows.remove(0)
                }
            };
            if validate_password(&parsed_request, conn, &user_row) {
                let mut cookies = req.cookies();
                let token = generate_csrf_token();
                let mut new_cookie = Cookie::new("token", token);
                new_cookie.set_path("/");
                let mut expiry_time = time::now();
                expiry_time.tm_hour += 1;
                new_cookie.set_expires(expiry_time);
                cookies.add(new_cookie);
                Success(ValidLogin{user: user_row})
            } else {
                OutcomeFailure((Status::Unauthorized, Failure::from(Status::Unauthorized)))
            }
        } else {
            OutcomeFailure((Status::Unauthorized, Failure::from(Status::Unauthorized)))
        }
    }
}

#[post("/login", format = "application/json", data = "<validated_user>")]
pub fn login(validated_user: ValidLogin) -> Json<ValidatedCredentials> {
    Json(ValidatedCredentials {
        username: validated_user.user.username,
        display_name: validated_user.user.display_name
    })
}

fn validate_password(login_request: &LoginRequest, conn: DbConn, user: &User) -> bool {
    match verify(&login_request.password, &user.password) {
        Ok(verified) => verified,
        Err(_) => false
    }
}

fn generate_csrf_token() -> String {
    let mut aes_key :[u8; 32] = Default::default();
    aes_key.copy_from_slice(env::var("AES_KEY").expect("Unable to fetch crypto AES key").as_bytes());
    let ttl = env::var("SESSION_TIMEOUT").expect("Unable to determine sesh timeout");
    let protect = AesGcmCsrfProtection::from_key(aes_key);
    let (cookie, token) = protect.generate_token_pair(None, ttl.parse::<i64>().expect("Unable to parse sesh timeout")).expect("Catastrophic cryptographic error");
    cookie.b64_string()
}

fn validate_token(cookie: Cookie, token: String) -> Result<bool, Box<Error>> {
    let c_token : String = String::from(cookie.value());
    let token_bytes = BASE64.decode(token.as_bytes())?;
    let cookie_bytes = BASE64.decode(c_token.as_bytes())?;
    let mut aes_key :[u8; 32] = Default::default();
    aes_key.copy_from_slice(env::var("AES_KEY").expect("Unable to fetch crypto AES key").as_bytes());
    let protect = AesGcmCsrfProtection::from_key(aes_key);
    let parsed_token = protect.parse_token(&token_bytes)?;
    let parsed_cookie = protect.parse_cookie(&cookie_bytes)?;

    Ok(protect.verify_token_pair(&parsed_token, &parsed_cookie))
}
