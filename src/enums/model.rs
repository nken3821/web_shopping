use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Model {
    ADDRESS,
    USER,
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::ADDRESS => "address",
            Model::USER => "user",
        }
    }
}

impl TryFrom<&str> for Model {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "address" => Ok(Model::ADDRESS),
            "user" => Ok(Model::USER),
            _ => Err(format!("Invalid model: {value}")),
        }
    }
}
