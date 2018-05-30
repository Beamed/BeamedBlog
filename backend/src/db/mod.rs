pub mod schema;
pub mod user;

use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;
use r2d2;
use std::env;
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes the database pool, using r2d2 and diesel
pub fn init_pool() -> Pool {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Unable to connect to database. Hard aborting.")
}

///Implemented using rocket docs on implement FromRequest to allow state to be injected into paths
///An either really clever solution, or really cumbersome. I haven't decided!
// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);



// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
