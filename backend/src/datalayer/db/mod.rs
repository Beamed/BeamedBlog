use diesel::prelude::*;
use diesel::pg::PgConnection;
use ::diesel::r2d2::ConnectionManager;
use std::env;
use std::ops::Deref;

pub type Pool = ::diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DbConn = ::diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Initializes the database pool, using r2d2 and diesel
pub fn init_pool() -> Pool {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    ::diesel::r2d2::Pool::new(manager).expect("Unable to connect to database. Hard aborting.")
}