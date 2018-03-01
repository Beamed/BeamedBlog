#[derive(Queryable)]
pub struct User {
    id: u32,
    username: String,
    password: String,
    email: String,
    display_name: String,
}