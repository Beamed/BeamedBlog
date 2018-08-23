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

