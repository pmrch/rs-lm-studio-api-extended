use crate::prelude::*;

// A message role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl TryFrom<&str> for Role {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value = value.trim().to_lowercase();

        if &value == "system" { Ok(Self::System) }
        else if &value == "user" { Ok(Self::User) }
        else if &value == "assistant" { Ok(Self::Assistant) }
        else { Err("Invalid role was provided".to_string()) }
    }
}
