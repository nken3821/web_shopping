use chrono::NaiveDateTime;

pub struct UserInternal {
    pub id: i32,
    pub email: String,
    pub full_name: String,
    pub user_name: String,
    pub phone: String,
    // pub role: String
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
}
