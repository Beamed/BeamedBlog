pub const USERNAME_COOKIE : &'static str = "beamed-username";
pub const DISPLAY_COOKIE : &'static str = "beamed-displayname";
pub const AUTHOR_COOKIE : &'static str = "beamed-author";

#[derive(FromForm)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        display_name -> Nullable<Varchar>,
        author -> Bool,
    }
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm {
    pub username: String,
    pub password: String,
    pub email: String,
    pub display_name: Option<String>,
    pub author: bool,
}
//it's pretty ridiculous you need to have separate models for inserts
#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub display_name: Option<String>,
    pub author: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatedCredentials {
    pub username: String,
    pub display_name: Option<String>,
    pub author: bool,
}

impl<'a> From<&'a User> for ValidatedCredentials {
    fn from(user: &'a User) -> ValidatedCredentials {
        ValidatedCredentials {
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            author: user.author,
        }
    }
}
