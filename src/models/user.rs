use chrono::NaiveDateTime;
use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use crate::schema::users;

#[derive(Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub user_name: String,
    pub phone: String,
    // pub role: String
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub user_name: String,
    pub phone: String,
    // pub role: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
}