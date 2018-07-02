use datalayer::{self, ValidatedCredentials, LoginRequest};
use serde_json;
use ::app_state::AppState;
use ::http::{Error};
use ::http;
use std::fmt::Display;
use std::fmt;
use std::error::Error as StdError;


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
