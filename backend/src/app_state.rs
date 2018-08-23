//it sucks we need our state to know about database
//but c'est la vie for now.
//maybe refactor to create a wrapper around the pool at some point
use datalayer::db::{Pool, DbConn, init_pool};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub struct AppState {
    db: Pool
}

impl AppState {
    pub fn get_connection(&self) -> Result<DbConn, ::diesel::r2d2::PoolError> {
        self.db.get()
    }
    pub fn new() -> AppState {
        AppState {
            db: init_pool()
        }
    }
}
