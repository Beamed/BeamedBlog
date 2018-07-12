pub mod db;
use ::app_state::AppState;
use ::models::user::User;
use std::error::Error;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::fmt::{self, Display, Formatter};
use self::db::DbConn;
use diesel::dsl::Eq;
use diesel::types::Text;
use models::login_request::LoginRequest;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthError;

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

pub fn get_user_from_request(login_request: &LoginRequest, state: &AppState) ->
                            Result<User, Box<dyn Error>> {
    let user = get_user_by_username(&login_request.username, state.get_connection()?)?;
    if validate_password(login_request, &user, state.get_connection()?) {
        Ok(user)
    } else {
        Err(Box::new(AuthError{}))
    }
}

fn validate_password(login_request: &LoginRequest, user: &User, conn: DbConn) -> bool {
    match verify(&login_request.password, &user.password) {
        Ok(verified) => verified,
        Err(_) => false
    }
}

fn get_user_by_username(req_username: &String, conn: DbConn) -> Result<User, AuthError> {
    use diesel::dsl::*;
    use diesel::prelude::*;
    use self::db::schema::users::dsl::*;
    let user_row : User = {
        let mut user_rows = match users.filter(username.eq(req_username)).load::<User>(&*conn) {
            Ok(user) => user,
            Err(e) => {
                error!("Unable to fetch user rows from DB.");
                return Err(AuthError);
            }
        };
        if user_rows.len() != 1 {
            debug!("user_rows length unexpected value for username: {}, found: {}", req_username , user_rows.len());
            return Err(AuthError);
        } else {
            user_rows.remove(0)
        }
    };
    Ok(user_row)
}