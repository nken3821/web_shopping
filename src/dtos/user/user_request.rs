use rocket::serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub phone:Option<String>
}
