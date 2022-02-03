use crate::gpg::ArmoredKey;
use anyhow::Result;
use chrono::{DateTime, Local};
use pgp::SignedPublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt user
pub struct User {
    pub id: String,
    pub username: String,
    pub active: bool,
    pub deleted: bool,
    pub is_mfa_enabled: bool,
    pub last_logged_in: Option<DateTime<Local>>,
    #[serde(rename = "created")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    pub modified_at: DateTime<Local>,
    pub role: Role,
    pub profile: Profile,
    #[serde(rename = "gpgkey")]
    pub gpg_key: GPGKey,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt role
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "created")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    pub modified_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt user's profile
pub struct Profile {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(rename = "created")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    pub modified_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Struct representing a Passbolt user's GPG key information
pub struct GPGKey {
    pub id: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub bits: u16,
    pub fingerprint: String,
    pub expires: Option<DateTime<Local>>,
    pub deleted: bool,
    pub armored_key: String,
    #[serde(rename = "created")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "modified")]
    pub modified_at: DateTime<Local>,
}

impl User {
    /// Returns the public key of the user
    pub fn public_key(&self) -> Result<SignedPublicKey> {
        Ok(SignedPublicKey::from_asc(&self.gpg_key.armored_key)?.0)
    }
}
