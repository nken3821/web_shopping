use chrono::NaiveDateTime;
use diesel::associations::Identifiable;
use diesel::{Selectable, associations::Associations, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::Serialize;
use crate::{schema::addresses};
use crate::models::User;

#[derive(Selectable, Queryable, Associations, Identifiable, Serialize)]
#[diesel(table_name = addresses, belongs_to(User))]
pub struct Address {
    pub id: i32,
    pub user_id: i32,
    pub receiver_name: String,
    pub phone: String,
    pub province: String,
    pub district: Option<String>,
    pub ward: String,
    pub street: String,
    pub is_default: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = addresses)]
pub struct NewAddress {
    pub user_id: i32,
    pub receiver_name: String,
    pub phone: String,
    pub province: String,
    pub district: Option<String>,
    pub ward: String,
    pub street: String,
    pub is_default: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = addresses)]
pub struct UpdateAdress {
    pub receiver_name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub ward: Option<String>,
    pub street: Option<String>,
    pub is_default: Option<bool>, 
}


#[derive(Serialize)]
pub struct UserWithAddresses {
    pub user: User,
    pub addresses: Vec<Address>,
}