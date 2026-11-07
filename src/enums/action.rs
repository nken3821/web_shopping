use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Action {
    GETLIST,
    GETONE,
    DELETE,
    UPDATE,
    CREATE,
    DELETELIST,
    LOGIN,
}

impl Action {
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::GETONE => "get_model",
            Action::GETLIST => "get_list_model",
            Action::DELETE => "delete_model",
            Action::DELETELIST => "delete_list_model",
            Action::UPDATE => "update_model",
            Action::CREATE => "create_model",
            Action::LOGIN => "login_user",
        }
    }
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "get_list_model" => Ok(Action::GETLIST),
            "get_model" => Ok(Action::GETONE),
            "delete_model" => Ok(Action::DELETE),
            "update_model" => Ok(Action::UPDATE),
            "create_model" => Ok(Action::CREATE),
            "delete_list_model" => Ok(Action::DELETELIST),
            "login_user" => Ok(Action::LOGIN),
            _ => Err(format!("Invalid action: {value}")),
        }
    }
}
