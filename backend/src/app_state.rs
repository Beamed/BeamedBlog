//it sucks we need our state to know about database
//but c'est la vie for now.
//maybe refactor to create a wrapper around the pool at some point
use datalayer::db::{Pool, DbConn};


pub struct AppState {
    db: Pool
}

impl AppState {
    pub fn get_connection(&self) -> Result<DbConn, ::r2d2::Error> {
        self.db.get()
    }
}