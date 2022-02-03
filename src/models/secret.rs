use crate::gpg::decrypt_message;
use crate::Passbolt;
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Secret {
    id: String,
    user_id: String,
    pub data: String,
    #[serde(rename = "created")]
    created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    modified_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretData {
    password: String,
    description: String,
}

impl Secret {
    /// Creates a new instance of the Secret struct
    pub fn new(
        id: String,
        user_id: String,
        data: String,
        created_at: DateTime<Local>,
        modified_at: DateTime<Local>,
    ) -> Self {
        Secret {
            id,
            user_id,
            data,
            created_at,
            modified_at,
        }
    }

    pub fn decrypt_data(&self, passbolt: &Passbolt) -> Result<SecretData> {
        Ok(serde_json::from_str(
            decrypt_message(
                passbolt.private_key(),
                passbolt.private_key_pw(),
                self.data.clone(),
            )?
            .as_str(),
        )?)
    }
}
