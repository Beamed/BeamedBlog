extern crate bcrypt;

use std::time::{Duration};
use time;
use serde_derive::*;
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
/*
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
*/

