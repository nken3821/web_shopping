use crate::schema::activity_logs;
use diesel::{Selectable, associations::Identifiable, deserialize::Queryable, prelude::Insertable};
use serde::Serialize;

use chrono::NaiveDateTime;

#[derive(Insertable)]
#[diesel(table_name = activity_logs)]
pub struct NewActivityLog {
    pub actor_id: Option<i32>,
    pub actor_role: Option<String>,
    pub action: String,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
}

#[derive(Selectable, Queryable, Identifiable, Serialize)]
#[diesel(table_name = activity_logs)]
pub struct ActivityLog {
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
