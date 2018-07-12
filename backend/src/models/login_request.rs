#[derive(FromForm)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}