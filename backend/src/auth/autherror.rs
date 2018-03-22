use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct AuthError;

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Authentication Error")
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        "Error authenticating user session token."
    }
    //no need ot track what is likely session expiring
    fn cause(&self) -> Option<&Error> {
        None
    }
}