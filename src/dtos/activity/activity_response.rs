use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActivityResponse {
    pub id: i32,
    pub actor_id: Option<i32>,
    pub actor_role: Option<String>,
    pub action: String,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
}
