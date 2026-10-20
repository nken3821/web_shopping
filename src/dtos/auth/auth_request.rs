use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub fullname: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}
