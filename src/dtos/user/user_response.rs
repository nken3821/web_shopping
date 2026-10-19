use rocket::serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub fullname: String,
    pub created_at: String,
    pub updated_at: String,
}