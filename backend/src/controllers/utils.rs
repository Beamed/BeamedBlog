use ::hyper::{Body, StatusCode, Response};
use serde::Deserialize;
use hyper::rt::{self, Future, Stream};
use hyper::Chunk;
use std::error::Error as StdError;
use std::fmt;
use serde_json;

#[derive(Debug, Clone)]
pub struct ParsingError{

}

impl StdError for ParsingError {

}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occurred parsing a request payload.")
    }
}
