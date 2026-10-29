use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserRole {
    User,
    Admin,
    Guest,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        }
    }
}

impl TryFrom<&str> for UserRole {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "admin" => Ok(UserRole::Admin),
            "guest" => Ok(UserRole::Guest),
            "user" => Ok(UserRole::User),
            _ => Err(format!("Invalid role: {value}")),
        }
    }
}
