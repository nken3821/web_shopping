use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateAddressRequest {
    pub receiver_name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub street: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAddressRequest {
    pub receiver_name: String,
    pub phone: String,
    pub province: String,
    pub district: Option<String>,
    pub ward: String,
    pub street: String,
}
