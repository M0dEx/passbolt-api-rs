use crate::gpg::decrypt_message;
use crate::Passbolt;
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt secret
pub struct Secret {
    pub id: String,
    pub user_id: String,
    pub data: String,
    #[serde(rename = "created")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    pub modified_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt secret's data
pub struct SecretData {
    pub password: String,
    pub description: String,
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

    /// Decrypts the encrypted data from secret
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
