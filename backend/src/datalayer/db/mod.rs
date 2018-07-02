pub mod schema;

use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;
use r2d2;
use std::env;
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Initializes the database pool, using r2d2 and diesel
pub fn init_pool() -> Pool {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Unable to connect to database. Hard aborting.")
}