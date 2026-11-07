use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    SUCCESS,
    FAILED,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::SUCCESS => "success",
            Status::FAILED => "failed",
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "success" => Ok(Self::SUCCESS),
            "failed" => Ok(Self::FAILED),
            _ => Err(format!("Invalid status: {value}")),
        }
    }
}
