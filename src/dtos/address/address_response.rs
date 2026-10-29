use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub id: i32,
    pub receiver_name: String,
    pub phone: String,
    pub province: String,
    pub district: Option<String>,
    pub ward: String,
    pub street: String,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}
