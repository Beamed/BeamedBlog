use ::app_state::AppState;
use ::models::user::{User, UserForm};
use std::error::Error;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::fmt::{self, Display, Formatter};
use diesel::types::Text;
use models::login_request::LoginRequest;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::insert_into;
use diesel::associations::HasTable;
use models::user::users::dsl::*;
use super::db::DbConn;

pub enum UserDataError {
    AuthError,
    UserNotFoundError(String),
    UsernameAlreadyExistsError(String),
    DatabaseError(Box<dyn Error>),
}

impl Display for UserDataError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>  {
        match self {
            UserDataError::AuthError => f.write_str("Authentication Error"),
            UserDataError::UserNotFoundError(unf) => f.write_str(format!("User Not Found: {}", unf).as_str()),
            UserDataError::UsernameAlreadyExistsError(uae) => f.write_str(format!("User Already Exists: {}", uae).as_str()),
            UserDataError::DatabaseError(db_error) => f.write_str(format!("DB Error: {}", db_error.description()).as_str()),
        }
    }
}

pub fn get_user_from_request(state: &AppState, login_request: &LoginRequest) -> Result<User, UserDataError> {
    let user = get_user_by_username(get_connection_from_state(state)?,
                                    &login_request.username)?;
    if validate_password(login_request, &user, get_connection_from_state(state)?) {
        Ok(user)
    } else {
        Err(UserDataError::AuthError)
    }
}

pub fn create_new_user(state: &AppState, user: &UserForm) -> Result<(), UserDataError> {
    let current_user = get_user_by_username(get_connection_from_state(state)?,
                                            &user.username);
    match current_user {
        Ok(user) => Err(UserDataError::UsernameAlreadyExistsError(user.username)),
        Err(err) => match err {
            UserDataError::UserNotFoundError(_) => create_user(state, user),
            _ => Err(err)
        }
    }

}

fn create_user(state: &AppState, user: &UserForm) -> Result<(), UserDataError> {
    info!("Creating new user: {}", user.username);
    let encrypted_pass = {
        let pass = &user.password;
        hash(pass.as_str(), DEFAULT_COST).map_err(
            |err| UserDataError::DatabaseError(Box::from(err)))?
    };

    let new_user_to_create = UserForm {
        username: user.username.clone(),
        password: encrypted_pass,
        display_name: user.display_name.clone(),
        email: user.email.clone(),
        author: user.author.clone(),
    };
    let conn = get_connection_from_state(state)?;
    info!("Invoking database insert..");
    insert_into(users)
        .values(&new_user_to_create)
        .execute(&conn)
        .map(|_| () )
        .map_err(|err| UserDataError::DatabaseError(Box::from(err)))
}

fn get_connection_from_state(state: &AppState) -> Result<DbConn, UserDataError> {
    info!("Getting db connection from the app..");
    state.get_connection().map_err(|err| UserDataError::DatabaseError(Box::from(err)))
}

fn validate_password(login_request: &LoginRequest, user: &User, conn: DbConn) -> bool {
    match verify(&login_request.password, &user.password) {
        Ok(verified) => verified,
        Err(_) => false
    }
}

fn get_user_by_username(conn: DbConn, req_username: &String) -> Result<User, UserDataError> {
    let user_row : User = {
        let mut user_rows = match users.filter(username.eq(req_username)).load::<User>(&conn) {
            Ok(user) => user,
            Err(e) => {
                error!("Unable to fetch user rows from DB.");
                return Err(UserDataError::AuthError);
            }
        };
        if user_rows.len() != 1 {
            debug!("user_rows length unexpected value for username: {}, found: {}", req_username , user_rows.len());
            assert!(user_rows.len() < 1);
            return Err(UserDataError::UserNotFoundError(req_username.clone()));
        } else {
            user_rows.remove(0)
        }
    };
    Ok(user_row)
}