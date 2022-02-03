use crate::gpg::decrypt_message;
use crate::Passbolt;
use anyhow::{Error, Result};
use chrono::{DateTime, Local};
use secstr::SecUtf8;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Clone)]
/// Struct representing a Passbolt secret's data
pub struct SecretData {
    pub password: SecUtf8,
    pub description: SecUtf8,
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
        let json: Value = serde_json::from_str(
            decrypt_message(
                &passbolt.private_key,
                &passbolt.private_key_pw,
                self.data.clone(),
            )?
            .unsecure(),
        )?;

        Ok(SecretData {
            password: SecUtf8::from(json["password"].as_str().ok_or(Error::msg(
                "Could not parse password from decrypted data JSON",
            ))?),
            description: SecUtf8::from(json["description"].as_str().ok_or(Error::msg(
                "Could not parse description from decrypted data JSON",
            ))?),
        })
    }
}
