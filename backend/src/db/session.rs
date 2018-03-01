#[derive(Queryable)]
pub struct Session {
    id: u32,
    user_id: u32,
    csrf_token: String
}